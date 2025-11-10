#!/bin/bash

echo "start"
yum update
yum install -y awscli

local ENDPOINT=http://localstack:4566

echo 'Waiting for LocalStack to be ready...'
until aws dynamodb --endpoint-url=$ENDPOINT  list-tables >/dev/null 2>&1; do
    sleep 2
done
echo 'Creating ratel-local table with GSIs...'
aws --endpoint-url=$ENDPOINT dynamodb create-table --cli-input-json file:///scripts/dynamodb-schema.json
echo 'ratel-local-main table and GSIs created successfully'

echo 'Waiting for LocalStack to be ready...'
until aws --endpoint-url=$ENDPOINT sqs list-queues >/dev/null 2>&1; do
    sleep 2
done &&
echo 'Creating SQS queues...'
aws --endpoint-url=$ENDPOINT sqs create-queue --queue-name watermark-queue
aws --endpoint-url=$ENDPOINT sqs create-queue --queue-name artwork-image-queue
echo 'SQS queues created successfully'

echo 'Creating admin user...'
ADMIN_UUID="00000000-0000-0000-0000-000000000001"
ADMIN_PASSWORD_HASH="d590005c41712ddad6630ca03348fad16ce2fbfb611725116c14631ff02268d8"
TIMESTAMP=$(date +%s%3N)

aws --endpoint-url=$ENDPOINT dynamodb put-item \
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

for idx in {2..10}; do
  aws --endpoint-url=$ENDPOINT dynamodb put-item \
    --table-name ratel-local-main \
    --item '{
      "pk": {"S": "USER#00000000-0000-0000-0000-00000000000'${idx}'"},
      "sk": {"S": "USER"},
      "created_at": {"N": "'${TIMESTAMP}'"},
      "updated_at": {"N": "'${TIMESTAMP}'"},
      "display_name": {"S": "User '${idx}'"},
      "email": {"S": "user'${idx}'@ratel.foundation"},
      "profile_url": {"S": "https://metadata.ratel.foundation/ratel/default-profile.png" },
      "username": {"S": "user'${idx}'"},
      "term_agreed": {"BOOL": true},
      "informed_agreed": {"BOOL": true},
      "user_type": {"N": "1"},
      "password": {"S": "'${ADMIN_PASSWORD_HASH}'"},
      "theme": {"N": "3"},
      "points": {"N": "0"},
      "followers_count": {"N": "0"},
      "followings_count": {"N": "0"},
      "description": {"S": ""},
      "gsi1_pk": {"S": "EMAIL#PASSWORD#hi+'${idx}'@ratel.foundation"},
      "gsi1_sk": {"S": "'${ADMIN_PASSWORD_HASH}'"},
      "gsi2_pk": {"S": "USERNAME#user'${idx}'"},
      "gsi2_sk": {"S": "TS#'${TIMESTAMP}'"},
      "gsi3_pk": {"S": "EMAIL#hi+'${idx}'@ratel.foundation"},
      "gsi3_sk": {"S": "TS#'${TIMESTAMP}'"}
    }'
done

# Create a Team (hiteam)
aws --endpoint-url=$ENDPOINT dynamodb put-item \
    --table-name ratel-local-main \
    --item '{
            "gsi6_pk": {"S": "TEAM"},
            "profile_url": {"S": ""},
            "created_at": {"N": "'${TIMESTAMP}'"},
            "description": {"S": ""},
            "display_name": {"S": "hiteam"},
            "followers": {"N": "0"},
            "updated_at": {"N": "'${TIMESTAMP}'"},
            "followings": {"N": "0"},
            "gsi1_pk": {"S": "TEAM_NAME_IDX#TEAM"},
            "sk": {"S": "TEAM"},
            "gsi2_pk": {"S": "USERNAME#hiteam"},
            "pk": {"S": "TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "gsi2_sk": {"S": "TS#'${TIMESTAMP}'"},
            "gsi1_sk": {"S": "hiteam"},
            "gsi6_sk": {"S": "0"},
            "username": {"S": "hiteam"}
        }'

# Create a TEAM group (Admin)
aws --endpoint-url=$ENDPOINT dynamodb put-item \
    --table-name ratel-local-main \
    --item '{
            "permissions": {"N": "-4611686018420032497"},
            "members": {"N": "2"},
            "gsi1_pk": {"S": "TEAM_GROUP_PK#TEAM_GROUP#0e55a3b3-fb35-45b2-8a57-1440dda643ef"},
            "name": {"S": "Admin"},
            "sk": {"S": "TEAM_GROUP#0e55a3b3-fb35-45b2-8a57-1440dda643ef"},
            "created_at": {"N": "'${TIMESTAMP}'"},
            "description": {"S": "Administrators group with all permissions"},
            "pk": {"S": "TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "gsi1_sk": {"S": "'${TIMESTAMP}'"}
        }'

