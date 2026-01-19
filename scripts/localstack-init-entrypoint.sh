#!/bin/bash

echo "start"
yum update
yum install -y awscli

export ENDPOINT=http://localstack:4566

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
ADMIN_UUID="10000000-0000-0000-0000-000000000001"
ADMIN_PASSWORD_HASH="d590005c41712ddad6630ca03348fad16ce2fbfb611725116c14631ff02268d8"
TIMESTAMP=$(date +%s%3N)

# Membership
aws --endpoint-url=$ENDPOINT dynamodb batch-write-item --request-items file://scripts/dynamodb-data/membership.json

# Reward
aws --endpoint-url=$ENDPOINT dynamodb batch-write-item --request-items file://scripts/dynamodb-data/reward.json

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

# Create Attribute Codes
echo 'Creating attribute codes...'

# Attribute Code 1 - Konkuk female
aws --endpoint-url=$ENDPOINT dynamodb put-item \
  --table-name ratel-local-main \
  --item '{
    "pk": {"S": "ATTRIBUTE_CODE#wKFegq"},
    "sk": {"S": "ATTRIBUTE_CODE"},
    "created_at": {"N": "'${TIMESTAMP}'"},
    "university": {"S": "Konkuk"},
    "gender": {"S": "female"},
    "gsi1_pk": {"S": "AC#ATTRIBUTE_CODE"},
    "gsi1_sk": {"S": "AC#'${TIMESTAMP}'"}
  }'

# Attribute Code 2 - Konkuk male
aws --endpoint-url=$ENDPOINT dynamodb put-item \
  --table-name ratel-local-main \
  --item '{
    "pk": {"S": "ATTRIBUTE_CODE#bVn0Vq"},
    "sk": {"S": "ATTRIBUTE_CODE"},
    "created_at": {"N": "'$((TIMESTAMP + 1000))'"},
    "university": {"S": "Konkuk"},
    "gender": {"S": "male"},
    "gsi1_pk": {"S": "AC#ATTRIBUTE_CODE"},
    "gsi1_sk": {"S": "AC#'$((TIMESTAMP + 1000))'"}
  }'

# Attribute Code 3 - Sogang female
aws --endpoint-url=$ENDPOINT dynamodb put-item \
  --table-name ratel-local-main \
  --item '{
    "pk": {"S": "ATTRIBUTE_CODE#bIFviB"},
    "sk": {"S": "ATTRIBUTE_CODE"},
    "created_at": {"N": "'$((TIMESTAMP + 2000))'"},
    "university": {"S": "Sogang"},
    "gender": {"S": "female"},
    "gsi1_pk": {"S": "AC#ATTRIBUTE_CODE"},
    "gsi1_sk": {"S": "AC#'$((TIMESTAMP + 2000))'"}
  }'

# Attribute Code 4 - Sogang male
aws --endpoint-url=$ENDPOINT dynamodb put-item \
  --table-name ratel-local-main \
  --item '{
    "pk": {"S": "ATTRIBUTE_CODE#j94EA1"},
    "sk": {"S": "ATTRIBUTE_CODE"},
    "created_at": {"N": "'$((TIMESTAMP + 3000))'"},
    "university": {"S": "Sogang"},
    "gender": {"S": "male"},
    "gsi1_pk": {"S": "AC#ATTRIBUTE_CODE"},
    "gsi1_sk": {"S": "AC#'$((TIMESTAMP + 3000))'"}
  }'

echo 'Attribute codes created successfully'

# Test Users for Public Deliberation
echo 'Creating test users...'

i=1

