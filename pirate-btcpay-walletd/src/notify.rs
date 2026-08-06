//! Background sync + deposit-notification loop.
//!
//! `pirate-wallet-service` has no push hook for "a new transaction was
//! decrypted" the way the legacy `walletd`'s block-scan loop did. This
//! reimplements that behavior one layer up: periodically sync, list incoming
//! deposits, and fire the same BTCPay webhook walletd used for every deposit
//! not already recorded in `notified_deposits`.
//!
//! It also drives lightwalletd failover (see `failover.rs`): each tick's
//! sync result feeds `FailoverState`, and whenever it isn't already on the
//! primary endpoint it re-probes higher-priority ones so it switches back
//! once one of them recovers.

use crate::AppState;
use pirate_wallet_service::SyncMode;
use std::sync::Arc;
use std::time::Duration;

pub fn spawn(state: Arc<AppState>) {
    tokio::spawn(async move {
        loop {
            if let Err(e) = tick(&state).await {
                tracing::warn!("sync/notify tick failed: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(state.config.poll_interval as u64)).await;
        }
    });
}

async fn tick(state: &Arc<AppState>) -> anyhow::Result<()> {
    let sync_result = pirate_wallet_service::start_sync(state.wallet_id.clone(), SyncMode::Compact).await;

    let switch = state
        .failover
        .lock()
        .await
        .record_result(sync_result.is_ok());
    if let Some(next_url) = switch {
        tracing::warn!(
            "lightwalletd endpoint failed {} times in a row, failing over to {}",
            crate::failover::FAILURE_THRESHOLD,
            next_url
        );
        if let Err(e) =
            pirate_wallet_service::set_lightd_endpoint(state.wallet_id.clone(), next_url, None)
        {
            tracing::warn!("Failed to switch lightd endpoint: {}", e);
        }
    }

    try_switch_back_to_higher_priority_endpoint(state).await;

    sync_result?;

    let deposits = pirate_wallet_service::list_incoming_deposits(state.wallet_id.clone(), None)?;
    for deposit in deposits {
        if !deposit.confirmed {
            continue;
        }
        let should_notify = state
            .bookkeeping
            .lock()
            .await
            .mark_notified(&deposit.txid, &deposit.address)?;
        if should_notify {
            notify_transaction(state, &deposit.txid).await;
        }
    }
    Ok(())
}

/// If currently failed over to a lower-priority endpoint, probe
/// higher-priority ones in order and switch back to the first one that
/// answers, so the wallet doesn't stay on a backup forever after the
/// primary recovers.
async fn try_switch_back_to_higher_priority_endpoint(state: &Arc<AppState>) {
    let candidates: Vec<(usize, String)> = {
        let failover = state.failover.lock().await;
        failover
            .higher_priority_endpoints()
            .map(|(i, url)| (i, url.to_string()))
            .collect()
    };

    for (index, url) in candidates {
        let result = pirate_wallet_service::test_node(url.clone(), None).await;
        let healthy = matches!(result, Ok(r) if r.success);
        if !healthy {
            continue;
        }

        state.failover.lock().await.switch_to(index);
        tracing::info!("higher-priority lightwalletd endpoint {} recovered, switching back", url);
        if let Err(e) =
            pirate_wallet_service::set_lightd_endpoint(state.wallet_id.clone(), url, None)
        {
            tracing::warn!("Failed to switch back to recovered lightd endpoint: {}", e);
        }
        return;
    }
}

async fn notify_transaction(state: &Arc<AppState>, txid: &str) {
    let uri = format!(
        "{}/piratelikedaemoncallback/tx?cryptoCode=arrr&hash={}",
        state.config.notify_host, txid
    );
    tracing::info!("notify {}", uri);
    if let Err(e) = state.http.get(&uri).send().await {
        tracing::warn!("Failed to notify new tx: {}", e);
    }
}
