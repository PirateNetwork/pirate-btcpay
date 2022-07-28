use std::borrow::Borrow;
use anyhow::anyhow;
use lazycell::AtomicLazyCell;
use rocket::futures::StreamExt;
use tonic::Request;
use tonic::transport::Channel;
use zcash_client_backend::encoding::encode_payment_address;
use zcash_primitives::consensus::{BlockHeight, Parameters};
use zcash_primitives::merkle_tree::CommitmentTree;
use zcash_primitives::sapling::Node;
use zcash_primitives::sapling::note_encryption::try_sapling_compact_note_decryption;
use zcash_primitives::transaction::components::sapling::CompactOutputDescription;
use zcash_primitives::zip32::ExtendedFullViewingKey;
use crate::{Error, get_appstore, NETWORK, Result};
use crate::db::Db;
use crate::lw_rpc::{BlockId, BlockRange, ChainSpec, Empty};
use crate::lw_rpc::compact_tx_streamer_client::CompactTxStreamerClient;
use ff::PrimeField;
use group::GroupEncoding;
use lazy_static::lazy_static;
use tokio::sync::{Semaphore, MutexGuard};
use crate::rpc::notify_transaction;

lazy_static! {
    static ref SYNC_LOCK: Semaphore = Semaphore::new(1);
}

pub async fn scan_blocks(notify_url: &str) -> Result<()> {
    let _permit = SYNC_LOCK.acquire().await.map_err(|_| anyhow!("SyncLock Error"))?;
    let app = get_appstore();
    let starting_height = app.config.starting_height as u64;
    let mut client = app.lwd_client.lock().await;
    let mut rewind_count = 10;

    for _attempt in 0..3 {
        let block_id = {
            let store = app.store.lock().unwrap();
            store.get_last_synced()?
        };
        match scan_blocks_inner(&mut client, block_id.as_ref(), starting_height, &app.fvk, notify_url).await {
            Ok(_) => {
                log::info!("Scan Completed");
                return Ok(())
            }
            Err(Error::ReorgDetected) => {
                log::info!("Block Reorganization detected");
            } // fall out & retry
            e => return e
        }
        if let Some(ref block_id) = block_id {
            let store = app.store.lock().unwrap();
            store.rewind_blocks(block_id.height.saturating_sub(rewind_count))?;
        }
        rewind_count *= 2; // exponential backoff
    }
    Err(anyhow!("Block scanning failed").into())
}

async fn scan_blocks_inner(client: &mut CompactTxStreamerClient<Channel>, start_block: Option<&BlockId>, starting_height: u64, fvk: &ExtendedFullViewingKey,
                           notify_url: &str) -> Result<()> {
    let app = get_appstore();
    let latest_height = client.get_latest_block(Request::new(ChainSpec {})).await?.into_inner().height;
    let height = start_block.map(|b| b.height + 1).unwrap_or(starting_height);

    let mut block_stream = client.get_block_range(Request::new(BlockRange {
        start: Some(BlockId {
            height,
            hash: vec![],
        }),
        end: Some(BlockId { height: latest_height, hash: vec![] }),
    })).await?.into_inner();

    let mut last_block = None;
    while let Some(block) = block_stream.message().await? {
        if let Some(ref block_id) = start_block {
            if block.height == block_id.height + 1 && block.prev_hash != block_id.hash {
                log::info!("{} {}", block.height, hex::encode(&block.prev_hash));
                return Err(Error::ReorgDetected);
            }

        }

        log::info!("{}", block.height);

        // Note scan
        let vk = &fvk.fvk.vk;
        let ivk = vk.ivk();
        let height = BlockHeight::from_u32(block.height as u32);
        for transaction in block.vtx.iter() {
            let mut value = 0;
            let mut address = None;
            for cout in transaction.outputs.iter() {
                let co = CompactOutputDescription {
                    cmu: bls12_381::Scalar::from_repr(cout.cmu.clone().try_into().unwrap()).unwrap(),
                    epk: jubjub::ExtendedPoint::from_bytes(&cout.epk.clone().try_into().unwrap()).unwrap(),
                    enc_ciphertext: cout.ciphertext.to_vec(),
                };
                if let Some((note, pa)) = try_sapling_compact_note_decryption(&NETWORK, height, &ivk, &co) {
                    address = Some(encode_payment_address(NETWORK.hrp_sapling_payment_address(), &pa));
                    value += note.value;
                    // Db::put_received_note(app.store.clone(), &transaction.hash, &address, block.height as u32, note.value)?;
                }
            }
            if let Some(address) = address {
                Db::put_transaction(app.store.clone(), &transaction.hash, &address, block.height as u32, value)?;
                let mut tx_id = transaction.hash.to_vec();
                tx_id.reverse();
                let res = notify_transaction(&hex::encode(&tx_id), notify_url).await;
                if let Err(e) = res {
                    log::warn!("Failed to notify new tx: {}", e.to_string());
                }
            }
        }
        last_block = Some(BlockInfo {
            height: block.height as u32,
            hash: block.hash.try_into().unwrap(),
        });
    }

    if let Some(block) = last_block {
        Db::put_block_height(app.store.clone(), &block.hash, block.height as u32)?;
    }
    Ok(())
}

pub async fn get_height() -> Result<u32> {
    let app = get_appstore();
    let mut client = app.lwd_client.lock().await;
    let rep = client.get_lightd_info(Request::new(Empty {})).await?.into_inner();
    Ok(rep.block_height as u32)
}

struct BlockInfo {
    height: u32,
    hash: [u8; 32],
}
