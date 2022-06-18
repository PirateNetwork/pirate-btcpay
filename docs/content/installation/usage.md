---
title: Usage
weight: 30
---

`walletd` is meant to be used with BTCPayserver. However, it is a standalone
server that answers to REST RPC.

## Configuration

The configuratio is in `Rocket.toml`

```toml
[default]
db_dir = "."
confirmations = 6
lwd_url = "http://localhost:9067"
poll_interval = 10
notify_host = "localhost"
fvk = "zxviews1qw73xgmkqqqqpqx3f2a93lddczpdk4kt58dh5nceks7l8jhw6zc30up6ph3etllafdp5f90st33sa6p27jmevdelhx6fpmhk698vyu7lfn0z6gpqr3nfedhec5k9x0w826fej59j84xqkndqe0ymety9sgc4ttfn82gheu988d9dr9n99zege9egzadkwf8eeur3xm5tajctx625ux67w0dlsl8hsa4l889js800jq6xvcnyp3v0tnh65cqwq6xfnk9cqczue24wyyskv63ke"
starting_height = 1915516

[debug]
address = "127.0.0.1"

[release]
address = "0.0.0.0"
```

The default section applies to both profile. The debug section is for debug builds and the release profile is for the 
release build.

- `db_dir`: Location of the database directory where the `wallet.db` file is created.
- `confirmations`: The number of block confirmations needed for a payment to be unlocked
- `lwd_url`: URL of the `lightwalletd` server
- `poll_interval`: Frequency at which the wallet will check new blocks
- `notify_host`: Hostname of the BTCPayserver. `walletd` calls back BTCPayserver when it detects an incoming payment.
- `fvk`: The wallet Extended Full Viewing Key
- `starting_height`: The birth height of the wallet, i.e. no transactions occurred before that height.
- `address`: Listening address

## Wallet Configuration

{{%notice warning%}}
At a minimum, you will need to change `fvk` and `starting_height`.
{{%/notice %}}
