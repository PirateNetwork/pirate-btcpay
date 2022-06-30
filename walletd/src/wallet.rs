use delegate::delegate;
use rusqlite::params;
use crate::{get_appstore, Result};
use crate::data::Transfer;
use crate::db::Db;
use crate::rpc::data::{CreateAccountResponse, CreateAddressResponse, GetTransactionByIdResponse, MakeURIResponse};

pub async fn create_account(label: Option<String>) -> Result<CreateAccountResponse> {
    let app = get_appstore();
    let account = Db::create_account(app.store.clone(), label, &app.fvk);
    Ok(account.await?)
}

pub async fn create_address(label: Option<String>, account_index: u32) -> Result<CreateAddressResponse> {
    let app = get_appstore();
    let account = Db::create_address(app.store.clone(), label, account_index, &app.fvk);
    Ok(account.await?)
}

pub fn make_uri(address: &str, amount: u64, payment_id: &str, tx_description: &str, recipient_name: &str) -> Result<String> {
    todo!()
}
