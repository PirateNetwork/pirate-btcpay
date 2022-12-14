use serde::{Serialize, Deserialize};
use crate::data::{AccountBalance, Transfer};

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    pub label: Option<String>,
}

#[derive(Serialize)]
pub struct CreateAccountResponse {
    pub account_index: u32,
    pub address: String,
}

#[derive(Deserialize)]
pub struct CreateAddressRequest {
    pub account_index: u32,
    pub label: Option<String>,
}

#[derive(Serialize)]
pub struct CreateAddressResponse {
    pub address: String,
    pub address_index: u32,
}

#[derive(Serialize)]
pub struct GetAccountsResponse {
    pub subaddress_accounts: Vec<AccountBalance>,
    pub total_balance: u64,
    pub total_unlocked_balance: u64,
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

#[derive(Serialize)]
pub struct GetHeightResponse {
    pub height: u32,
}

#[derive(Deserialize)]
pub struct MakeURIRequest {
    pub address: String,
    pub amount: u64,
    pub payment_id: String,
    pub tx_description: String,
    pub recipient_name: String,
}

#[derive(Serialize)]
pub struct MakeURIResponse {
    pub uri: String,
}

#[derive(Serialize)]
pub struct SyncInfoResponse {
    pub height: u32,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    #[allow(dead_code)]
    start_height: Option<u32>,
}

#[derive(Serialize)]
pub struct ScanResponse;