## loop 10
while [ $i -le 8 ]
do
    echo "Creating test user $i"
    aws --endpoint-url=$ENDPOINT dynamodb put-item \
        --table-name ratel-local-main \
        --item '{
        "pk": {"S": "USER#00000000-0000-0000-0000-00000000000'${i}'"},
        "sk": {"S": "USER"},
        "created_at": {"N": "'${TIMESTAMP}'"},
        "updated_at": {"N": "'${TIMESTAMP}'"},
        "display_name": {"S": "User'${i}'"},
        "email": {"S": "hi+user'${i}'@biyard.co"},
        "profile_url": {"S": "https://metadata.ratel.foundation/ratel/default-profile.png"},
        "username": {"S": "user'${i}'"},
        "term_agreed": {"BOOL": true},
        "informed_agreed": {"BOOL": true},
        "user_type": {"N": "1"},
        "password": {"S": "'${ADMIN_PASSWORD_HASH}'"},
        "theme": {"N": "3"},
        "points": {"N": "0"},
        "followers_count": {"N": "0"},
        "followings_count": {"N": "0"},
        "description": {"S": ""},
        "gsi1_pk": {"S": "EMAIL#PASSWORD#hi+user'${i}'@biyard.co"},
        "gsi1_sk": {"S": "'${ADMIN_PASSWORD_HASH}'"},
        "gsi2_pk": {"S": "USERNAME#user'${i}'"},
        "gsi2_sk": {"S": "TS#'${TIMESTAMP}'"},
        "gsi3_pk": {"S": "EMAIL#hi+user'${i}'@biyard.co"},
        "gsi3_sk": {"S": "TS#'${TIMESTAMP}'"}
    }'

    i=$((i + 1))
done

aws --endpoint-url=$ENDPOINT dynamodb put-item \
  --table-name ratel-local-main \
  --item '{
    "pk": {"S": "USER#00000000-0000-0000-0000-000000000019"},
    "sk": {"S": "USER"},
    "created_at": {"N": "'${TIMESTAMP}'"},
    "updated_at": {"N": "'${TIMESTAMP}'"},
    "display_name": {"S": "Guest1"},
    "email": {"S": "hi+anon1@biyard.co"},
    "profile_url": {"S": "https://metadata.ratel.foundation/ratel/default-profile.png"},
    "username": {"S": "anon1"},
    "term_agreed": {"BOOL": true},
    "informed_agreed": {"BOOL": true},
    "user_type": {"N": "1"},
    "password": {"S": "'${ADMIN_PASSWORD_HASH}'"},
    "theme": {"N": "3"},
    "points": {"N": "0"},
    "followers_count": {"N": "0"},
    "followings_count": {"N": "0"},
    "description": {"S": "건국대 여자"},
    "gsi1_pk": {"S": "EMAIL#PASSWORD#hi+anon1@biyard.co"},
    "gsi1_sk": {"S": "'${ADMIN_PASSWORD_HASH}'"},
    "gsi2_pk": {"S": "USERNAME#anon1"},
    "gsi2_sk": {"S": "TS#'${TIMESTAMP}'"},
    "gsi3_pk": {"S": "EMAIL#hi+anon1@biyard.co"},
    "gsi3_sk": {"S": "TS#'${TIMESTAMP}'"}
  }'


i=1

while [ $i -le 2 ]
do
    aws --endpoint-url=$ENDPOINT dynamodb put-item \
    --table-name ratel-local-main \
    --item '{
        "pk": {"S": "USER#00000000-0000-0000-0000-00000000002'${i}'"},
        "sk": {"S": "USER"},
        "created_at": {"N": "'${TIMESTAMP}'"},
        "updated_at": {"N": "'${TIMESTAMP}'"},
        "display_name": {"S": "Creator'${i}'"},
        "email": {"S": "hi+admin'${i}'@biyard.co"},
        "profile_url": {"S": "https://metadata.ratel.foundation/ratel/default-profile.png"},
        "username": {"S": "admin'${i}'"},
        "term_agreed": {"BOOL": true},
        "informed_agreed": {"BOOL": true},
        "user_type": {"N": "1"},
        "password": {"S": "'${ADMIN_PASSWORD_HASH}'"},
        "theme": {"N": "3"},
        "points": {"N": "0"},
        "followers_count": {"N": "0"},
        "followings_count": {"N": "0"},
        "description": {"S": "공론조사 관리자"},
        "gsi1_pk": {"S": "EMAIL#PASSWORD#hi+admin'${i}'@biyard.co"},
        "gsi1_sk": {"S": "'${ADMIN_PASSWORD_HASH}'"},
        "gsi2_pk": {"S": "USERNAME#admin'${i}'"},
        "gsi2_sk": {"S": "TS#'${TIMESTAMP}'"},
        "gsi3_pk": {"S": "EMAIL#hi+admin'${i}'@biyard.co"},
        "gsi3_sk": {"S": "TS#'${TIMESTAMP}'"}
    }'
    i=$((i + 1))
