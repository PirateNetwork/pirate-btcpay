#[macro_use]
extern crate rocket;

use anyhow::Context;
use walletd::config::AppConfig;
use walletd::{APP, App, Result};
use walletd::rpc::*;

#[rocket::main]
async fn main() -> Result<()> {
    env_logger::init();
    let _ = dotenv::dotenv();

    let rocket = rocket::build();
    let app_config: AppConfig = rocket.figment().extract().context("Cannot parse config")?;
    let app = App::new(app_config);
    let _ = APP.fill(app);
    log::info!("{:?}", APP.borrow().unwrap().config);

    let _ = rocket.mount(
        "/",
        routes![
                create_account,
                create_address,
                get_accounts,
                get_transaction,
                get_transfers,
                get_fee_estimate,
                get_height,
                sync_info,
                request_scan,
            ])
        .launch()
        .await?;

    Ok(())
}
