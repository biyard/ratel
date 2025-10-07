#!/usr/bin/env bash
set -euo pipefail

TABLE="ratel-dev-main"
REGION="us-east-1"

# gsi1~gsi6 순서대로 생성 (필요한 것만 나열)
INDEXES=(
  # "gsi1_pk gsi1_sk gsi1-index"
  # "gsi2_pk gsi2_sk gsi2-index"
  # "gsi3_pk gsi3_sk gsi3-index"
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
  # 간단히는 그대로 넣고, 중복이면 에러 나면 아래 주석처럼 빼서 재시도하면 됨.

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

  wait_index_active "$TABLE" "$NAME" "$REGION"
done
