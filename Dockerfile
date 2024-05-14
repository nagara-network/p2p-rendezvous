FROM ghcr.io/bamboolabs-foundation/builder-rust-llvm:latest AS builder

ARG RUST_BUILD_ARG="-C target-cpu=x86-64-v4"

WORKDIR /builder

COPY . .

RUN RUSTFLAGS="${RUST_BUILD_ARG}" cargo build --release

FROM ghcr.io/bamboolabs-foundation/base-ubuntu2204:latest

LABEL org.opencontainers.image.authors "nagara Network Developers <dev@nagara.network>"
LABEL org.opencontainers.image.source "https://github.com/nagara-network/p2p-rendezvous"
LABEL org.opencontainers.image.description "nagara Network - P2P Rendezvous Server"

WORKDIR /app

COPY --from=builder /builder/target/release/nagara-p2p-rendezvous nagara-p2p-rendezvous

ENTRYPOINT [ "/app/nagara-p2p-rendezvous" ]
