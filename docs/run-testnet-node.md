
## **1. local**

### install
[install environment](./install-environment.md)

### build

```
cargo build --release --features testnet
```

### run

```
./substrate  --database auto
```

### **2. docker**
```
docker run -id <your dockerhub account>/substrate:latest ""
```
> my command `docker run -id chainsupport/substarte-testnet-node:latest ""`