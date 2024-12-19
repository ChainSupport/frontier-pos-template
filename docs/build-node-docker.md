## first

```
docker buildx create --use
```
## build testnet

```
docker buildx build --platform linux/amd64,linux/arm64 -t substrate-testnet-node:1.0.0 -t substrate-testnet-node:latest --load -f ./docker/testnet-node.Dockerfile .
```

> my command `docker buildx build --platform linux/amd64,linux/arm64 -t chainsupport/substrate-testnet-node:1.0.0 -t chainsupport/substrate-testnet-node:latest --push -f ./docker/testnet-node.Dockerfile .`

## build mainnet
```
docker buildx build --platform linux/amd64,linux/arm64 -t substrate-mainnet-node:1.0.0 -t substrate-mainnet-node:latest  --load -f ./docker/mainnet-node.Dockerfile .

```
> my command `docker buildx build --platform linux/amd64,linux/arm64 -t chainsupport/substrate-mainnet-node:1.0.0 -t chainsupport/substrate-mainnet-node:latest  --push -f ./docker/mainnet-node.Dockerfile .`