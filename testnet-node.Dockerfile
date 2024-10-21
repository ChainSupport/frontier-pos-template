# build stage: where we create binary
FROM rust:1.74 AS builder

RUN apt update && apt install -y make clang pkg-config libssl-dev protobuf-compiler build-essential git curl llvm make
RUN rustup default stable && \
    rustup component add rust-src && \
    rustup target add wasm32-unknown-unknown

WORKDIR /substrate
COPY . /substrate
RUN cargo build --release --features testnet

# This is the 2nd stage: a very small image where we copy the substrate binary."
FROM docker.io/library/ubuntu:22.04
LABEL description="Multistage Docker image for frontier-pos-template: a development template based on Polkadot's NPOS consensus, compatible with EVM" \
	io.parity.image.type="builder" \
	io.parity.image.authors="chainsupport" \
	io.parity.image.vendor="ChainSupport" 

COPY --from=builder /substrate/target/release/substrate /usr/local/bin
COPY --from=builder /substrate/scripts/validator_node_init.sh /usr/local/bin
COPY --from=builder /substrate/scripts/normal_node_init.sh /usr/local/bin

ENV BASE_PATH=/data
ENV SESSION_KEYS_PASSWORD=root
ENV SESSION_KEYS_INDEX=0

RUN useradd -m -u 1000 -U -s /bin/base -d /substrate substrate && \
	mkdir -p ${BASE_PATH} /substrate/.local/share/substrate && \
	chown -R substrate:substrate /data && \
	ln -s ${BASE_PATH} /substrate/.local/share/substrate && \
# Sanity checks
	ldd /usr/local/bin/substrate && \
# # unclutter and minimize the attack surface
# 	rm -rf /usr/bin /usr/sbin && \
    chmod 777 /usr/local/bin/validator_node_init.sh && \
	chmod 777 /usr/local/bin/normal_node_init.sh && \
	/usr/local/bin/substrate --version

# RUN /usr/local/bin/substrate --version 
USER substrate
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]
ENTRYPOINT ["/usr/local/bin/substrate", "--chain", "staging", "--database", "auto", "--base-path", "/data" ]
CMD [ "--help" ]
