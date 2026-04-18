#! /bin/bash
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export COMMIT=local
export ECR=ratel/app-shell
export AWS_REGION=ap-northeast-2
export AWS_DEFAULT_REGION=ap-northeast-2
export DYNAMO_ENDPOINT=http://localstack:4566
export ANDROID_EMULATOR_ID=""

# Build App-shell
cd app/ratel
make build-testing
make docker


# Run testings infra
cd ../..
make testing

# Run Playwright tests
cd playwright
make test
