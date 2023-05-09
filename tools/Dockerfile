FROM --platform=x86_64 debian:bookworm AS firmware-builder

# FIRMWARE_GIT should be a git repo with the firmware source code
ARG FIRMWARE_GIT=https://github.com/flipperdevices/flipperzero-firmware.git
ARG BRANCH=0.82.3

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    ca-certificates \
    git \
    curl \
    clang \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN git clone -b $BRANCH --recursive $FIRMWARE_GIT firmware

WORKDIR /app/firmware

RUN ./fbt

####################
# bindgen
FROM --platform=x86_64 rust:bookworm AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    ca-certificates \
    libudev-dev \
    libclang-dev \
    clang \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt

WORKDIR /app

COPY --from=firmware-builder /app/firmware /app/firmware

WORKDIR /app/flipperzero-rs/

COPY . .

WORKDIR /app/flipperzero-rs/tools

RUN cargo run --bin generate-bindings /app/firmware/build/f7-firmware-D/sdk_headers/

RUN cp bindings.rs ../crates/sys/src

RUN rustfmt --config newline_style=unix ../crates/sys/src/bindings.rs

WORKDIR /app/flipperzero-rs/crates

# test it builds
RUN cargo build --release

####################
ENTRYPOINT [ "cat", "sys/src/bindings.rs" ]