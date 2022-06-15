#[macro_use]
extern crate rocket;

use anyhow::Context;
use walletd::config::{AppConfig, CONFIG};
use walletd::Result;
use walletd::rpc::*;

#[rocket::main]
async fn main() -> Result<()> {
    env_logger::init();
    let _ = dotenv::dotenv();

    let rocket = rocket::build();
    let app_config: AppConfig = rocket.figment().extract().context("Cannot parse config")?;
    CONFIG.fill(app_config).unwrap();
    log::info!("{:?}", CONFIG.borrow().unwrap());

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