done

echo 'Test users created successfully'

# Create UserMemberships for test users
echo 'Creating user memberships...'

# Calculate expiration timestamp (30 days from now in milliseconds)
EXPIRED_AT=$((TIMESTAMP + 30 * 24 * 60 * 60 * 1000))

# Admin1 - PRO Membership (40 credits)
aws --endpoint-url=$ENDPOINT dynamodb put-item \
  --table-name ratel-local-main \
  --item '{
    "pk": {"S": "USER#00000000-0000-0000-0000-000000000001"},
    "sk": {"S": "USER_MEMBERSHIP"},
    "created_at": {"N": "'${TIMESTAMP}'"},
    "updated_at": {"N": "'${TIMESTAMP}'"},
    "expired_at": {"N": "'${EXPIRED_AT}'"},
    "membership_pk": {"S": "MEMBERSHIP#PRO"},
    "status": {"S": "Active"},
    "total_credits": {"N": "40"},
    "remaining_credits": {"N": "40"},
    "auto_renew": {"BOOL": true},
    "gsi1_pk": {"S": "UM#MEMBERSHIP#PRO"},
    "gsi1_sk": {"S": "TS#'${TIMESTAMP}'"},
    "gsi2_pk": {"S": "USER#00000000-0000-0000-0000-000000000001"},
    "gsi2_sk": {"S": "TS#'${TIMESTAMP}'"},
    "gsi3_pk": {"S": "USER_MEMBERSHIP#00000000-0000-0000-0000-000000000001"},
    "gsi3_sk": {"S": "TS#'${TIMESTAMP}'"}
  }'

# User1 - PRO Membership (40 credits)
aws --endpoint-url=$ENDPOINT dynamodb put-item \
  --table-name ratel-local-main \
  --item '{
    "pk": {"S": "USER#00000000-0000-0000-0000-000000000001"},
    "sk": {"S": "USER_MEMBERSHIP"},
    "created_at": {"N": "'${TIMESTAMP}'"},
    "updated_at": {"N": "'${TIMESTAMP}'"},
    "expired_at": {"N": "'${EXPIRED_AT}'"},
    "membership_pk": {"S": "MEMBERSHIP#PRO"},
    "status": {"S": "Active"},
    "total_credits": {"N": "40"},
    "remaining_credits": {"N": "40"},
    "auto_renew": {"BOOL": true},
    "gsi1_pk": {"S": "UM#MEMBERSHIP#PRO"},
    "gsi1_sk": {"S": "TS#'${TIMESTAMP}'"}
  }'

echo 'User memberships created successfully'

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


echo '======================================'
echo 'LocalStack initialization completed!'
echo '======================================'
echo ''
echo 'Test Users Created:'
echo '  user1: hi+user1@biyard.co / admin!234'
echo '  user2: hi+user2@biyard.co / admin!234'
echo '  user3: hi+user3@biyard.co / admin!234'
echo '  user4: hi+user4@biyard.co / admin!234'
echo '  user5: hi+user5@biyard.co / admin!234'
echo '  user6: hi+user6@biyard.co / admin!234'
echo '  user7: hi+user7@biyard.co / admin!234'
echo '  user8: hi+user8@biyard.co / admin!234'
echo '  anon1: hi+anon1@biyard.co / admin!234'
echo '  admin1: hi+admin1@biyard.co / admin!234'
echo '  admin2: hi+admin2@biyard.co / admin!234'
echo ''
echo 'System Admin:'
echo '  Email: admin@ratel.foundation'
echo '  Password: admin!234'
echo '  Username: admin'
echo ''
echo 'User Memberships:'
echo '  user1: PRO (40 credits)'
echo ''
echo 'Attribute Codes:'
echo '  j94EA1 - Sogang Male'
echo '  bIFviB - Sogang Female'
echo '  bVn0Vq - Konkuk Male'
echo '  wKFegq - Konkuk Female'
echo '======================================'
