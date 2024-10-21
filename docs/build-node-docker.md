## first

```
docker buildx create --use
```
## build testnet

```
docker buildx build --platform linux/amd64,linux/arm64 -t <your dockerhub account>/substrate-testnet-node:1.0.0 -t <your dockerhub account>/substrate-testnet-node:latest --push -f testnet-node.Dockerfile .
```

> my command `docker buildx build --platform linux/amd64,linux/arm64 -t chainsupport/substrate-testnet-node:1.0.0 -t chainsupport/substrate-testnet-node:latest --push -f testnet-node.Dockerfile .`

## build mainnet
```
docker buildx build --platform linux/amd64,linux/arm64 -t <your dockerhub account>/substrate-mainnet-node:1.0.0 -t <your dockerhub account>/substrate-mainnet-node:latest  --push -f mainnet-node.Dockerfile .

```
> my command `docker buildx build --platform linux/amd64,linux/arm64 -t chainsupport/substrate-mainnet-node:1.0.0 -t chainsupport/substrate-mainnet-node:latest  --push -f mainnet-node.Dockerfile .`