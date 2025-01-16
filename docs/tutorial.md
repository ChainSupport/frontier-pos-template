

## install rust environment

[install rust environment](./install-environment.md)

## clone project

```
git clone https://github.com/ChainSupport/frontier-pos-template.git
cd frontier-pos-template
```

## build

[build local node](./build-node-local.md)
## run testnet node
```
cargo build --release --features testnet
./target/release/substrate --dev --alice
```

## Unit tests

```
cargo test -p babe-consensus-data-provider -- --nocapture
cargo test -p ecdsa-keyring -- --nocapture
```

## Generate test coverage report

```
cargo clean
cargo tarpaulin --out html --run-types Tests -p babe-consensus-data-provider
```

```
cargo clean
cargo tarpaulin --out html --run-types Tests -p ecdsa-keyring
```

## Cargo clippy

```
cargo clippy --features testnet
cargo clippy --features mainnet
```

## docker build

[build node docker](./build-node-docker.md)

## run by docker

```
docker-compose up
```

## Vist [explorer](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)

![explorer](./images/explorer.jpg)

> Here, we can see that two validators are randomly producing blocks, fully compatible with NPOS (Nominated Proof-of-Stake).

## Connect MetaMask Wallet

> The following steps assume that you have already created a wallet address. If you haven't created a wallet yet, please create one in MetaMask first.

1. Add your network to MetaMask.
    ```
    Network name: Frontier Testnet
    Default RPC URL: http://localhost:9944
    Chain ID: 42
    Currency symbol: UNIT
    ```
    <div align="center">
    <img src="./images/add_network_to_metamask.jpg" alt="add_network_to_metamask">
    </div>

    <!-- ![add network](./images/add_network_to_metamask.jpg) -->

2. Connect to your network and copy your wallet address.
    1. select network
    <div align="center">
    <img src="./images/before_select.jpg" alt="before_select">
    </div>
    2. copy your address
        <div align="center">
        <img src="./images/copy%20address.jpg" alt="copy your address">
        </div>

3. Use `CHARLETH` to transfer UNIT to your wallet address in the [explorer](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/accounts). Your MetaMask wallet will display the balance.
    1. transfer 
        <div align="center">
        <img src="./images/charleth%20transfer.jpg" alt="charleth transfer">
        </div>
    2. balance
        <div align="center">
        <img src="./images/balance.jpg" alt="balance">
        </div>

## Run your scan (Blockscout)

> Transactions in the EVM can be viewed on this scan.
1. deploy
    ```
    git clone https://github.com/ChainSupport/blockscout.git
    cd blockscout/docker-compose
    ```

    ```
    docker-compose up --build
    ```

2. When you send token to any address from MetaMask, you will find the transfer record on the [scan](http://localhost/).
    1. Send token by MetaMask
        <div align="center">
        <img src="./images/transfer.jpg" alt="transfer">
        </div>
    2. transactions  
        <div align="center">
        <img src="./images/scan.jpg" alt="transaction">
        </div>

> Note: Currently, Blockscout does not support displaying native transfer records, meaning transactions made through the `Balances` pallet are not shown here. (We plan to provide support for this in the future.)
## Deploy a smart contract

### Option 1: Remix
1. Visit [Remix](https://remix.ethereum.org/#)
   ![remix](./images/remix.jpg)
    
2. Select a network
    ![select network for remix](./images/select%20network%20for%20remix.jpg)
3. Compile and deploy the smart contract
    1. Compoile
        ![compile 2_Owner](./images/compile_owner.jpg)
    2. deploy
        ![deploy 2_Owner](./images/deploy.jpg)

### Option 2: hardhat
[https://hardhat.org/tutorial](https://hardhat.org/tutorial)