#!/bin/bash

# This script lists the files in the current directory
#ls

mkdir keystore

./target/release/substrate key generate --output-type json --scheme ed25519 | tee keystore/grandpa.json

./target/release/substrate key generate --output-type json --scheme sr25519 | tee keystore/babe.json

./target/release/substrate key generate --output-type json --scheme sr25519 | tee keystore/im_online.json

./target/release/substrate key generate --output-type json --scheme sr25519 | tee keystore/authority_discovery.json

./target/release/substrate key generate --output-type json --scheme sr25519 | tee keystore/mixnet.json

./target/release/substrate key generate --output-type json --scheme ecdsa | tee keystore/beefy.json
