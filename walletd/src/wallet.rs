use delegate::delegate;
use rusqlite::params;
use tokio::sync::MutexGuard;
use crate::{get_appstore, Result};
use crate::db::Db;
use crate::rpc::data::CreateAccountResponse;

pub struct Wallet;

impl Wallet {
    pub async fn create_account(label: Option<String>) -> Result<CreateAccountResponse> {
        let app = get_appstore();
        let account = Db::create_account(app.store.clone(), label, &app.fvk);
        Ok(account.await?)
    }
}


    // let id_account: Option<u32> =
    //     connection.query_row("SELECT MAX(account) FROM addresses", [], |row| row.get(0))?;
    // let id_account = id_account.map(|id| id + 1).unwrap_or(0);
    // let (diversifier_index, address) = self.next_diversifier(&connection)?;
    //
    // connection.execute("INSERT INTO addresses(label, account, sub_account, address, diversifier_index) VALUES (?1,?2,?3,?4,?5)",
    //                    params![name, id_account, 0, &address, diversifier_index])?;
    // let account = Account {
    //     account_index: id_account,
    //     address,
    // };
