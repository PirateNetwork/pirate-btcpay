#[macro_use]
extern crate rocket;

use anyhow::Context;
use rocket::figment::Figment;
use rocket::figment::providers::{Env, Format, Toml};
use walletd::config::AppConfig;
use walletd::{App, APPSTORE, get_appstore, Result};
use walletd::rpc::*;

#[rocket::main]
async fn main() -> Result<()> {
    env_logger::init();
    let _ = dotenv::dotenv();

    let figment = Figment::new()
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Env::prefixed("BTCPAYSERVER_"));
    let app_config: AppConfig = figment.extract().context("Cannot parse config")?;

    let poll_interval = app_config.poll_interval;
    let app = App::new(app_config).await;
    let _ = APPSTORE.fill(app);
    log::info!("{:?}", get_appstore().config);

    monitor_task(poll_interval);

    let rocket = rocket::build();
    let _ = rocket.mount(
        "/",
        routes![
                create_account,
                create_address,
                get_accounts,
                get_transaction,
                get_transfers,
                get_height,
                sync_info,
                request_scan,
            ])
        .launch()
        .await?;

    Ok(())
}
