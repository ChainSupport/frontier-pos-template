#!/bin/bash

# after generate node key
./target/release/substarte build-spec --raw  --base-path db --chain dev --disable-default-bootnode > chain-spec.json