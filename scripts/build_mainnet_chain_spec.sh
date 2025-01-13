#!/bin/bash
cargo build --release --features mainnet
../target/release/substrate build-spec --raw  --base-path db --chain local_mainnet --disable-default-bootnode > mainnet-chain-spec.json