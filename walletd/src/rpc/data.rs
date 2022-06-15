use serde::{Serialize, Deserialize};
use crate::data::{AccountBalance, Transfer};

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    label: Option<String>,
}

#[derive(Serialize)]
pub struct CreateAccountResponse {
    account_index: u32,
    address: String,
}

#[derive(Deserialize)]
pub struct CreateAddressRequest {
    account_index: u32,
    label: Option<String>,
}

#[derive(Serialize)]
pub struct CreateAddressResponse {
    address: String,
    address_index: u32,
}

#[derive(Deserialize)]
pub struct GetAccountsRequest {
    tag: Option<String>,
}

#[derive(Serialize)]
pub struct GetAccountsResponse {
    subaddress_accounts: Vec<AccountBalance>,
    total_balance: u64,
    total_unlocked_balance: u64,
}

#[derive(Deserialize)]
pub struct GetTransactionByIdRequest {
    pub txid: String,
    pub account_index: u32,
}

#[derive(Serialize)]
pub struct GetTransactionByIdResponse {
    pub transfer: Transfer,
    pub transfers: Vec<Transfer>,
}

#[derive(Deserialize)]
pub struct GetTransfersRequest {
    pub account_index: u32,
    pub r#in: bool,
    pub subaddr_indices: Vec<u32>,
}

#[derive(Serialize)]
pub struct GetTransfersResponse {
    pub r#in: Vec<Transfer>,
}

#[derive(Deserialize)]
pub struct GetFeeEstimateRequest;

#[derive(Serialize)]
pub struct GetFeeEstimateResponse {
    pub fee: u64,
}

#[derive(Deserialize)]
pub struct GetHeightRequest;

#[derive(Serialize)]
pub struct GetHeightResponse {
    pub height: u32,
}

#[derive(Deserialize)]
pub struct SyncInfoRequest;

#[derive(Serialize)]
pub struct SyncInfoResponse {
    pub target_height: u32,
    pub height: u32,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    start_height: Option<u32>,
}

#[derive(Serialize)]
pub struct ScanResponse;
