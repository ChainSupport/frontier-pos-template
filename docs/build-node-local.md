
## install environment
[https://docs.substrate.io/install/](https://docs.substrate.io/install/)

```
rustup component add rust-src
```
## build testnet
```
cargo build --release --features testnet
```

## build mainnet
```
cargo build --release --features mainnet
```