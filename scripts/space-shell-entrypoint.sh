#! /bin/bash

apt update && apt install -y curl jq build-essential cmake pkg-config libssl-dev

cd packages/space-shell
cargo install cargo-binstall
cargo binstall dioxus-cli --force

export CARGO_TARGET_DIR=/tmp/cargo-target

make run
