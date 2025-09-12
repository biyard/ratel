#! /bin/bash

cd packages/fetcher
cargo install cargo-binstall
cargo binstall --no-confirm cargo-watch

export DATABASE_TYPE=postgres

make run
