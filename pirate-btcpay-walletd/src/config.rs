use anyhow::{anyhow, Context, Result};
use pirate_wallet_service::TunnelMode;

/// Configuration for the walletd-compatible shim.
///
/// Field names and the `BTCPAYSERVER_`-prefixed env var convention match the
/// legacy `walletd` config contract exactly, so `btcpayserver-docker` only
/// needs to add `storage_passphrase`, not rewire everything else.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Sapling extended full viewing key for the wallet this shim serves.
    pub sapling_fvk: String,
    /// Ironwood extended full viewing key for the same wallet.
    ///
    /// Required, not optional: `pirate-wallet-service` derives new receive
    /// addresses as Ironwood once the network's Ironwood activation height
    /// passes, and errors out if no Ironwood key is on file at that point.
    /// Omitting this would mean address generation silently breaks the
    /// moment the network activates Ironwood, not at deploy time.
    pub ironwood_fvk: String,
    /// Passphrase for the wallet-service's encrypted-at-rest storage.
    ///
    /// Independent from `sapling_fvk`/`ironwood_fvk` by design: deriving it
    /// from either viewing key would mean anyone with them (which flow
    /// through less-trusted paths, like the BTCPay store config) could also
    /// reconstruct the wallet DB's encryption key, defeating the point of
    /// separate at-rest encryption.
    pub storage_passphrase: String,
    pub db_dir: String,
    pub starting_height: u32,
    pub confirmations: u32,
    /// Lightwalletd endpoints in priority order (index 0 = primary). See
    /// `failover.rs` for how the notify loop fails over on repeated sync
    /// failures and switches back once a higher-priority endpoint recovers.
    pub lwd_urls: Vec<String>,
    pub poll_interval: u32,
    pub notify_host: String,
    /// Transport used to reach `lwd_urls` (and any other outbound wallet
    /// traffic): Tor, I2P, a SOCKS5 proxy, or a direct clearnet connection.
    /// This is one global mode, not per-URL — lightwalletd is being
    /// upgraded to expose Tor/I2P endpoints, and `pirate-wallet-service`
    /// (which this shim is built on) already supports connecting over them.
    /// Defaults to `direct` to preserve existing deployments' behavior;
    /// operators who want the privacy properties opt in explicitly.
    pub tunnel_mode: TunnelMode,
}

fn env_var(name: &str) -> Result<String> {
    std::env::var(format!("BTCPAYSERVER_{name}"))
        .with_context(|| format!("Missing required env var BTCPAYSERVER_{name}"))
}

fn env_var_or(name: &str, default: &str) -> String {
    std::env::var(format!("BTCPAYSERVER_{name}")).unwrap_or_else(|_| default.to_string())
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        let starting_height: u32 = env_var_or("starting_height", "0")
            .parse()
            .map_err(|_| anyhow!("BTCPAYSERVER_starting_height must be a u32"))?;
        let confirmations: u32 = env_var_or("confirmations", "6")
            .parse()
            .map_err(|_| anyhow!("BTCPAYSERVER_confirmations must be a u32"))?;
        let poll_interval: u32 = env_var_or("poll_interval", "10")
            .parse()
            .map_err(|_| anyhow!("BTCPAYSERVER_poll_interval must be a u32"))?;

        Ok(AppConfig {
            sapling_fvk: env_var("sapling_fvk")?,
            ironwood_fvk: env_var("ironwood_fvk")?,
            storage_passphrase: env_var("storage_passphrase")?,
            db_dir: env_var_or("db_dir", "."),
            starting_height,
            confirmations,
            lwd_urls: parse_lwd_urls(&env_var("lwd_urls")?)?,
            poll_interval,
            notify_host: env_var("notify_host")?,
            tunnel_mode: parse_tunnel_mode(
                &env_var_or("tunnel_mode", "direct"),
                std::env::var("BTCPAYSERVER_tunnel_socks5_url")
                    .ok()
                    .filter(|s| !s.trim().is_empty()),
            )?,
        })
    }
}

fn parse_lwd_urls(value: &str) -> Result<Vec<String>> {
    let urls: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect();
    if urls.is_empty() {
        return Err(anyhow!(
            "BTCPAYSERVER_lwd_urls must contain at least one URL"
        ));
    }
    Ok(urls)
}

fn parse_tunnel_mode(mode: &str, socks5_url: Option<String>) -> Result<TunnelMode> {
    match mode.trim().to_lowercase().as_str() {
        "tor" => Ok(TunnelMode::Tor),
        "i2p" => Ok(TunnelMode::I2p),
        "socks5" => {
            let url = socks5_url.ok_or_else(|| {
                anyhow!("BTCPAYSERVER_tunnel_socks5_url is required when tunnel_mode=socks5")
            })?;
            Ok(TunnelMode::Socks5 { url })
        }
        "direct" => Ok(TunnelMode::Direct),
        other => Err(anyhow!(
            "Invalid BTCPAYSERVER_tunnel_mode '{other}': expected tor, i2p, socks5, or direct"
        )),
    }
}
