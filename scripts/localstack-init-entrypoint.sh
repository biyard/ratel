#!/bin/bash

echo "start"
yum update
yum install -y awscli

echo 'Waiting for LocalStack to be ready...'
until aws dynamodb --endpoint-url=http://localstack:4566  list-tables >/dev/null 2>&1; do
    sleep 2
done
echo 'Creating ratel-local table with GSIs...'
aws --endpoint-url=http://localstack:4566 dynamodb create-table --cli-input-json file:///scripts/dynamodb-schema.json
echo 'ratel-local-main table and GSIs created successfully'

echo 'Waiting for LocalStack to be ready...'
until aws --endpoint-url=http://localstack:4566 sqs list-queues >/dev/null 2>&1; do
    sleep 2
done &&
echo 'Creating SQS queues...'
aws --endpoint-url=http://localstack:4566 sqs create-queue --queue-name watermark-queue 
aws --endpoint-url=http://localstack:4566 sqs create-queue --queue-name artwork-image-queue
echo 'SQS queues created successfully'
