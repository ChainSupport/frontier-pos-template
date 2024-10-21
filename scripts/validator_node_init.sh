#!/bin/bash
set -e

echo $(/usr/local/bin/substrate key generate-node-key --base-path $BASE_PATH)
echo $(/usr/local/bin/substrate key insert --key-type gran --scheme ed25519 --base-path $BASE_PATH  --suri //$SESSION_KEYS_PASSWORD//fir//ed//$SESSION_KEYS_INDEX)
echo $(/usr/local/bin/substrate key insert --key-type babe --scheme sr25519 --base-path $BASE_PATH  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$SESSION_KEYS_INDEX)
echo $(/usr/local/bin/substrate key insert --key-type imon --scheme sr25519 --base-path $BASE_PATH  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$SESSION_KEYS_INDEX)
echo $(/usr/local/bin/substrate key insert --key-type auth --scheme sr25519 --base-path $BASE_PATH  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$SESSION_KEYS_INDEX)
echo $(/usr/local/bin/substrate key insert --key-type mixn --scheme sr25519 --base-path $BASE_PATH  --suri //$SESSION_KEYS_PASSWORD/fir/sr/$SESSION_KEYS_INDEX)
echo $(/usr/local/bin/substrate key insert --key-type beef --scheme ecdsa --base-path $BASE_PATH  --suri //$SESSION_KEYS_PASSWORD//fir//ecdsa//$SESSION_KEYS_INDEX)

echo "初始配置设置成功！"

# sleep 1h
