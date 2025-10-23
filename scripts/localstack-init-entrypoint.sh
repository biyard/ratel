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

echo 'Creating admin user...'
ADMIN_UUID="00000000-0000-0000-0000-000000000001"
ADMIN_PASSWORD_HASH="d590005c41712ddad6630ca03348fad16ce2fbfb611725116c14631ff02268d8"
TIMESTAMP=$(date +%s%3N)

aws --endpoint-url=http://localstack:4566 dynamodb put-item \
  --table-name ratel-local-main \
  --item '{
    "pk": {"S": "USER#'${ADMIN_UUID}'"},
    "sk": {"S": "USER"},
    "created_at": {"N": "'${TIMESTAMP}'"},
    "updated_at": {"N": "'${TIMESTAMP}'"},
    "display_name": {"S": "Admin User"},
    "email": {"S": "admin@ratel.foundation"},
    "profile_url": {"S": "https://metadata.ratel.foundation/ratel/default-profile.png" },
    "username": {"S": "admin"},
    "term_agreed": {"BOOL": true},
    "informed_agreed": {"BOOL": true},
    "user_type": {"N": "98"},
    "password": {"S": "'${ADMIN_PASSWORD_HASH}'"},
    "theme": {"N": "3"},
    "points": {"N": "0"},
    "followers_count": {"N": "0"},
    "followings_count": {"N": "0"},
    "description": {"S": ""},
    "gsi1_pk": {"S": "EMAIL#PASSWORD#admin@ratel.foundation"},
    "gsi1_sk": {"S": "'${ADMIN_PASSWORD_HASH}'"},
    "gsi2_pk": {"S": "USERNAME#admin"},
    "gsi2_sk": {"S": "TS#'${TIMESTAMP}'"},
    "gsi3_pk": {"S": "EMAIL#admin@ratel.foundation"},
    "gsi3_sk": {"S": "TS#'${TIMESTAMP}'"}
  }'

echo 'Admin user created successfully'
echo 'Login credentials:'
echo '  Email: admin@ratel.foundation'
echo '  Password: admin!234'
echo '  Username: admin'
