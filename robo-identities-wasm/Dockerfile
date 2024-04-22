FROM debian:testing-slim

# install common packages
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    ca-certificates curl file \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev && \
    rm -rf /var/lib/apt/lists/*

# install Rust toolchain
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain nightly -y

ENV PATH="/root/.cargo/bin:${PATH}"

# install depdendencies
RUN cargo install wasm-pack

COPY . /app

WORKDIR /app/robo-identities-wasm
