#!/bin/bash

export cdir=$(pwd)

# if ../../.build/evm-keys, run `make .build/evm-keys` 
if [ ! -f ../../.build/evm-keys ]; then
    cd ../../
    make .build/evm-keys > /dev/null 2>&1
    cd $cdir
fi

# if jq is not installed, install it
if ! command -v jq &> /dev/null
then
    echo "jq could not be found, install it..."
    exit 1
fi

export ADDR=$(jq ".[0].address" ../../.build/evm-keys.json | tr -d \")
export KEY=$(jq ".[0].private_key" ../../.build/evm-keys.json | tr -d \")

echo "$ADDR $KEY"
