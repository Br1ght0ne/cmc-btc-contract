# CMC-BTC contract (Near)

## Setup

### Prerequisites

- [near-sdk prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)
- software: `curl`, `jq`
- env variables:
  - `CMC_API_KEY` - your CoinMarketCap API key
  - `CONTRACT_ACCOUNT` - your contract account name (e.g. `cmc-btc.yourname.testnet`)
  - `USER_ACCOUNT` - your NEAR account name (e.g. `yourname.testnet` - you _need_ to have the keys on your machine)

## Usage

- build: `./build.sh`;
- deploy: `near deploy cmc-btc.brightone.testnet --wasmFile res/cmc_btc_contract.wasm`;
- schedule `update_price.sh` to run every hour to update the prices.
