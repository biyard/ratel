#! /bin/bash
apt update && apt install -y curl jq build-essential cmake pkg-config libssl-dev

cd packages/fetcher
cargo install cargo-binstall
cargo binstall --no-confirm cargo-watch

export DATABASE_TYPE=postgres

make run
