use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

/// Local bookkeeping the shim needs on top of `pirate-wallet-service`.
///
/// `pirate-wallet-service` derives one sequential diversifier stream per
/// wallet; it has no notion of BTCPay's per-store `AccountIndex` address-pool
/// buckets. This mirrors exactly what the legacy `walletd`'s own `db.rs` did:
/// a small local table mapping (account_index, sub_account) -> address, so
/// multiple BTCPay stores can share one shim instance without colliding.
///
/// It also tracks which deposits have already fired the BTCPay webhook,
/// since `list_incoming_deposits` returns full history on every poll rather
/// than an incremental "since last call" window.
pub struct Bookkeeping {
    conn: Connection,
}

impl Bookkeeping {
    pub fn open(dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(dir)?;
        let conn = Connection::open(dir.join("pirate-btcpay-walletd.db"))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS wallet_state (
                id INTEGER PRIMARY KEY CHECK (id = 0),
                wallet_id TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS address_allocations (
                account_index INTEGER NOT NULL,
                sub_account INTEGER NOT NULL,
                address TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                PRIMARY KEY (account_index, sub_account)
            );
            CREATE TABLE IF NOT EXISTS notified_deposits (
                txid TEXT NOT NULL,
                address TEXT NOT NULL,
                PRIMARY KEY (txid, address)
            );",
        )?;
        Ok(Bookkeeping { conn })
    }

    pub fn get_wallet_id(&self) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row("SELECT wallet_id FROM wallet_state WHERE id = 0", [], |r| {
                r.get(0)
            })
            .optional()?)
    }

    pub fn set_wallet_id(&self, wallet_id: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO wallet_state (id, wallet_id) VALUES (0, ?1)
             ON CONFLICT (id) DO UPDATE SET wallet_id = excluded.wallet_id",
            params![wallet_id],
        )?;
        Ok(())
    }

    /// Allocate a new account bucket (sub_account 0) for `address`.
    /// Mirrors walletd's own `MAX(account) + 1` allocation scheme.
    pub fn create_account(&self, address: &str) -> Result<u32> {
        let next_account: Option<u32> = self
            .conn
            .query_row("SELECT MAX(account_index) FROM address_allocations", [], |r| {
                r.get(0)
            })?;
        let account_index = next_account.map(|v| v + 1).unwrap_or(0);
        self.conn.execute(
            "INSERT INTO address_allocations (account_index, sub_account, address, created_at)
             VALUES (?1, 0, ?2, strftime('%s','now'))",
            params![account_index, address],
        )?;
        Ok(account_index)
    }

    /// Allocate the next address within an existing account bucket.
    /// Errors if `create_account` was never called for `account_index`,
    /// matching walletd's own behavior (its MAX(sub_account) query errors on
    /// an unknown account rather than silently starting one).
    pub fn create_address(&self, account_index: u32, address: &str) -> Result<u32> {
        let max_sub: Option<u32> = self
            .conn
            .query_row(
                "SELECT MAX(sub_account) FROM address_allocations WHERE account_index = ?1",
                params![account_index],
                |r| r.get(0),
            )
            .optional()?
            .flatten();
        let sub_account = max_sub
            .ok_or_else(|| anyhow!("Unknown account_index {account_index}; call create_account first"))?
            + 1;
        self.conn.execute(
            "INSERT INTO address_allocations (account_index, sub_account, address, created_at)
             VALUES (?1, ?2, ?3, strftime('%s','now'))",
            params![account_index, sub_account, address],
        )?;
        Ok(sub_account)
    }

    pub fn base_address(&self, account_index: u32) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row(
                "SELECT address FROM address_allocations WHERE account_index = ?1 AND sub_account = 0",
                params![account_index],
                |r| r.get(0),
            )
            .optional()?)
    }

    pub fn list_account_indices(&self) -> Result<Vec<u32>> {
        let mut stmt = self
            .conn
            .prepare("SELECT DISTINCT account_index FROM address_allocations ORDER BY account_index")?;
        let rows = stmt.query_map([], |r| r.get(0))?;
        Ok(rows.collect::<rusqlite::Result<Vec<u32>>>()?)
    }

    pub fn addresses_for_account(&self, account_index: u32) -> Result<Vec<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT address FROM address_allocations WHERE account_index = ?1")?;
        let rows = stmt.query_map(params![account_index], |r| r.get(0))?;
        Ok(rows.collect::<rusqlite::Result<Vec<String>>>()?)
    }

    /// (account_index, sub_account) for a given address, if it was allocated here.
    pub fn lookup_address(&self, address: &str) -> Result<Option<(u32, u32)>> {
        Ok(self
            .conn
            .query_row(
                "SELECT account_index, sub_account FROM address_allocations WHERE address = ?1",
                params![address],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?)
    }

    /// Records that `(txid, address)` was notified. Returns true if this was
    /// the first time (i.e. the webhook should actually fire).
    pub fn mark_notified(&self, txid: &str, address: &str) -> Result<bool> {
        let changed = self.conn.execute(
            "INSERT INTO notified_deposits (txid, address) VALUES (?1, ?2)
             ON CONFLICT (txid, address) DO NOTHING",
            params![txid, address],
        )?;
        Ok(changed > 0)
    }
}
