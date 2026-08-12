use anyhow::Result;
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

const SCHEMA_SQL: &str = "
    CREATE TABLE IF NOT EXISTS wallet_state (
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
    );
    CREATE TABLE IF NOT EXISTS account_pools (
        account_index INTEGER PRIMARY KEY,
        use_ironwood INTEGER NOT NULL
    );
";

impl Bookkeeping {
    pub fn open(dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(dir)?;
        let conn = Connection::open(dir.join("pirate-btcpay-walletd.db"))?;
        conn.execute_batch(SCHEMA_SQL)?;
        Ok(Bookkeeping { conn })
    }

    #[cfg(test)]
    fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA_SQL)?;
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

    /// Allocate a new account bucket (sub_account 0) for `address`, fixing
    /// the shielded pool (Sapling vs Ironwood) that every address in this
    /// account will be generated from from now on.
    /// Mirrors walletd's own `MAX(account) + 1` allocation scheme.
    pub fn create_account(&self, address: &str, use_ironwood: bool) -> Result<u32> {
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
        self.set_account_pool(account_index, use_ironwood)?;
        Ok(account_index)
    }

    /// The pool (`true` = Ironwood, `false` = Sapling) an account was fixed
    /// to at creation time, if it has one on record yet.
    pub fn account_pool(&self, account_index: u32) -> Result<Option<bool>> {
        Ok(self
            .conn
            .query_row(
                "SELECT use_ironwood FROM account_pools WHERE account_index = ?1",
                params![account_index],
                |r| r.get::<_, i64>(0),
            )
            .optional()?
            .map(|v| v != 0))
    }

    /// Fixes an account's pool the first time it's recorded; a no-op on
    /// every call after that, so an account's pool never silently changes.
    fn set_account_pool(&self, account_index: u32, use_ironwood: bool) -> Result<()> {
        self.conn.execute(
            "INSERT INTO account_pools (account_index, use_ironwood) VALUES (?1, ?2)
             ON CONFLICT (account_index) DO NOTHING",
            params![account_index, use_ironwood as i64],
        )?;
        Ok(())
    }

    /// Allocate the next address within an account bucket, creating the
    /// bucket if `account_index` hasn't been seen before.
    ///
    /// BTCPay's own flow is supposed to call `create_account` once (from
    /// the store's Pirate settings page) before ever reserving an address
    /// for a given `account_index` - but a store's saved config can end up
    /// with an `account_index` that was never actually registered here
    /// (e.g. that original `create_account` call 500'd server-side, and
    /// BTCPay kept its client-side default anyway; confirmed happening
    /// live). Treat an unseen `account_index` as starting fresh at
    /// sub_account 0 instead of erroring, so a store in that state still
    /// gets a working deposit address instead of failing every invoice.
    ///
    /// `use_ironwood` should be whatever pool the caller resolved via
    /// `account_pool` (falling back to the wallet's default when this
    /// account has no pool on record yet, i.e. it's being self-healed here
    /// for the first time) - `set_account_pool` is a no-op for an account
    /// that already has one, so it only actually fixes the pool the first
    /// time an account_index is seen.
    pub fn create_address(&self, account_index: u32, address: &str, use_ironwood: bool) -> Result<u32> {
        let max_sub: Option<u32> = self
            .conn
            .query_row(
                "SELECT MAX(sub_account) FROM address_allocations WHERE account_index = ?1",
                params![account_index],
                |r| r.get(0),
            )
            .optional()?
            .flatten();
        let sub_account = max_sub.map(|v| v + 1).unwrap_or(0);
        self.conn.execute(
            "INSERT INTO address_allocations (account_index, sub_account, address, created_at)
             VALUES (?1, ?2, ?3, strftime('%s','now'))",
            params![account_index, sub_account, address],
        )?;
        self.set_account_pool(account_index, use_ironwood)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_account_fixes_the_requested_pool() {
        let bk = Bookkeeping::open_in_memory().unwrap();
        let account_index = bk.create_account("zs1sapling...", false).unwrap();
        assert_eq!(bk.account_pool(account_index).unwrap(), Some(false));
    }

    #[test]
    fn create_address_inherits_the_account_pool_regardless_of_the_flag_passed() {
        let bk = Bookkeeping::open_in_memory().unwrap();
        let account_index = bk.create_account("zs1ironwood...", true).unwrap();
        // A later create_address call always resolves use_ironwood from
        // account_pool() before calling in; passing false here simulates a
        // caller bug and proves set_account_pool's ON CONFLICT DO NOTHING
        // keeps the account's real pool (true) rather than overwriting it.
        bk.create_address(account_index, "zs1ironwood-addr2...", false)
            .unwrap();
        assert_eq!(bk.account_pool(account_index).unwrap(), Some(true));
    }

    #[test]
    fn create_address_self_heals_an_unknown_account_into_the_given_pool() {
        let bk = Bookkeeping::open_in_memory().unwrap();
        assert_eq!(bk.account_pool(7).unwrap(), None);
        bk.create_address(7, "zs1selfhealed...", true).unwrap();
        assert_eq!(bk.account_pool(7).unwrap(), Some(true));
    }

    #[test]
    fn account_pool_is_none_for_an_account_never_seen() {
        let bk = Bookkeeping::open_in_memory().unwrap();
        assert_eq!(bk.account_pool(42).unwrap(), None);
    }
}
