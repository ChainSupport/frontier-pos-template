#!/bin/bash
cargo build --release --features testnet
../target/release/substrate build-spec --raw  --base-path db --chain local_testnet --disable-default-bootnode > testnet-chain-spec.json