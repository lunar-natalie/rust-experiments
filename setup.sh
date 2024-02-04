#!/bin/sh
rustup update && \
rustup target add wasm32-unknown-unknown && \
rustup component add rustfmt --toolchain nightly && \
cargo install cargo-binstall && \
cargo binstall trunk leptosfmt