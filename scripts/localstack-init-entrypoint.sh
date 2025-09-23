#!/bin/bash

echo "start"
yum update
yum install -y awscli

echo 'Waiting for LocalStack to be ready...'
until aws dynamodb --endpoint-url=${LOCALSTACK_ENDPOINT}  list-tables >/dev/null 2>&1; do
    sleep 2
done
echo 'Creating ratel-local table with GSIs...'
aws --endpoint-url=${LOCALSTACK_ENDPOINT} dynamodb create-table --cli-input-json file:///scripts/dynamodb-schema.json
echo 'ratel-local-main table and GSIs created successfully'
aws --endpoint-url=${LOCALSTACK_ENDPOINT} dynamodb create-table --cli-input-json file:///scripts/dynamodb-session-schema.json
echo 'ratel-local-session table and GSIs created successfully'

echo 'Waiting for LocalStack to be ready...'
until aws --endpoint-url=${LOCALSTACK_ENDPOINT} sqs list-queues >/dev/null 2>&1; do
    sleep 2
done &&
echo 'Creating SQS queues...'
aws --endpoint-url=${LOCALSTACK_ENDPOINT} sqs create-queue --queue-name watermark-queue
aws --endpoint-url=${LOCALSTACK_ENDPOINT} sqs create-queue --queue-name artwork-image-queue
echo 'SQS queues created successfully'
