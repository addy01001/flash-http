FROM ubuntu:16.04

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Update new packages
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Verify Rust installation
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install tauri-cli --version "^2.0.0-beta"
RUN rustup default stable
RUN apt -y install pkg-config
ENV PKG_CONFIG_PATH=/usr/share/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig

