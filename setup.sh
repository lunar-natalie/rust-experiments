#!/bin/sh
rustup target add wasm32-unknown-unknown && \
rustup update && \
rustup component add rustfmt --toolchain nightly && \
cargo install cargo-binstall && \
cargo binstall trunk leptosfmt