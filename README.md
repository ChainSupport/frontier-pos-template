
This is a general public chain development template based on [polkadot-sdk](https://github.com/paritytech/polkadot-sdk) and [frontier](https://github.com/polkadot-evm/frontier), using the NPOS protocol and compatible with the Ethereum EVM. It aims to make Polkadot technology more developer-friendly and to be used by more public chain teams.
[polkadot-sdk](https://github.com/paritytech/polkadot-sdk) is an excellent public chain development technology, and we thank the ParityTech team for their contributions.

## run develop chain
[start develop](./docs/start-develop.md)

## Visit the [explorer](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer)

![explorer](./docs/images/explorer.jpg)
> This is a developer-friendly expolrer.


## Deploy your scan
[https://github.com/blockscout/blockscout](https://github.com/blockscout/blockscout)
> Note that currently BlockScout does not support displaying transfer records in the balances module, so you should use MetaMask as much as possible for transfers.

## connect MetaMask wallet
```
Network name: Frontier Testnet
Default RPC URL: http://localhost:9944
Chain ID: 42
Currency symbol: UNIT
```

![add network to MetaMask](./docs/images/add_network_to_metamask.jpg)


> To make it easier for users and developers in the Ethereum community to connect to your network node or wallet, you should add your network on [https://chainlist.org/](https://chainlist.org/).

## Develop and deploy your Solidity contract.
- hardhat
[https://hardhat.org/tutorial](https://hardhat.org/tutorial)

- remix
  [https://remix.ethereum.org/#](https://remix.ethereum.org/#)

## More

- [tutorial](./docs/test.md)

## License

This project is licensed under the LICENSE_APACHE2. See the [LICENSE](./LICENSE_APACHE2) file for details.


