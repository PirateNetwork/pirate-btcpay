use std::path::Path;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, NO_PARAMS, OptionalExtension, params, Row};
use zcash_client_backend::encoding::encode_payment_address;
use zcash_primitives::consensus::Parameters;
use zcash_primitives::sapling::keys::FullViewingKey;
use zcash_primitives::zip32::{DiversifierIndex, ExtendedFullViewingKey};
use anyhow::{anyhow, Context};
use crate::lw_rpc::BlockId;
use crate::{NETWORK, Result};
use crate::data::{AccountBalance, SubAddress, Transfer};
use crate::rpc::data::{CreateAccountResponse, CreateAddressResponse, GetTransactionByIdResponse};

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
        connection.execute(
            "CREATE TABLE IF NOT EXISTS transactions (
            id_tx INTEGER PRIMARY KEY,
            txid BLOB NOT NULL UNIQUE,
            height INTEGER NOT NULL,
            address TEXT NOT NULL,
            value INTEGER NOT NULL)",
            NO_PARAMS,
        )?;
        Ok(())
    }

    pub fn is_new(&self) -> Result<bool> {
        let r = self.connection
            .query_row("SELECT 1 FROM addresses", NO_PARAMS, |r| r.get::<_, u32>(0)).optional()?;

        Ok(r.is_none())
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
        let db = db.lock().unwrap();
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
        let db = db.lock().unwrap();
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

    pub fn get_accounts(db: Arc<Mutex<Self>>, height: u32, confirmations: u32) -> anyhow::Result<Vec<AccountBalance>> {
        let db = db.lock().unwrap();
        let confirmed_height = height - confirmations + 1;
        let mut s = db.connection.prepare(
            "WITH base AS (SELECT account, address FROM addresses WHERE sub_account = 0), \
            balances AS (SELECT account, SUM(value) AS total from transactions t JOIN addresses a ON t.address = a.address GROUP BY a.account), \
            unlocked_balances AS (SELECT account, SUM(value) AS unlocked from transactions t JOIN addresses a ON t.address = a.address WHERE height <= ?1 GROUP BY a.account) \
            SELECT a.account, a.label, b.total, COALESCE(u.unlocked, 0) AS unlocked, base.address as base_address \
            FROM addresses a JOIN balances b ON a.account = b.account LEFT JOIN unlocked_balances u ON u.account = a.account JOIN base ON base.account = a.account GROUP BY a.account")?;

        let rows = s.query_map(params![confirmed_height], |row| {
            let id_account: u32 = row.get(0)?;
            let label: String = row.get(1)?;
            let balance: i64 = row.get(2)?;
            let unlocked: i64 = row.get(3)?;
            let base_address: String = row.get(4)?;
            Ok(AccountBalance {
                account_index: id_account,
                label,
                balance: balance as u64,
                unlocked_balance: unlocked as u64,
                base_address,
                tag: "".to_string(),
            })
        })?;

        let mut sub_accounts: Vec<AccountBalance> = vec![];
        for row in rows {
            let sa = row?;
            sub_accounts.push(sa);
        }

        Ok(sub_accounts)
    }

    pub fn put_transaction(db: Arc<Mutex<Self>>, tx_hash: &[u8], address: &str, height: u32, value: u64) -> Result<()> {
        let db = db.lock().unwrap();
        db.connection.execute("INSERT INTO transactions(txid, height, address, value) VALUES (?1, ?2, ?3, ?4) ON CONFLICT (txid) DO NOTHING", params![tx_hash, height, address, value as i64])?;
        Ok(())
    }

    pub fn put_block_height(db: Arc<Mutex<Self>>, tx_hash: &[u8], height: u32) -> Result<()> {
        let db = db.lock().unwrap();
        db.connection.execute("INSERT INTO blocks(hash, height) VALUES (?1, ?2)", params![tx_hash, height])?;
        Ok(())
    }

    pub fn get_transaction(db: Arc<Mutex<Self>>, account_index: u32, txid: &[u8], latest_height: u32, confirmations: u32) -> Result<GetTransactionByIdResponse> {
        let db = db.lock().unwrap();
        let transfer = db.connection.query_row("SELECT t.address, value, sub_account, txid, height \
        FROM transactions t JOIN addresses a ON t.address = a.address WHERE \
        txid = ?1 AND a.account = ?2", params![txid, account_index], |row| Self::row_to_transfer(row, latest_height, account_index, confirmations)).optional()?;
        let transfer = transfer.ok_or(anyhow!("No such transaction"))?;
        let rep = GetTransactionByIdResponse {
            transfer: transfer.clone(),
            transfers: vec![
                transfer
            ]
        };
        Ok(rep)
    }

    pub fn get_transfers(db: Arc<Mutex<Self>>, latest_height: u32, account_index: u32, subaddr_indices: &[u32], confirmations: u32) -> Result<Vec<Transfer>> {
        let db = db.lock().unwrap();
        let mut s = db.connection.prepare("SELECT a.address, value, sub_account, txid, height \
            FROM transactions t JOIN addresses a ON t.address = a.address WHERE \
            account = ?1")?;
        let rows = s.query_map(params![account_index], |row| Self::row_to_transfer(row, latest_height, account_index, confirmations))?;
        let mut transfers: Vec<Transfer> = vec![];
        for row in rows {
            let row = row?;
            if subaddr_indices.contains(&row.subaddr_index.minor) {
                transfers.push(row);
            }
        }
        Ok(transfers)
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

    fn row_to_transfer(row: &Row, latest_height: u32, account_index: u32, confirmations: u32) -> rusqlite::Result<Transfer> {
        let address: String = row.get(0)?;
        let value: i64 = row.get(1)?;
        let sub_account: u32 = row.get(2)?;
        let mut txid: Vec<u8> = row.get(3)?;
        txid.reverse();
        let height: u32 = row.get(4)?;
        let t = Transfer {
            address,
            amount: value as u64,
            confirmations: latest_height - height + 1,
            height,
            fee: 0,
            note: String::new(),
            payment_id: String::new(),
            subaddr_index: SubAddress {
                major: account_index,
                minor: sub_account
            },
            suggested_confirmations_threshold: confirmations,
            timestamp: 0,
            txid: hex::encode(txid),
            r#type: "in".to_string(),
            unlock_time: 0
        };
        Ok(t)
    }
}
