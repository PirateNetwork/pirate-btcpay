---
title: Requirements
weight: 10
---

## Pirated & Lightwalletd

You need a connection to a `lightwalletd` built for Pirate Chain.

The URL will typically have the port 9067 and look like `https://lwd.pirate-chain.com:9067`

We will call this the `LWD_URL` in the rest of the documentation.


```
./pirated
./lightwalletd --pirate-conf-path ~/.komodo/PIRATE/PIRATE.conf --no-tls-very-insecure --data-dir .  --log-file /dev/stdout
```

## Rust

You need to install [Rust](https://www.rust-lang.org/).
