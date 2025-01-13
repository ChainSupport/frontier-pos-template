#!/bin/bash
set -e

echo $(/usr/local/bin/substrate key generate-node-key --base-path /data)
echo $(/usr/local/bin/substrate key insert --key-type gran --scheme ed25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD//fir//ed//$INDEX)
echo $(/usr/local/bin/substrate key insert --key-type babe --scheme sr25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX)
echo $(/usr/local/bin/substrate key insert --key-type imon --scheme sr25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX)
echo $(/usr/local/bin/substrate key insert --key-type auth --scheme sr25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX)
echo $(/usr/local/bin/substrate key insert --key-type mixn --scheme sr25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$INDEX)
echo $(/usr/local/bin/substrate key insert --key-type beef --scheme ecdsa --base-path /data  --suri //$SESSION_KEYS_PASSWORD//fir//ecdsa//$INDEX)

echo "Validator's node has been initialized successfully."
