#!/bin/bash

echo "start"
yum update
yum install -y awscli

echo 'Waiting for LocalStack to be ready...'
export AWS_ENDPOINT_URL="${LOCALSTACK_ENDPOINT:-http://localhost:4566}"
echo "Using AWS endpoint URL: $AWS_ENDPOINT_URL"
wait_for_localstack() {
  echo "Waiting for LocalStack to be ready..."
  until curl -s "${AWS_ENDPOINT_URL}/_localstack/health" | jq -e '.services.dynamodb == "running" and .services.sqs == "running"' >/dev/null 2>&1; do
    sleep 2
  done
  echo "LocalStack is ready."
}
wait_for_localstack

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
