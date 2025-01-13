
#!/bin/bash
set -e

echo $(../target/release/substrate key insert --key-type gran --scheme ed25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD//fir//ed//1)
echo $(../target/release/substrate key insert --key-type babe --scheme sr25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/1)
echo $(../target/release/substrate key insert --key-type imon --scheme sr25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/1)
echo $(../target/release/substrate key insert --key-type auth --scheme sr25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/1)
echo $(../target/release/substrate key insert --key-type mixn --scheme sr25519 --base-path /data  --suri //$SESSION_KEYS_PASSWORD/fir/sr/1)
echo $(../target/release/substrate key insert --key-type beef --scheme ecdsa --base-path /data  --suri //$SESSION_KEYS_PASSWORD//fir//ecdsa//1)