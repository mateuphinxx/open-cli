FROM rust:1.89.0-alpine AS base

WORKDIR /build

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    ca-certificates \
    git

RUN rustup component add rustfmt clippy && \
    cargo --version && \
    rustc --version && \
    rustfmt --version && \
    clippy-driver --version

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse \
    CARGO_INCREMENTAL=0 \
    CARGO_NET_RETRY=10 \
    RUSTUP_MAX_RETRIES=10 \
    RUSTFLAGS="-C target-feature=-crt-static"

FROM base AS planner

RUN cargo install cargo-chef

COPY Cargo.toml Cargo.lock ./

RUN cargo chef prepare --recipe-path recipe.json

FROM base AS dependencies

ENV CARGO_TARGET_DIR=/tmp/target

RUN cargo install cargo-chef

COPY --from=planner /build/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

FROM base AS builder

ENV CARGO_TARGET_DIR=/tmp/target

COPY Cargo.toml Cargo.lock ./
COPY src ./src

COPY --from=dependencies /tmp/target /tmp/target
COPY --from=dependencies /usr/local/cargo /usr/local/cargo

RUN cargo build --release && \
    strip /tmp/target/release/opencli && \
    rm -rf /tmp/target/release/deps /tmp/target/release/build /tmp/target/release/*.d

FROM alpine:3.19 AS runtime

RUN apk add --no-cache \
    ca-certificates \
    libgcc \
    libc6-compat \
    libstdc++ \
    curl \
    git \
    bash

COPY --from=builder /tmp/target/release/opencli /usr/local/bin/opencli

RUN opencli --version

RUN addgroup -S opencli && \
    adduser -S opencli -G opencli -h /home/opencli -s /bin/bash && \
    mkdir -p /home/opencli/.config/opencli /workspace && \
    chown -R opencli:opencli /home/opencli /workspace

USER opencli
WORKDIR /workspace

ENV HOME=/home/opencli \
    USER=opencli \
    RUST_LOG=info

LABEL org.opencontainers.image.title="OpenCLI" \
      org.opencontainers.image.description="CLI tool for open.mp server management and Pawn project building" \
      org.opencontainers.image.authors="mateuphinxx" \
      org.opencontainers.image.source="https://github.com/mateuphinxx/open-cli" \
      org.opencontainers.image.licenses="MIT"

ENTRYPOINT ["opencli"]
CMD ["--help"]

FROM base AS development

WORKDIR /workspace

RUN apk add --no-cache \
    git \
    curl \
    vim \
    bash

RUN cargo install cargo-watch cargo-expand cargo-edit

ENV RUST_LOG=debug \
    RUST_BACKTRACE=full

VOLUME ["/workspace", "/usr/local/cargo/registry"]

ENTRYPOINT ["/bin/bash"]

FROM base AS lint

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo fmt --check && \
    cargo clippy --all-targets --all-features -- -D warnings && \
    cargo check --all-targets --all-features

FROM base AS test

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

ENV RUST_BACKTRACE=1 \
    RUST_LOG=debug

RUN cargo test --release --verbose -- --nocapture

FROM runtime AS production

ENV RUST_LOG=warn

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD opencli --version || exit 1

USER opencli
WORKDIR /workspace
