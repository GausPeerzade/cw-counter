# CosmWasm Starter Pack

This is a template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

## Creating a new repo from template

Assuming you have a recent version of rust and cargo (v1.58.1+) installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:

**Latest: 1.0.0-beta6**

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME
```

**Older Version**

Pass version as branch flag:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch <version> --name PROJECT_NAME
```

Example:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch 0.16 --name PROJECT_NAME
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.

## Create a Repo

After generating, you have a initialized local git repo, but no commits, and no remote.
Go to a server (eg. github) and create a new upstream repo (called `YOUR-GIT-URL` below).
Then run the following:

```sh
# this is needed to create a valid Cargo.lock file (see below)
cargo check
git branch -M main
git add .
git commit -m 'Initial Commit'
git remote add origin YOUR-GIT-URL
git push -u origin main
```

## CI Support

We have template configurations for both [GitHub Actions](.github/workflows/Basic.yml)
and [Circle CI](.circleci/config.yml) in the generated project, so you can
get up and running with CI right away.

One note is that the CI runs all `cargo` commands
with `--locked` to ensure it uses the exact same versions as you have locally. This also means
you must have an up-to-date `Cargo.lock` file, which is not auto-generated.
The first time you set up the project (or after adding any dep), you should ensure the
`Cargo.lock` file is updated, so the CI will test properly. This can be done simply by
running `cargo check` or `cargo unit-test`.

## Using your project

Once you have your custom repo, you should check out [Developing](./Developing.md) to explain
more on how to run tests and develop code. Or go through the
[online tutorial](https://docs.cosmwasm.com/) to get a better feel
of how to develop.

[Publishing](./Publishing.md) contains useful information on how to publish your contract
to the world, once you are ready to deploy it on a running blockchain. And
[Importing](./Importing.md) contains information about pulling in other contracts or crates
that have been published.

Please replace this README file with information about your specific project. You can keep
the `Developing.md` and `Publishing.md` files as useful referenced, but please set some
proper description in the README.

## Gitpod integration

[Gitpod](https://www.gitpod.io/) container-based development platform will be enabled on your project by default.

Workspace contains:

- **rust**: for builds
- [wasmd](https://github.com/CosmWasm/wasmd): for local node setup and client
- **jq**: shell JSON manipulation tool

Follow [Gitpod Getting Started](https://www.gitpod.io/docs/getting-started) and launch your workspace.

zone connect duck trial leg deposit cabbage quit hundred patient excuse food drive icon tongue girl matrix around magic volcano fat rate thunder area
mantra1yam8f37qctexa3p3np8f094m9qfltl4tjct65d

export CHAIN_ID="mantra-hongbai-1"
export TESTNET_NAME="mantra-hongbai"
export FEE_DENOM="uom"
export STAKE_DENOM="uom"
export BECH32_HRP="wasm"
export WASMD_VERSION="v0.27.0"
export CONFIG_DIR=".mantrachaind"
export BINARY="mantrachaind"

export COSMJS_VERSION="v0.28.1"
export GENESIS_URL="https://<location-to-be-provided>/config/genesis.json"

export RPC="https://rpc.hongbai.mantrachain.io:443"
export FAUCET="https://faucet.hongbai.mantrachain.io"

export NODE=(--node $RPC)
export TXFLAG=($NODE --chain-id $CHAIN_ID --gas-prices 0.25uom --gas auto --gas-adjustment 1.3)

RES=$(mantrachaind tx wasm store target/wasm32-unknown-unknown/release/cw_counter.wasm --from wallet --node https://rpc.hongbai.mantrachain.io:443 --chain-id mantra-hongbai-1 --gas-prices 0.0002uom --gas auto --gas-adjustment 2 -y --output json)

INIT='{"val" : 100}'

mantrachaind tx wasm instantiate $CODE_ID "$INIT" --from wallet --label "name service" --node "https://rpc.hongbai.mantrachain.io:443" --chain-id $CHAIN_ID --gas-prices 0.25uom --gas auto --gas-adjustment 1.3 -y --no-admin

gaus@GausPeerzade:/mnt/d/CosmWasm/cw-counter$ mantrachaind tx wasm instantiate $CODE_ID "$INIT" --from wallet --label "name service" --node "https://rpc.hongbai.mantrachain.io:443" --chain-id $CHAIN_ID --gas-prices 0.25uom --gas auto --gas-adjustment 1.3 -y --no-admin
Enter keyring passphrase (attempt 1/3):
gas estimate: 179335
code: 0
codespace: ""
data: ""
events: []
gas_used: "0"
gas_wanted: "0"
height: "0"
info: ""
logs: []
raw_log: '[]'
timestamp: ""
tx: null
txhash: 26E24D43080067D4DB7FB8F13E918E158EC70D16E1E00E986112DE257F540439

CONTRACT=$(mantrachaind query wasm list-contract-by-code $CODE_ID --node "https://rpc.hongbai.mantrachain.io:443" --output json | jq -r '.contracts[-1]')

mantra1p73558u0g52e96tzna8uhaea0v3mtda7whxqaxq6due44kgvu2aqzjk06h
