#!/bin/bash

echo "start"

echo 'Waiting for LocalStack to be ready...'
until awslocal dynamodb list-tables >/dev/null 2>&1; do
    sleep 2
done
echo 'Creating ratel-local table with GSIs...'
awslocal dynamodb create-table --cli-input-json file://./scripts/dynamodb-schema.json
echo 'ratel-local-main table and GSIs created successfully'
awslocal dynamodb create-table --cli-input-json file://./scripts/dynamodb-session-schema.json
echo 'ratel-local-session table and GSIs created successfully'

