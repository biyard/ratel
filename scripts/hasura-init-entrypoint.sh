#!/bin/bash

echo 'Waiting for Hasura to be ready...' &&
until curl -s http://hasura:8080/healthz > /dev/null 2>&1; do
    sleep 2
done &&
echo 'Tracking all tables in ratel database...' &&
curl -X POST http://hasura:8080/v1/metadata \
    -H 'Content-Type: application/json' \
    -H 'X-Hasura-Admin-Secret: ratel_admin_secret' \
    -d '{\"type\": \"pg_track_all_tables\", \"args\": {\"source\": \"default\"}}' &&
echo 'All tables tracked successfully'
