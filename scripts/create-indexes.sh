#!/usr/bin/env bash
set -euo pipefail

TABLE="ratel-prod-main"
REGION="ap-northeast-2"

INDEXES=(
  # "gsi1_pk gsi1_sk gsi1-index"
  # "gsi2_pk gsi2_sk gsi2-index"
  "gsi3_pk gsi3_sk gsi3-index"
  "gsi4_pk gsi4_sk gsi4-index"
  "gsi5_pk gsi5_sk gsi5-index"
  "gsi6_pk gsi6_sk gsi6-index"
)

wait_index_active() {
  local table="$1" index="$2" region="$3"
  echo "⏳ Waiting for $index to become ACTIVE..."
  while true; do
    status=$(aws dynamodb describe-table \
      --region "$region" \
      --table-name "$table" \
      --query "Table.GlobalSecondaryIndexes[?IndexName=='$index'].IndexStatus | [0]" \
      --output text 2>/dev/null || echo "UNKNOWN")

    # when index not yet visible, status may be "None" or "UNKNOWN"
    if [[ "$status" == "ACTIVE" ]]; then
      echo "✅ $index is ACTIVE"
      break
    fi
    sleep 5
  done
}

for def in "${INDEXES[@]}"; do
  read -r PK SK NAME <<<"$def"

  echo "🚀 Creating $NAME on $TABLE"

  # Build AttributeDefinitions only for keys that aren't already defined (optional optimization).

  aws dynamodb update-table \
    --region "$REGION" \
    --table-name "$TABLE" \
    --attribute-definitions AttributeName="$PK",AttributeType="S" AttributeName="$SK",AttributeType="S" \
    --global-secondary-index-updates "[
      {
        \"Create\": {
          \"IndexName\": \"$NAME\",
          \"KeySchema\": [
            {\"AttributeName\": \"$PK\", \"KeyType\": \"HASH\"},
            {\"AttributeName\": \"$SK\", \"KeyType\": \"RANGE\"}
          ],
          \"Projection\": {\"ProjectionType\": \"ALL\"}
        }
      }
    ]" > /dev/null

  wait_index_active "$TABLE" "$NAME" "ap-northeast-2"
  wait_index_active "$TABLE" "$NAME" "us-east-1"
  wait_index_active "$TABLE" "$NAME" "eu-central-1"
done
