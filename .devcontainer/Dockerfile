FROM rust:latest as build

# Install zsh
RUN sh -c "$(wget -O- https://github.com/deluan/zsh-in-docker/releases/download/v1.1.5/zsh-in-docker.sh)" -- -t agnoster

# Make zsh default
RUN chsh -s /bin/zsh

# Install wasm32-unknown-unknown
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN cargo install trunk

# Install rustfmt
RUN rustup component add rustfmt

# Install bacon
RUN cargo install --locked bacon

# Install clippy
RUN rustup component add clippy

# Install watch
RUN cargo install cargo-watch --locked

# Additional components required to compute LLVM code coverage
RUN rustup component add llvm-tools-preview
RUN cargo install cargo-llvm-cov

# Install cargo audit
RUN cargo install cargo-audit

# Install bevy dependencies
RUN apt update
RUN apt install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libxcursor1 mesa-vulkan-drivers  libxi6