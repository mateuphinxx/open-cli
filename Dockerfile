FROM rust:1.70-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/opencli /usr/local/bin/opencli

RUN useradd -m -s /bin/bash opencli
USER opencli
WORKDIR /home/opencli

ENTRYPOINT ["opencli"]

FROM rust:1.70-slim AS development

WORKDIR /workspace

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch

ENTRYPOINT ["bash"]

FROM builder AS test

WORKDIR /app

COPY . .

RUN cargo test --release

ENTRYPOINT ["cargo", "test"]