# Setting Owner(hi+2@ratel.foundation)
aws --endpoint-url=$ENDPOINT dynamodb put-item \
    --table-name ratel-local-main \
    --item '{
            "profile_url": {"S": "https://metadata.ratel.foundation/ratel/default-profile.png"},
            "user_pk": {"S": "USER#00000000-0000-0000-0000-000000000002"},
            "gsi1_pk": {"S": "USER_PK#USER#00000000-0000-0000-0000-000000000002"},
            "sk": {"S": "TEAM_OWNER"},
            "pk": {"S": "TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "display_name": {"S": "User 2"},
            "gsi1_sk": {"S": "TEAM_OWNER"},
            "username": {"S": "user2"}
        }'

# Add users to team (hi+2@ratel.foundation, hi+3@ratel.foundation)
## User team
aws --endpoint-url=$ENDPOINT dynamodb put-item \
    --table-name ratel-local-main \
    --item '{
            "last_used_at": {"N": "'${TIMESTAMP}'"},
            "profile_url": {"S": "https://metadata.ratel.foundation/ratel/default-profile.png"},
            "gsi1_pk": {"S": "TEAM_PK#USER_TEAM#TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "sk": {"S": "USER_TEAM#TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "pk": {"S": "USER#00000000-0000-0000-0000-000000000002"},
            "display_name": {"S": "hiteam"},
            "gsi1_sk": {"S": "'${TIMESTAMP}'"},
            "username": {"S": "hiteam"}
        }'
## User group
aws --endpoint-url=$ENDPOINT dynamodb put-item \
    --table-name ratel-local-main \
    --item '{
            "team_group_permissions": {"N": "-4611686018420032497"},
            "gsi1_pk": {"S": "TEAM_GROUP_PK#USER_TEAM_GROUP#TEAM_GROUP#0e55a3b3-fb35-45b2-8a57-1440dda643ef"},
            "sk": {"S": "USER_TEAM_GROUP#TEAM_GROUP#0e55a3b3-fb35-45b2-8a57-1440dda643ef"},
            "gsi2_pk": {"S": "USER_TEAM_GROUP#TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "pk": {"S": "USER#00000000-0000-0000-0000-000000000002"},
            "team_pk": {"S": "TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "gsi2_sk": {"S": "USER#00000000-0000-0000-0000-000000000002"},
            "gsi1_sk": {"S": "-4611686018420032497"}
        }'

## User team
aws --endpoint-url=$ENDPOINT dynamodb put-item \
    --table-name ratel-local-main \
    --item '{
            "last_used_at": {"N": "'${TIMESTAMP}'"},
            "profile_url": {"S": "https://metadata.ratel.foundation/ratel/default-profile.png"},
            "gsi1_pk": {"S": "TEAM_PK#USER_TEAM#TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "sk": {"S": "USER_TEAM#TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "pk": {"S": "USER#00000000-0000-0000-0000-000000000003"},
            "display_name": {"S": "hiteam"},
            "gsi1_sk": {"S": "'${TIMESTAMP}'"},
            "username": {"S": "hiteam"}
        }'
## User group
aws --endpoint-url=$ENDPOINT dynamodb put-item \
    --table-name ratel-local-main \
    --item '{
            "team_group_permissions": {"N": "-4611686018420032497"},
            "gsi1_pk": {"S": "TEAM_GROUP_PK#USER_TEAM_GROUP#TEAM_GROUP#0e55a3b3-fb35-45b2-8a57-1440dda643ef"},
            "sk": {"S": "USER_TEAM_GROUP#TEAM_GROUP#0e55a3b3-fb35-45b2-8a57-1440dda643ef"},
            "gsi2_pk": {"S": "USER_TEAM_GROUP#TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "pk": {"S": "USER#00000000-0000-0000-0000-000000000003"},
            "team_pk": {"S": "TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61"},
            "gsi2_sk": {"S": "USER#00000000-0000-0000-0000-000000000003"},
            "gsi1_sk": {"S": "-4611686018420032497"}
        }'

echo 'Admin user created successfully'
echo 'Login credentials:'
echo '  Email: admin@ratel.foundation'
echo '  Password: admin!234'
echo '  Username: admin'
