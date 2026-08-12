//! HTTP routes matching the legacy `walletd`'s JSON contract exactly, so
//! `PirateRPCProvider`/`PirateAddressPoolService`/`PirateDaemonCallbackController`
//! on the BTCPay side need no changes at all.

use crate::AppState;
use anyhow::anyhow;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use pirate_wallet_service::SyncMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create_account", post(create_account))
        .route("/create_address", post(create_address))
        .route("/get_accounts", post(get_accounts))
        .route("/get_transfer_by_txid", post(get_transfer_by_txid))
        .route("/get_transfers", post(get_transfers))
        .route("/get_height", post(get_height))
        .route("/sync_info", post(sync_info))
        .route("/request_scan", post(request_scan))
        .route("/get_fee_estimate", post(get_fee_estimate))
        .with_state(state)
}

struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            self.0.to_string(),
        )
            .into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        ApiError(e)
    }
}

type ApiResult<T> = Result<Json<T>, ApiError>;

/// Derive a fresh receive address for the wallet's single (watch-only) key
/// group, from whichever shielded pool the caller has already resolved.
///
/// Deliberately does not use `pirate_wallet_service::next_receive_address`:
/// that function (and `current_receive_address`) always requires a real
/// Sapling or Ironwood spending key to derive an address, which this wallet
/// never has (it only ever holds viewing keys, via `import_viewing_wallet`).
/// `generate_address_for_key` derives straight from the stored viewing key
/// instead, which is the only address-generation path that actually works
/// for a watch-only wallet.
async fn next_fresh_address(state: &AppState, use_ironwood: bool) -> anyhow::Result<String> {
    crate::safe_calls::generate_address_for_key(state.wallet_id.clone(), state.key_id, use_ironwood)
        .await
}

/// Pool to use when a request doesn't name one explicitly - whatever this
/// wallet's own Ironwood-activation flag currently says. This is also the
/// only pool a pre-`pool`-field BTCPay client will ever get, so upgrading
/// walletd alone doesn't change existing behavior.
fn default_use_ironwood(state: &AppState) -> anyhow::Result<bool> {
    pirate_wallet_service::is_ironwood_active_for_wallet(state.wallet_id.clone())
}

/// Parses the optional `"pool"` request field ("sapling" or "ironwood",
/// case-insensitive) into `use_ironwood`. `None` means the caller didn't
/// specify a pool at all.
fn parse_pool(pool: Option<&str>) -> anyhow::Result<Option<bool>> {
    match pool {
        None => Ok(None),
        Some(s) => match s.to_ascii_lowercase().as_str() {
            "sapling" => Ok(Some(false)),
            "ironwood" => Ok(Some(true)),
            other => Err(anyhow!(
                "Unknown pool '{other}'; expected 'sapling' or 'ironwood'"
            )),
        },
    }
}

fn pool_name(use_ironwood: bool) -> &'static str {
    if use_ironwood {
        "ironwood"
    } else {
        "sapling"
    }
}

#[derive(Deserialize)]
struct CreateAccountRequest {
    #[allow(dead_code)]
    label: Option<String>,
    /// "sapling" or "ironwood"; defaults to the wallet's own Ironwood
    /// activation flag when omitted (old clients, or callers with no
    /// preference).
    #[serde(default)]
    pool: Option<String>,
}

#[derive(Serialize)]
struct CreateAccountResponse {
    account_index: u32,
    address: String,
}

async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAccountRequest>,
) -> ApiResult<CreateAccountResponse> {
    let explicit_pool = parse_pool(req.pool.as_deref())?;
    let use_ironwood = match explicit_pool {
        Some(v) => v,
        None => default_use_ironwood(&state)?,
    };
    let address = next_fresh_address(&state, use_ironwood).await?;
    let account_index = {
        let bk = state.bookkeeping.lock().await;
        let account_index = bk.create_account(&address)?;
        // Only an explicit request pins the pool; a default resolved from
        // the wallet's current state is left unpinned, so this account
        // keeps auto-following that default if it later changes.
        if let Some(pinned) = explicit_pool {
            bk.set_account_pool(account_index, pinned)?;
        }
        account_index
    };
    Ok(Json(CreateAccountResponse {
        account_index,
        address,
    }))
}

#[derive(Deserialize)]
struct CreateAddressRequest {
    account_index: u32,
    #[allow(dead_code)]
    label: Option<String>,
}

#[derive(Serialize)]
struct CreateAddressResponse {
    address: String,
    address_index: u32,
}

