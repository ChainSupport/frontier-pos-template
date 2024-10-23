
## **1. local**

### install
[install environment](./install-environment.md)

### build

```
cargo build --release --features testnet
```

### run

```
./substrate  --database auto --alice
```

### **2. docker**
```
docker run -id <your dockerhub account>/substrate:latest "--alice"
```
> my command `docker run -id chainsupport/substarte-testnet-node:latest "--alice"`