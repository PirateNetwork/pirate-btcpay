---
title: Usage
weight: 30
---

`pirate-btcpay-walletd` is meant to be used with BTCPayserver. However, it is
a standalone server that answers to REST RPC.

## Configuration

Configuration is entirely through environment variables, each prefixed
`BTCPAYSERVER_`:

- `db_dir`: Location of the database directory where the wallet databases are created.
- `confirmations`: The number of block confirmations needed for a payment to be unlocked
- `lwd_urls`: Comma-separated `lightwalletd` server URLs, in priority order (first is primary). The notify loop fails over to the next one after 3 consecutive sync failures, and periodically re-probes higher-priority endpoints so it switches back once one recovers — you don't need to intervene manually.
- `poll_interval`: Frequency at which the wallet will check new blocks
- `notify_host`: Hostname of the BTCPayserver. `pirate-btcpay-walletd` calls back BTCPayserver when it detects an incoming payment.
- `sapling_fvk`: The wallet's Sapling Extended Full Viewing Key
- `ironwood_fvk`: The same wallet's Ironwood Extended Full Viewing Key. Required, not optional — `pirate-btcpay-walletd` derives new receive addresses as Ironwood once the network's Ironwood activation height passes, and address generation fails outright if no Ironwood key is on file at that point. Export it from the same seed as `sapling_fvk`.
- `storage_passphrase`: Passphrase for the wallet's encrypted-at-rest local storage. Independent of `sapling_fvk`/`ironwood_fvk` — never derive it from either viewing key, since anyone with them (which flow through less-trusted paths, like the BTCPay store config) would then also be able to reconstruct the wallet DB's encryption key.
- `starting_height`: The birth height of the wallet, i.e. no transactions occurred before that height.
- `tunnel_mode`: Transport used to reach `lwd_urls`: `direct` (default), `tor`, `i2p`, or `socks5`. Lightwalletd is being upgraded to expose Tor/I2P endpoints, and the wallet engine this shim is built on already supports connecting over them — set this to actually use one instead of clearnet.
- `tunnel_socks5_url`: Proxy URL, required only when `tunnel_mode` is `socks5`.

`db_dir`, `confirmations`, `poll_interval`, `starting_height`, and
`tunnel_mode` have defaults (`.`, `6`, `10`, `0`, `direct` respectively);
`sapling_fvk`, `ironwood_fvk`, `storage_passphrase`, `lwd_urls`, and
`notify_host` are required and the process will fail to start without them.

## Wallet Configuration

{{%notice warning%}}
At a minimum, you will need to set `sapling_fvk`, `ironwood_fvk`,
`starting_height`, and a real `storage_passphrase` (not the placeholder from
the sample compose file).
{{%/notice %}}
