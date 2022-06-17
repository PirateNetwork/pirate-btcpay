# Clone

```
git clone https://github.com/hhanh01/pirate-btcpay.git
```

Edit the `lwd_url` in `Rocket.toml`

# Build

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