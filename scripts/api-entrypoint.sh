#! /bin/bash

## TODO: Setting up KAIA keys
apt update && apt install -y curl jq

export ADDR=$(jq ".[0].address" .build/evm-keys.json | tr -d \")
export KEY=$(jq ".[0].private_key" .build/evm-keys.json | tr -d \")

## TODO: Setting up DID keys(P256 and BBS_BLS keys)


export POOL_SIZE="1"
export DATABASE_TYPE="postgres"
export OPENAPI_KEY=dummy
export BASE_DOMAIN=dev.ratel.foundation
export AUTH_DOMAIN=dev.ratel.foundation
export HASURA_GRAPHQL_AUTH_HOOK=https://api.dev.ratel.foundation/v1/auth/hasura
export MAIN_API_ENDPOINT="https://api.dev.ratel.foundation"

export JWT_SECRET_KEY=ratel-jwt-secret-key

## TODO: Implement and use mock PUT API for slack notifications

export SLACK_CHANNEL_BILL="dummy"
export SLACK_CHANNEL_SPONSOR="dummy"
export SLACK_CHANNEL_ABUSING="dummy"
export SLACK_CHANNEL_MONITOR="dummy"

export KAIA_ENDPOINT=https://public-en-kairos.node.kaia.io
export KAIA_OWNER_ADDR=$ADDR
export KAIA_OWNER_KEY=$KEY
export KAIA_FEEPAYER_ADDR=$ADDR
export KAIA_FEEPAYER_KEY=$KEY

export TELEGRAM_TOKEN=dummy

export BBS_BLS_X=dummy
export BBS_BLS_Y=dummy
export BBS_BLS_D=dummy
export BBS_BLS_CRV=BLS12381G2

export P256_X=dummy
export P256_Y=dummy
export P256_D=dummy
export P256_CRV=P-256

cd packages/main-api
cargo install cargo-binstall
cargo binstall --no-confirm cargo-watch

make run
