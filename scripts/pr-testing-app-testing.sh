#! /bin/bash
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export COMMIT=local
export ECR=ratel/app-shell
export AWS_REGION=ap-northeast-2
export AWS_DEFAULT_REGION=ap-northeast-2
export DYNAMO_ENDPOINT=http://localstack:4566
export ANDROID_EMULATOR_ID=""
export DYNAMO_TABLE_PREFIX=ratel-local
export QDRANT_PREFIX=ratel-local
export CI=true
export CARGO_TARGET_DIR=$(pwd)/target

make infra

# Run App testings
cd app/ratel
make test
