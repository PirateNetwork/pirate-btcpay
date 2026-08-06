mod bookkeeping;
mod config;
mod failover;
mod notify;
mod routes;

use anyhow::Context;
use bookkeeping::Bookkeeping;
use config::AppConfig;
use failover::FailoverState;
use pirate_wallet_service::WalletId;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub wallet_id: WalletId,
    pub bookkeeping: Mutex<Bookkeeping>,
    pub failover: Mutex<FailoverState>,
    pub config: AppConfig,
    pub http: reqwest::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env().context("Failed to load config")?;
    let db_dir = PathBuf::from(&config.db_dir);

    pirate_wallet_service::configure_wallet_storage(
        config.db_dir.clone(),
        config.storage_passphrase.clone(),
    )
    .context("Failed to open wallet storage")?;

    tracing::info!("Setting tunnel mode: {:?}", config.tunnel_mode);
    pirate_wallet_service::set_tunnel(config.tunnel_mode.clone())
        .context("Failed to set tunnel mode")?;

    let bookkeeping = Bookkeeping::open(&db_dir).context("Failed to open bookkeeping db")?;

    let wallet_id = match bookkeeping.get_wallet_id()? {
        Some(id) => id,
        None => {
            let id = pirate_wallet_service::import_viewing_wallet(
                "btcpayserver".to_string(),
                Some(config.sapling_fvk.clone()),
                Some(config.ironwood_fvk.clone()),
                config.starting_height,
            )
            .context("Failed to import viewing wallet")?;
            bookkeeping.set_wallet_id(&id)?;
            id
        }
    };

    tracing::info!("pirate-btcpay-walletd serving wallet {}", wallet_id);

    let failover = FailoverState::new(config.lwd_urls.clone());
    pirate_wallet_service::set_lightd_endpoint(wallet_id.clone(), failover.active_url().to_string(), None)
        .context("Failed to set initial lightd endpoint")?;

    let state = Arc::new(AppState {
        wallet_id,
        bookkeeping: Mutex::new(bookkeeping),
        failover: Mutex::new(failover),
        config,
        http: reqwest::Client::new(),
    });

    notify::spawn(state.clone());

    let app = routes::router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
