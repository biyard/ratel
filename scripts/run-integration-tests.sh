#!/bin/bash

# Integration tests runner for DynamoDB and SQS functionality
# This script sets up the local AWS infrastructure and runs the tests

set -e

echo "🚀 Starting DynamoDB and SQS integration tests..."

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is running
if ! docker info >/dev/null 2>&1; then
    print_error "Docker is not running. Please start Docker first."
    exit 1
fi

# Set environment variables for testing
export AWS_ENDPOINT_URL_DYNAMODB="http://localhost:4566"
export AWS_ENDPOINT_URL_SQS="http://localhost:4566"
export AWS_ACCESS_KEY_ID="test"
export AWS_SECRET_ACCESS_KEY="test"
export AWS_REGION="us-east-1"
export DUAL_WRITE_ENABLED="true"
export DUAL_WRITE_TABLE_NAME="ratel-test"
export DYNAMODB_TABLE_NAME="ratel-test"

print_status "Environment variables set for LocalStack"

# Start LocalStack services if not already running
print_status "Starting LocalStack services..."
docker-compose up -d localstack dynamodb-init sqs-init

# Wait for LocalStack to be ready
print_status "Waiting for LocalStack to be ready..."
timeout=60
counter=0
while ! curl -s http://localhost:4566/health >/dev/null 2>&1; do
    if [ $counter -ge $timeout ]; then
        print_error "LocalStack failed to start within $timeout seconds"
        exit 1
    fi
    sleep 2
    counter=$((counter + 2))
    echo -n "."
done
echo ""

print_status "LocalStack is ready!"

# Wait a bit more for DynamoDB and SQS initialization to complete
print_status "Waiting for DynamoDB and SQS initialization..."
sleep 10

# Verify DynamoDB table exists
print_status "Verifying DynamoDB table..."
if aws --endpoint-url=http://localhost:4566 dynamodb describe-table --table-name ratel-test >/dev/null 2>&1; then
    print_status "DynamoDB table 'ratel-test' is ready"
else
    print_warning "DynamoDB table 'ratel-test' not found, creating it..."
    # Create table if it doesn't exist
    aws --endpoint-url=http://localhost:4566 dynamodb create-table \
        --cli-input-json file://scripts/dynamodb-schema.json \
        --table-name ratel-test
    sleep 5
fi

# Verify SQS queues exist
print_status "Verifying SQS queues..."
if aws --endpoint-url=http://localhost:4566 sqs get-queue-url --queue-name watermark-queue >/dev/null 2>&1; then
    print_status "SQS queues are ready"
else
    print_warning "SQS queues not found, creating them..."
    aws --endpoint-url=http://localhost:4566 sqs create-queue --queue-name watermark-queue
    aws --endpoint-url=http://localhost:4566 sqs create-queue --queue-name artwork-image-queue
fi

# Run the tests
print_status "Running DynamoDB integration tests..."
cd packages/main-api

# Run DynamoDB tests
if RUST_LOG=debug cargo test tests::dynamo_tests --features integration-tests -- --nocapture; then
    print_status "✅ DynamoDB tests passed!"
else
    print_error "❌ DynamoDB tests failed!"
    TEST_FAILED=1
fi

# Run SQS tests
print_status "Running SQS integration tests..."
if RUST_LOG=debug cargo test tests::sqs_tests --features integration-tests -- --nocapture; then
    print_status "✅ SQS tests passed!"
else
    print_error "❌ SQS tests failed!"
    TEST_FAILED=1
fi

# Run all integration tests together
print_status "Running all integration tests..."
if RUST_LOG=debug cargo test tests --features integration-tests -- --nocapture; then
    print_status "✅ All integration tests passed!"
else
    print_error "❌ Some integration tests failed!"
    TEST_FAILED=1
fi

# Cleanup (optional - comment out if you want to keep the services running)
if [ "${KEEP_SERVICES}" != "true" ]; then
    print_status "Cleaning up services..."
    cd ../../
    docker-compose down localstack dynamodb-init sqs-init
else
    print_status "Services kept running (KEEP_SERVICES=true)"
    print_status "DynamoDB Admin UI: http://localhost:8081"
    print_status "LocalStack Health: http://localhost:4566/health"
fi

if [ "${TEST_FAILED}" = "1" ]; then
    print_error "Some tests failed. Check the output above for details."
    exit 1
else
    print_status "🎉 All tests completed successfully!"
fi