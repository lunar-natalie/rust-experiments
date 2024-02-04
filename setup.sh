#!/bin/sh
BINSTALL_FLAGS=
if [ "$1" = "--no-confirm" ]; then
    BINSTALL_FLAGS=--no-confirm
fi
rustup target add wasm32-unknown-unknown && \
rustup update && \
rustup component add rustfmt --toolchain nightly && \
cargo install cargo-binstall && \
cargo binstall $BINSTALL_FLAGS trunk leptosfmt