#!/bin/bash

echo "ENDPOINT: $ENDPOINT"
echo "COLLECTION: $COLLECTION"

status=`curl -sf $ENDPOINT/collections/$COLLECTION | grep ok | wc -l`


if [ "$status" -eq "1" ]; then
    echo 'Collection already exists'
    exit 0;
fi

curl -sf -X PUT $ENDPOINT/collections/$COLLECTION -H 'Content-Type: application/json' -d '{"vectors":{"size":1024,"distance":"Cosine"}}'

echo "Collection $COLLECTION created";
