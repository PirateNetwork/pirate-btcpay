use std::path::Path;
use std::sync::Arc;
use rusqlite::{Connection, NO_PARAMS, OptionalExtension, params};
use tokio::sync::Mutex;
use zcash_client_backend::encoding::encode_payment_address;
use zcash_primitives::consensus::Parameters;
use zcash_primitives::sapling::keys::FullViewingKey;
use zcash_primitives::zip32::{DiversifierIndex, ExtendedFullViewingKey};
use anyhow::{anyhow, Context};
use crate::lw_rpc::BlockId;
use crate::{NETWORK, Result};
use crate::rpc::data::{CreateAccountResponse, CreateAddressResponse};

pub struct Db {
    connection: Connection,
}

impl Db {
    pub fn open(path: &Path) -> Result<Self> {
        let connection = Connection::open(path)?;
        Self::create_schema(&connection)?;
        Ok(Db {
            connection
        })
    }

    fn create_schema(connection: &Connection) -> Result<()> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS blocks (
            height INTEGER PRIMARY KEY,
            hash BLOB NOT NULL)",
            NO_PARAMS,
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS addresses (
            id_address INTEGER PRIMARY KEY,
            label TEXT NOT NULL,
            account INTEGER NOT NULL,
            sub_account INTEGER NOT NULL,
            address TEXT NOT NULL,
            diversifier_index INTEGER NOT NULL)",
            NO_PARAMS,
        )?;
        Ok(())
    }

    pub fn get_last_synced(&self) -> Result<Option<BlockId>> {
        let block_id = self.connection.query_row("SELECT hash, height FROM blocks WHERE height = (SELECT MAX(height) FROM blocks)",
        NO_PARAMS, |row| {
                let hash: Vec<u8> = row.get(0)?;
                let height: u32 = row.get(1)?;
                Ok(BlockId {
                    hash,
                    height: height as u64,
                })
            }).optional()?;
        Ok(block_id)
    }

    pub fn rewind_blocks(&self, height: u64) -> Result<()> {
        self.connection.execute("DELETE FROM blocks WHERE height > ?1", [height as u32])?;
        Ok(())
    }

    pub async fn create_account(db: Arc<Mutex<Self>>, label: Option<String>, fvk: &ExtendedFullViewingKey) -> Result<CreateAccountResponse> {
        let label = label.unwrap_or("".to_string());
        let db = db.lock().await;
        let id_account: Option<u32> =
            db.connection.query_row("SELECT MAX(account) FROM addresses", NO_PARAMS, |row| {
                let id: Option<u32> = row.get(0)?;
                Ok(id)
            })?;
        let id_account = id_account.map(|id| id + 1).unwrap_or(0);
        let (diversifier_index, address) = Self::next_diversifier(&db.connection, fvk)?;

        db.connection.execute("INSERT INTO addresses(label, account, sub_account, address, diversifier_index) VALUES (?1,?2,?3,?4,?5)",
                           params![label, id_account, 0, &address, diversifier_index as i64])?;
        let account = CreateAccountResponse {
            account_index: id_account,
            address,
        };
        Ok(account)
    }

    pub async fn create_address(db: Arc<Mutex<Self>>, label: Option<String>, account_index: u32, fvk: &ExtendedFullViewingKey) -> Result<CreateAddressResponse> {
        let label = label.unwrap_or("".to_string());
        let db = db.lock().await;
        let id_sub_account: u32 = db.connection.query_row(
            "SELECT MAX(sub_account) FROM addresses WHERE account = ?1",
            params![account_index],
            |row| row.get(0),
        )?;
        let id_sub_account = id_sub_account + 1;
        let (diversifier_index, address) = Self::next_diversifier(&db.connection, fvk)?;
        db.connection.execute("INSERT INTO addresses(label, account, sub_account, address, diversifier_index) VALUES (?1,?2,?3,?4,?5)",
                           params![label, account_index, id_sub_account, &address, diversifier_index as i64])?;

        let sub_account = CreateAddressResponse {
            address,
            address_index: id_sub_account
        };
        Ok(sub_account)
    }
    // Helpers

    fn next_diversifier(connection: &Connection, fvk: &ExtendedFullViewingKey) -> anyhow::Result<(u64, String)> {
        let diversifier: Option<i64> =
            connection.query_row("SELECT MAX(diversifier_index) FROM addresses", NO_PARAMS, |row| {
                let index: Option<i64> = row.get(0)?;
                Ok(index)
            })?;
        let (next_index, pa) = if let Some(diversifier) = diversifier {
            let mut di = [0u8; 11];
            di[0..8].copy_from_slice(&diversifier.to_le_bytes());
            let mut index = DiversifierIndex(di);
            index
                .increment()
                .map_err(|_| anyhow!("Out of diversified addresses"))?;
            let (index, pa) = fvk
                .address(index)
                .map_err(|_| anyhow!("Could not derive new subaccount"))?;
            (index, pa)
        } else {
                fvk
                .default_address()
                .map_err(|_| anyhow!("Cannot get default address"))?
        };
        let mut di = [0u8; 8];
        di.copy_from_slice(&next_index.0[0..8]);
        let next_index = u64::from_le_bytes(di);
        Ok((
            next_index,
            encode_payment_address(NETWORK.hrp_sapling_payment_address(), &pa),
        ))
    }
}
