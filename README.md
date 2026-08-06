<!--
Old readme - skip to line 41 for current one


# Clone

```
git clone https://github.com/PirateNetwork/pirate-btcpay.git
```

Edit the `lwd_url` in `Rocket.toml`
Edit the `fvk` in `Rocket.toml` with your Pirate Chain Full Viewing Key
Edit the `starting_height` in `Rocket.toml`

# Build

CD into the repo root ~/pirate-btcpay

```
git submodule update --init --recursive
cargo b --release -p pirate-btcpay-walletd
```

# Run

```
./target/release/pirate-btcpay-walletd
```

# Test

```
cd tests
npm install
npx mocha
```
-->



## Pirate Chain BTCPay Server

A minimal, self-hostable BTCPay Server setup for Pirate Chain (ARRR). This pulls prebuilt images and runs them with Docker.

## Prerequisites
- **Docker**: Install Docker Desktop or Docker Engine for your OS (`https://www.docker.com`)
- **curl**: Installed by default on macOS/Linux; on Windows, use `curl.exe --install` in CMD/PowerShell

## Quick start
Follow the instructions for your OS. All paths below create and use a new `pirate-btcpay` folder in your home directory.

### Windows (CMD)
1. Install Docker from `https://www.docker.com` and start it
2. Open CMD
3. Create a working folder and download the compose file:
   ```
   mkdir %USERPROFILE%\pirate-btcpay && cd %USERPROFILE%\pirate-btcpay
   curl -fsSL https://raw.githubusercontent.com/PirateNetwork/pirate-btcpay/main/docker/docker-compose.yml -o compose.yaml
   ```
4. Edit config values in `compose.yaml` file:
   - Set your Pirate Full Viewing Key and wallet birthday height from your wallet:
   ```
   sapling_fvk: Your-Sapling-Full-Viewing-Key
   ironwood_fvk: Your-Ironwood-Full-Viewing-Key (export from the same seed as sapling_fvk)
   storage_passphrase: Replace-The-Sample-Placeholder-With-A-Real-Passphrase
   starting_height: 200000 (example, base this on your wallet's birthday height)
   confirmations: 6 (Set to 6 by default. You can change the number of confirmations before a transaction is accepted)
   ```
5. Start services:
   ```
   docker compose up -d default
   ```
6. Open `http://localhost:14142/register` in your browser
7. Create an account
8. Create a store and set default currency to ARRR
9. Go to Settings → Pirate → Modify → tick Enabled → Save

### Windows (PowerShell)
1. Install Docker from `https://www.docker.com` and start it
2. Open PowerShell
3. Create a working folder and download the compose file:
   ```
   New-Item -ItemType Directory -Force -Path "$HOME/pirate-btcpay" | Out-Null
   Set-Location "$HOME/pirate-btcpay"
   curl.exe -fsSL https://raw.githubusercontent.com/PirateNetwork/pirate-btcpay/main/docker/docker-compose.yml -o compose.yaml
   ```
4. Edit `compose.yaml` and set `sapling_fvk` and `starting_height` as step 4 in Windows CMD instructions
5. Start services:
   ```
   docker compose up -d default
   ```
6. Complete the same UI steps as steps 6 to 9 in Windows CMD instructions

### macOS (Terminal)
1. Install Docker Desktop for Mac and start it
2. In Terminal:
   ```
   mkdir -p "$HOME/pirate-btcpay" && cd "$HOME/pirate-btcpay"
   curl -fsSL https://raw.githubusercontent.com/PirateNetwork/pirate-btcpay/main/docker/docker-compose.yml -o compose.yaml
   ```
3. Edit `compose.yaml` and set `sapling_fvk` and `starting_height` as step 4 in Windows CMD instructions
4. Start services:
   ```
   docker compose up -d default
   ```
5. Complete the same UI steps as steps 6 to 9 in Windows CMD instructions`

### Linux (bash)
1. Install Docker Engine or Docker Desktop and ensure your user can run Docker
2. In a terminal:
   ```
   mkdir -p "$HOME/pirate-btcpay" && cd "$HOME/pirate-btcpay"
   curl -fsSL https://raw.githubusercontent.com/PirateNetwork/pirate-btcpay/main/docker/docker-compose.yml -o compose.yaml
   ```
3. Edit `compose.yaml` and set `sapling_fvk` and `starting_height` as step 4 in Windows CMD instructions
4. Start services:
   ```
   docker compose up -d default
   ```
5. Complete the same UI steps as steps 6 to 9 in Windows CMD instructions

## Configuration notes
- **Sapling Full Viewing Key**: Put your ARRR Sapling FVK in `sapling_fvk`
- **Ironwood Full Viewing Key**: Put the same wallet's Ironwood FVK in `ironwood_fvk`. Required — address generation fails once the network activates Ironwood if this is missing.
- **Storage passphrase**: Set `storage_passphrase` to a real secret. It encrypts the wallet's local database and is intentionally independent of `sapling_fvk`/`ironwood_fvk` — never reuse or derive it from either viewing key.
- **starting_height**: Use your wallet’s birthday height for fastest sync
- **Lightwalletd URLs**: `lwd_urls` takes a comma-separated, priority-ordered list (defaults to `https://lightd1.pirate.black:443,https://64.23.167.130:9067`). If the primary goes down, the wallet fails over to the next one automatically and switches back once it recovers — no manual intervention needed.
- **Tor / I2P**: Set `tunnel_mode` to `tor`, `i2p`, or `socks5` (with `tunnel_socks5_url`) to reach `lwd_urls` over that transport instead of clearnet — useful once lightwalletd's Tor/I2P endpoints are live. Defaults to `direct`.

## Troubleshooting
- If the site isn’t reachable, ensure Docker is running and ports aren’t in use
- If walletd shows unhealthy, check logs and confirm `sapling_fvk`, `ironwood_fvk`, `storage_passphrase`, `lwd_urls`, and `starting_height` are all set correctly

## Build and use your own Docker images
If you prefer to build and publish your own `server` and `walletd` images, follow these steps.

1) Build images locally (from the repo root):
```
docker build -f docker/Dockerfile-server -t YOUR_DOCKERHUB_USERNAME/pirate_btcpay_server:YOUR_TAG .
docker build -f docker/Dockerfile-walletd -t YOUR_DOCKERHUB_USERNAME/pirate_btcpay_walletd:YOUR_TAG .
```

2) Publish images to Docker Hub (or your registry):
```
docker login
docker push YOUR_DOCKERHUB_USERNAME/pirate_btcpay_server:YOUR_TAG
docker push YOUR_DOCKERHUB_USERNAME/pirate_btcpay_walletd:YOUR_TAG
```

3) Point compose to your images by editing `docker/docker-compose.yml`:
- Replace the `image:` lines under `server` and `walletd` with your image names and tags.
```yaml
  server:
    image: YOUR_DOCKERHUB_USERNAME/pirate_btcpay_server:YOUR_TAG
    # ...

  walletd:
    image: YOUR_DOCKERHUB_USERNAME/pirate_btcpay_walletd:YOUR_TAG
    # ...
```

4) Apply changes:
```
docker compose up -d default
```

Note: The compose file currently defaults to `kardingson/pirate_btcpay_server:latest` and
`piratenetwork/pirate-btcpay-walletd:latest`. Overwrite these with your own names/tags as shown above.