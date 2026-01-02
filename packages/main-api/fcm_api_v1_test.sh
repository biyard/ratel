#!/bin/bash
# Login with gcloud auth login
# gcloud auth activate-service-account -key-file="YOUR_GCP_JSON_FILE_LOCATION"
TOKEN=$(gcloud auth print-access-token --scopes=https://www.googleapis.com/auth/firebase.messaging)
PROJECT_ID=$(gcloud config get-value core/project)
DEVICE_TOKEN=""

BODY="{
  \"message\": {
    \"token\": \"$DEVICE_TOKEN\",
    \"notification\": {
      \"title\": \"TITLE\",
      \"body\": \"BODY\"
    }
  }
}"

curl -X POST -H "Authorization: Bearer $TOKEN" \
-H "Content-Type: application/json" \
-d "$BODY" \
"https://fcm.googleapis.com/v1/projects/$PROJECT_ID/messages:send"
