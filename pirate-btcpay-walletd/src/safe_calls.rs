//! Timeout-guarded wrappers for `pirate_wallet_service` calls that are
//! unsafe to await directly from an HTTP handler or the notify loop while
//! the background sync engine may be concurrently active.
//!
//! Two independent hazards were confirmed live, against a real
//! lightwalletd endpoint, while this shim's own sync loop was running:
//!
//! - `set_lightd_endpoint` is a synchronous function that internally spins
//!   up its own single-threaded tokio runtime (via
//!   `futures::executor::block_on`) to cancel any in-flight sync session on
//!   the wallet before returning. Calling it directly from a task already
//!   running on this binary's main tokio runtime - exactly what the notify
//!   loop does when it fails over mid-sync - hung indefinitely.
//! - `list_address_balances` (used by `get_accounts`) hung indefinitely when
//!   called while the sync engine was concurrently reading/writing the same
//!   wallet sqlite database, almost certainly SQLite-level lock contention
//!   rather than a Rust-level deadlock.
//!
//! Both categories get the same treatment: run the call on a dedicated
//! `spawn_blocking` thread (so a hang there can't starve the async worker
//! pool the HTTP server and notify loop share) with an enforced timeout, so
//! a hang degrades to a clear timeout error instead of wedging the caller
//! forever.

use anyhow::{anyhow, Context, Result};
use pirate_wallet_service::WalletId;
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);

/// Run a blocking `pirate_wallet_service` call on a dedicated thread with a
/// timeout, instead of executing it directly on an async worker thread.
pub async fn guarded<F, T>(what: &str, timeout: Duration, f: F) -> Result<T>
where
    F: FnOnce() -> anyhow::Result<T> + Send + 'static,
    T: Send + 'static,
{
    let handle = tokio::task::spawn_blocking(f);
    match tokio::time::timeout(timeout, handle).await {
        Ok(join_result) => join_result.with_context(|| format!("{what} task panicked"))?,
        Err(_) => Err(anyhow!(
            "{what} did not complete within {timeout:?}; likely lock contention with the active sync engine"
        )),
    }
}

pub async fn set_lightd_endpoint(
    wallet_id: WalletId,
    url: String,
    tls_pin: Option<String>,
) -> Result<()> {
    guarded("set_lightd_endpoint", DEFAULT_TIMEOUT, move || {
        pirate_wallet_service::set_lightd_endpoint(wallet_id, url, tls_pin)
    })
    .await
}

pub async fn list_address_balances(
    wallet_id: WalletId,
    key_id: Option<i64>,
) -> Result<Vec<pirate_wallet_service::AddressBalanceInfo>> {
    guarded("list_address_balances", DEFAULT_TIMEOUT, move || {
        pirate_wallet_service::list_address_balances(wallet_id, key_id)
    })
    .await
}

pub async fn generate_address_for_key(
    wallet_id: WalletId,
    key_id: i64,
    use_ironwood: bool,
) -> Result<String> {
    guarded("generate_address_for_key", DEFAULT_TIMEOUT, move || {
        pirate_wallet_service::generate_address_for_key(wallet_id, key_id, use_ironwood)
    })
    .await
}

pub async fn list_incoming_deposits(
    wallet_id: WalletId,
    limit: Option<u32>,
) -> Result<Vec<pirate_wallet_service::AddressedDeposit>> {
    guarded("list_incoming_deposits", DEFAULT_TIMEOUT, move || {
        pirate_wallet_service::list_incoming_deposits(wallet_id, limit)
    })
    .await
}
