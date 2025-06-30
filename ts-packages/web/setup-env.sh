#!/bin/bash

echo "Generating .env.local from environment variables..."

# check if the file exists
if [ -f .env.local ]; then
    echo ".env.local already exists. Please remove it before running this script if you want to refresh."
    exit 0
fi

cat > .env.local <<EOL
NEXT_PUBLIC_ENV=local
NEXT_PUBLIC_LOG_LEVEL=debug
NEXT_PUBLIC_FIREBASE_API_KEY=${FIREBASE_API_KEY}
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=${FIREBASE_AUTH_DOMAIN}
NEXT_PUBLIC_FIREBASE_PROJECT_ID=${FIREBASE_PROJECT_ID}
NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=${FIREBASE_STORAGE_BUCKET}
NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=${FIREBASE_MESSAGING_SENDER_ID}
NEXT_PUBLIC_FIREBASE_APP_ID=${FIREBASE_APP_ID}
NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID=${FIREBASE_MEASUREMENT_ID}

NEXT_PUBLIC_SIGN_DOMAIN=dev.ratel.foundation
NEXT_PUBLIC_API_URL=https://api.dev.ratel.foundation
NEXT_PUBLIC_EXPERIMENT=true
NEXT_PUBLIC_GRAPHQL_URL=https://graphql.dev.ratel.foundation/v1/graphql
EOL

echo ".env.local created successfully."
