#!/bin/bash
set -e

TARGET=aarch64-unknown-linux-musl

rustup target add ${TARGET}
rustup toolchain install nightly --target ${TARGET}
rustup component add rust-src --toolchain nightly --target ${TARGET}

cargo-zigbuild +nightly build -r --target ${TARGET} -vv