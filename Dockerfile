FROM rust:1.89.0-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy

COPY Cargo.toml ./
COPY src ./src

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo build --release

FROM debian:trixie-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/opencli /usr/local/bin/opencli

RUN useradd -m -s /bin/bash opencli && \
    mkdir -p /home/opencli/.config/opencli && \
    chown -R opencli:opencli /home/opencli
USER opencli
WORKDIR /home/opencli

ENTRYPOINT ["opencli"]

FROM rust:1.89.0-slim AS development

WORKDIR /workspace

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo install cargo-watch

ENTRYPOINT ["bash"]

FROM builder AS test

WORKDIR /app

COPY . .

RUN cargo test --release
