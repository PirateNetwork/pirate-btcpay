use rocket::serde::json::Json;
use crate::{Error, scan_blocks};
use crate::rpc::data::*;

pub mod data;

#[post("/create_account", data = "<request>")]
pub fn create_account(
    request: Json<CreateAccountRequest>
) -> Result<Json<CreateAccountResponse>, Error> {
    todo!()
}

#[post("/create_address", data = "<request>")]
pub fn create_address(
    request: Json<CreateAddressRequest>,
) -> Result<Json<CreateAddressResponse>, Error> {
    todo!()
}

#[post("/get_accounts", data = "<_request>")]
pub async fn get_accounts(
    _request: Json<GetAccountsRequest>,
) -> Result<Json<GetAccountsResponse>, Error> {
    todo!()
}

#[post("/get_transfer_by_txid", data = "<request>")]
pub async fn get_transaction(
    request: Json<GetTransactionByIdRequest>,
) -> Result<Json<GetTransactionByIdResponse>, Error> {
    todo!()
}

#[post("/get_transfers", data = "<request>")]
pub async fn get_transfers(
    request: Json<GetTransfersRequest>,
) -> Result<Json<GetTransfersResponse>, Error> {
    todo!()
}

#[post("/get_fee_estimate", data = "<_request>")]
pub fn get_fee_estimate(_request: Json<GetFeeEstimateRequest>) -> Result<Json<GetFeeEstimateResponse>, Error> {
    todo!()
}

#[post("/get_height", data = "<_request>")]
pub async fn get_height(_request: Json<GetHeightRequest>) -> Result<Json<GetHeightResponse>, Error> {
    todo!()
}

#[post("/sync_info", data = "<_request>")]
pub async fn sync_info(_request: Json<SyncInfoRequest>) -> Result<Json<SyncInfoResponse>, Error> {
    todo!()
}

#[post("/request_scan")]
pub async fn request_scan() -> Result<(), Error> {
    scan_blocks().await?;
    Ok(())
}

