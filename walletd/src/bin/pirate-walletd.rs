#[macro_use]
extern crate rocket;

use anyhow::Context;
use walletd::config::{AppConfig, CONFIG};
use walletd::Result;

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
            ])
        .launch()
        .await?;

    Ok(())
}