/// Addresses within an account come from whatever pool that account is
/// pinned to, so unlike `create_account` this has no `pool` request field.
/// If the account was never explicitly pinned (via `create_account`'s
/// `pool` field), this re-resolves the wallet's current default every
/// call - so an account an operator never picked a pool for keeps
/// following that default automatically (e.g. it starts generating
/// Ironwood addresses on its own once Ironwood activates).
async fn create_address(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAddressRequest>,
) -> ApiResult<CreateAddressResponse> {
    let use_ironwood = match state.bookkeeping.lock().await.account_pool(req.account_index)? {
        Some(v) => v,
        None => default_use_ironwood(&state)?,
    };
    let address = next_fresh_address(&state, use_ironwood).await?;
    let address_index = state
        .bookkeeping
        .lock()
        .await
        .create_address(req.account_index, &address)?;
    Ok(Json(CreateAddressResponse {
        address,
        address_index,
    }))
}

#[derive(Serialize)]
struct AccountBalance {
    account_index: u32,
    label: String,
    balance: u64,
    unlocked_balance: u64,
    base_address: String,
    tag: String,
    /// "sapling" or "ironwood" - the pool addresses in this account
    /// currently come from. Lets the BTCPay store settings page show which
    /// pool each existing account actually uses right now.
    pool: String,
    /// `true` if `pool` was explicitly pinned and will never change;
    /// `false` if it's just today's resolution of the wallet's default and
    /// may later flip (e.g. once Ironwood activates).
    pool_pinned: bool,
}

#[derive(Serialize)]
struct GetAccountsResponse {
    subaddress_accounts: Vec<AccountBalance>,
    total_balance: u64,
    total_unlocked_balance: u64,
}

async fn get_accounts(State(state): State<Arc<AppState>>) -> ApiResult<GetAccountsResponse> {
    let balances = crate::safe_calls::list_address_balances(state.wallet_id.clone(), None).await?;
    let by_address: HashMap<String, _> = balances
        .into_iter()
        .map(|b| (b.address.clone(), b))
        .collect();

    let bk = state.bookkeeping.lock().await;
    let mut subaddress_accounts = Vec::new();
    let mut total_balance = 0u64;
    let mut total_unlocked_balance = 0u64;
    // Only computed if some account turns out to be unpinned below - avoids
    // an extra call when every account already has an explicit pool.
    let mut current_default_use_ironwood: Option<bool> = None;
    for account_index in bk.list_account_indices()? {
        let base_address = bk.base_address(account_index)?.unwrap_or_default();
        let mut balance = 0u64;
        let mut unlocked_balance = 0u64;
        for address in bk.addresses_for_account(account_index)? {
            if let Some(b) = by_address.get(&address) {
                balance = balance.saturating_add(b.balance);
                unlocked_balance = unlocked_balance.saturating_add(b.spendable);
            }
        }
        total_balance = total_balance.saturating_add(balance);
        total_unlocked_balance = total_unlocked_balance.saturating_add(unlocked_balance);
        let pinned = bk.account_pool(account_index)?;
        let pool_pinned = pinned.is_some();
        let use_ironwood = match pinned {
            Some(v) => v,
            None => match current_default_use_ironwood {
                Some(v) => v,
                None => {
                    let v = default_use_ironwood(&state)?;
                    current_default_use_ironwood = Some(v);
                    v
                }
            },
        };
        subaddress_accounts.push(AccountBalance {
            account_index,
            label: String::new(),
            balance,
            unlocked_balance,
            base_address,
            tag: String::new(),
            pool: pool_name(use_ironwood).to_string(),
            pool_pinned,
        });
    }

    Ok(Json(GetAccountsResponse {
        subaddress_accounts,
        total_balance,
        total_unlocked_balance,
    }))
}

#[derive(Serialize, Clone)]
struct SubAddress {
    major: u32,
    minor: u32,
}

#[derive(Serialize, Clone)]
struct Transfer {
    address: String,
    amount: u64,
    confirmations: u32,
    height: u32,
    fee: u64,
    note: String,
    payment_id: String,
    subaddr_index: SubAddress,
    suggested_confirmations_threshold: u32,
    timestamp: u64,
    txid: String,
    r#type: String,
    unlock_time: u64,
}

