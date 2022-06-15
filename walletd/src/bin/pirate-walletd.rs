#[macro_use]
extern crate rocket;
use walletd::Result;

#[rocket::main]
async fn main() -> Result<()> {
    env_logger::init();
    let _ = dotenv::dotenv();

    let rocket = rocket::build();

    let _ = rocket.mount(
        "/",
        routes![
            ])
        .launch()
        .await?;

    Ok(())
}
