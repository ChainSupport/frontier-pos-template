
## **1. local**

### install
[install environment](./install-environment.md)

### build

```
cargo build --release --features mainnet
```

### run

```
./substrate --database auto
```

### **2. docker**
```
docker run -id <your diockerhub account>/substarte:latest ""
```

> my command `docker run -id chainsupport/substarte-mainnet-node:latest ""`