/// Deposits attributed to a BTCPay-allocated address, reshaped into walletd's
/// `Transfer` shape. Deposits on addresses this shim never allocated (e.g.
/// pre-existing funds on the FVK before it was handed to BTCPay) are skipped
/// since there is no account/sub-account to report them under.
async fn deposits_as_transfers(state: &AppState) -> anyhow::Result<Vec<Transfer>> {
    let deposits = crate::safe_calls::list_incoming_deposits(state.wallet_id.clone(), None).await?;
    let bk = state.bookkeeping.lock().await;
    let mut transfers = Vec::new();
    for deposit in deposits {
        let Some((account_index, sub_account)) = bk.lookup_address(&deposit.address)? else {
            continue;
        };
        // deposit.confirmations is already computed relative to the
        // wallet's own locally-synced height - no need to recompute it
        // against sync_status separately.
        transfers.push(Transfer {
            address: deposit.address,
            amount: deposit.value,
            confirmations: deposit.confirmations,
            height: deposit.height.unwrap_or(0),
            fee: 0,
            note: String::new(),
            payment_id: String::new(),
            subaddr_index: SubAddress {
                major: account_index,
                minor: sub_account,
            },
            suggested_confirmations_threshold: state.config.confirmations,
            timestamp: deposit.timestamp.unwrap_or(0).max(0) as u64,
            txid: deposit.txid,
            r#type: "in".to_string(),
            unlock_time: 0,
        });
    }
    Ok(transfers)
}

#[derive(Deserialize)]
struct GetTransactionByIdRequest {
    txid: String,
    account_index: u32,
}

#[derive(Serialize)]
struct GetTransactionByIdResponse {
    transfer: Transfer,
    transfers: Vec<Transfer>,
}

async fn get_transfer_by_txid(
    State(state): State<Arc<AppState>>,
    Json(req): Json<GetTransactionByIdRequest>,
) -> ApiResult<GetTransactionByIdResponse> {
    let transfer = deposits_as_transfers(&state)
        .await?
        .into_iter()
        .find(|t| {
            t.txid.eq_ignore_ascii_case(&req.txid) && t.subaddr_index.major == req.account_index
        })
        .ok_or_else(|| anyhow!("No such transaction"))?;
    Ok(Json(GetTransactionByIdResponse {
        transfer: transfer.clone(),
        transfers: vec![transfer],
    }))
}

#[derive(Deserialize)]
struct GetTransfersRequest {
    account_index: u32,
    #[serde(rename = "in")]
    #[allow(dead_code)]
    incoming: bool,
    subaddr_indices: Vec<u32>,
}

#[derive(Serialize)]
struct GetTransfersResponse {
    r#in: Vec<Transfer>,
}

async fn get_transfers(
    State(state): State<Arc<AppState>>,
    Json(req): Json<GetTransfersRequest>,
) -> ApiResult<GetTransfersResponse> {
    let transfers: Vec<Transfer> = deposits_as_transfers(&state)
        .await?
        .into_iter()
        .filter(|t| t.subaddr_index.major == req.account_index)
        .filter(|t| {
            req.subaddr_indices.is_empty() || req.subaddr_indices.contains(&t.subaddr_index.minor)
        })
        .collect();
    Ok(Json(GetTransfersResponse { r#in: transfers }))
}

#[derive(Serialize)]
struct GetHeightResponse {
    height: u32,
}

async fn get_height(State(state): State<Arc<AppState>>) -> ApiResult<GetHeightResponse> {
    let status = pirate_wallet_service::sync_status(state.wallet_id.clone())?;
    Ok(Json(GetHeightResponse {
        height: status.target_height as u32,
    }))
}

#[derive(Serialize)]
struct SyncInfoResponse {
    height: u32,
}

async fn sync_info(State(state): State<Arc<AppState>>) -> ApiResult<SyncInfoResponse> {
    let status = pirate_wallet_service::sync_status(state.wallet_id.clone())?;
    Ok(Json(SyncInfoResponse {
        height: status.target_height as u32,
    }))
}

async fn request_scan(State(state): State<Arc<AppState>>) -> Result<(), ApiError> {
    pirate_wallet_service::start_sync(state.wallet_id.clone(), SyncMode::Compact).await?;
    Ok(())
}

#[derive(Serialize)]
struct GetFeeEstimateResponse {
    fee: u64,
}

async fn get_fee_estimate() -> ApiResult<GetFeeEstimateResponse> {
    let fee_info = pirate_wallet_service::get_fee_info()?;
    Ok(Json(GetFeeEstimateResponse {
        fee: fee_info.default_fee,
    }))
}
