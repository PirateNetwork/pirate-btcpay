use delegate::delegate;
use rusqlite::params;
use crate::{get_appstore, Result};
use crate::db::Db;
use crate::rpc::data::{CreateAccountResponse, CreateAddressResponse};

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
