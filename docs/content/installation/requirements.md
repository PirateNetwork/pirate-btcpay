---
title: Requirements
weight: 10
---

## Pirated & Lightwalletd

You need a connection to a `lightwalletd` built for Pirate Chain, configured
via `lwd_urls` (see the Usage page).

By default, `pirate-btcpay-walletd` connects to Pirate Network's own hosted
endpoints — no self-hosted `lightwalletd` is required to get started:

```
https://lightd1.pirate.black:443
https://64.23.167.130:9067
```

`lwd_urls` takes a comma-separated, priority-ordered list — if the primary
goes down, the wallet fails over to the next one automatically and switches
back once it recovers.

If you'd rather run your own `lightwalletd` (for full control or
decentralization), point `lwd_urls` at it instead:

```
./pirated
./lightwalletd --pirate-conf-path ~/.komodo/PIRATE/PIRATE.conf --no-tls-very-insecure --data-dir .  --log-file /dev/stdout
```

Lightwalletd is being upgraded to also expose Tor/I2P endpoints; set
`tunnel_mode` (see the Usage page) to connect over one of those instead of
clearnet.

## Rust

You need to install [Rust](https://www.rust-lang.org/).
