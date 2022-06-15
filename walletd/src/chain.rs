use std::borrow::Borrow;
use anyhow::anyhow;
use lazycell::AtomicLazyCell;
use rocket::futures::StreamExt;
use tonic::Request;
use tonic::transport::Channel;
use crate::{Error, get_appstore, Result};
use crate::db::Db;
use crate::lw_rpc::{BlockId, BlockRange, ChainSpec};
use crate::lw_rpc::compact_tx_streamer_client::CompactTxStreamerClient;

pub async fn scan_blocks() -> Result<()> {
    let app = get_appstore();
    let store = app.store.lock().await;
    let starting_height = app.config.starting_height as u64;
    let mut client = app.lwd_client.lock().await;
    let mut rewind_count = 10;

    for _attempt in 0..3 {
        let block_id = store.get_last_synced()?;
        match scan_blocks_inner(&mut client, block_id.as_ref(), starting_height).await {
            Ok(_) => break,
            Err(Error::ReorgDetected) => {
                log::info!("Block Reorganization detected");
            } // fall out & retry
            e => return e
        }
        if let Some(ref block_id) = block_id {
            store.rewind_blocks(block_id.height.saturating_sub(rewind_count))?;
        }
        rewind_count *= 2; // exponential backoff
    }
    Err(anyhow!("Block scanning failed").into())
}

async fn scan_blocks_inner(client: &mut CompactTxStreamerClient<Channel>, start_block: Option<&BlockId>, starting_height: u64) -> Result<()> {
    let latest_height = client.get_latest_block(Request::new(ChainSpec {})).await?.into_inner().height;
    let mut block_stream = client.get_block_range(Request::new(BlockRange {
        start: Some(BlockId {
            height: start_block.map(|b| b.height).unwrap_or(starting_height),
            hash: vec![],
        }),
        end: Some(BlockId { height: latest_height, hash: vec![] }),
    })).await?.into_inner();
    while let Some(block) = block_stream.message().await? {
        if let Some(ref block_id) = start_block {
            if block.height == block_id.height + 1 && block.prev_hash != block_id.hash {
                log::info!("{} {}", block.height, hex::encode(&block.prev_hash));
                return Err(Error::ReorgDetected);
            }

        }

        log::info!("{}", block.height);

        // TODO: Note scan
    }

    // TODO: Save checkpoint
    Ok(())
}
