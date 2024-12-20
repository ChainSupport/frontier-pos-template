

## 安装rust环境

[install rust environment](./install-environment.md)

## 克隆项目

```
git clone https://github.com/ChainSupport/frontier-pos-template.git
cd frontier-pos-template
```

## 编译项目

[build local node](./build-node-local.md)
## run测试节点
```
./target/release/substrate --dev --alice
```

## 测试命令

```
 cargo test -p babe-consensus-data-provider -- --nocapture
```

```
cargo test -p ecdsa-keyring -- --nocapture
```

```
cargo test --features testnet -- --nocapture

```

```
cargo test --features mainnet -- --nocapture

```

## 覆盖率报告

## 编译docker

[build node docker](./build-node-docker.md)

## 跑docker

```
docker-compose up
```

## NPOS截图
访问explorer
![explorer](./images/explorer.jpg)


## 连接小狐狸钱包

> 这里假设你已经创建钱包地址，如果你还没有创建钱包，请先在小狐狸钱包创建你的钱包。

1. 添加本地网络到小狐狸钱包
    ```
    Network name: Frontier Testnet
    Default RPC URL: http://localhost:9944
    Chain ID: 42
    Currency symbol: UNIT
    ```
    ![add network](./images/add_network_to_metamask.jpg)

2. 连接你的网络并且复制您的钱包地址

3. 在explorer中使用alice给你的钱包地址转账。小狐狸钱包显示余额。

## 部署你的scan

1. 部署

2. 在小狐狸钱包给任意地址转账，会发现scan上有这笔交易记录


## redmix部署EVM智能合约

1. 连接redmix
2. 选择节点
3. 编译和部署Lock智能合约

