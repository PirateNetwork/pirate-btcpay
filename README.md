# Clone

```
git clone https://github.com/PirateNetwork/pirate-btcpay.git
```

Edit the `lwd_url` in `Rocket.toml`
Edit the `fvk` in `Rocket.toml` with your Pirate Chain Full Viewing Key
Edit the `starting_height` in `Rocket.toml`
Edit

# Build

CD into the repo root ~/pirate-btcpay

```
git submodule update --init
cargo b --release
```

# Run

```
./target/release/walletd
```

# Test

```
cd tests
npm install
npx mocha
```

# API Endpoints

```
/create_account
/create_address
/get_accounts
/get_transfer_by_txid
/get_transfers
/get_height
/sync_info
/request_scan
```