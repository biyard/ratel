"""
One-shot backfill for the 2026-04-19 ~ 04-20 Biyard outage.

Reads `MISSING_IN_BIYARD` rows from a reconciliation detail CSV
(produced by the audit pipeline) and creates `PendingReward` rows in
`ratel-hackartist-main` so the retry endpoint
(`POST /api/admin/internal/run-pending-reward-retry`) can replay each
award against Biyard with the original month preserved.

`created_at` is set to the original `ratel_ts` (April) so the retry
worker formats `Some("YYYY-MM")` and Biyard bills the points against
April — even when the operator runs this in May.

Idempotent: every PutItem uses `ConditionExpression:
attribute_not_exists(pk)` so re-running `--execute` never duplicates a
row for the same `sk`.

Setup (boto3 only — keep out of ratel's Cargo deps):

  python3 -m venv /tmp/ratel-backfill-venv
  /tmp/ratel-backfill-venv/bin/pip install boto3

Usage:

  PY=/tmp/ratel-backfill-venv/bin/python

  # 1) preview — no DDB writes
  $PY scripts/backfill_pending_rewards.py --dry-run > targets.csv

  # 2) smoke test — adds exactly one row
  $PY scripts/backfill_pending_rewards.py --execute --limit 1

  # 3) full backfill — adds the rest (smoke row dedups via ConditionExpression)
  $PY scripts/backfill_pending_rewards.py --execute

After execute, call the retry endpoint to drain `PENDING` rows:

  curl -X POST <host>/api/admin/internal/run-pending-reward-retry \\
    -H 'Cookie: <admin session>'

Expected detail CSV columns: user_id, ratel_point_raw, ratel_ts,
ratel_sk, status. AWS creds: standard env / ~/.aws/credentials chain.
"""

import argparse
import csv
import sys
from datetime import datetime, timezone

import boto3
from botocore.exceptions import ClientError

REGION = "ap-northeast-2"
TABLE = "ratel-hackartist-main"
SPACE_ID = "019d70df-dfc0-7222-be71-e55c2bd8121a"
DEFAULT_DETAIL_CSV = "/tmp/ratel-audit-019d70df/biyard_reconciliation_detail.csv"

# i64::MIN — DynamoEntity macro shifts timestamps by this so signed ints sort
# lexicographically when serialized as fixed-width strings.
I64_MIN = -9223372036854775808


def parse_ts_ms(s: str) -> int:
    """Audit CSV stores ratel_ts as 'YYYY-MM-DD HH:MM:SS' UTC."""
    return int(
        datetime.strptime(s, "%Y-%m-%d %H:%M:%S")
        .replace(tzinfo=timezone.utc)
        .timestamp()
        * 1000
    )


def reward_key_from_history_sk(sk: str) -> str:
    """ratel UserRewardHistory.sk = '{RewardKey}###{time_key}' — drop the suffix."""
    if "###" in sk:
        return sk.split("###", 1)[0]
    return sk


def build_item(row: dict, owner_pk: str) -> dict:
    target_pk = f"USER#{row['user_id']}"
    amount = int(row["ratel_point_raw"])
    reward_key = reward_key_from_history_sk(row["ratel_sk"])
    ts_ms = parse_ts_ms(row["ratel_ts"])
    sk = f"PENDING_REWARD#{ts_ms}#{target_pk}#{reward_key}"
    space_pk = f"SPACE#{SPACE_ID}"

    shifted = ts_ms - I64_MIN
    gsi1_sk = f"TS#{shifted:020d}"

    return {
        "pk": {"S": "PENDING_REWARD"},
        "sk": {"S": sk},
        "created_at": {"N": str(ts_ms)},
        "target_pk": {"S": target_pk},
        "owner_pk": {"S": owner_pk},
        "space_pk": {"S": space_pk},
        "reward_key": {"S": reward_key},
        "amount": {"N": str(amount)},
        "description": {"S": "outage-backfill"},
        "status": {"S": "PENDING"},
        "updated_at": {"N": str(ts_ms)},
        "retry_count": {"N": "0"},
        "last_error": {"S": ""},
        "gsi1_pk": {"S": "PR_STATUS#PENDING"},
        "gsi1_sk": {"S": gsi1_sk},
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--detail-csv", default=DEFAULT_DETAIL_CSV)
    ap.add_argument("--owner-pk", default="TEAM#840")
    ap.add_argument("--limit", type=int, default=None,
                    help="Process only the first N rows (for smoke test)")
    ap.add_argument("--user-id", default=None,
                    help="Process only rows whose user_id matches (e.g. for single-user smoke test)")
    grp = ap.add_mutually_exclusive_group(required=True)
    grp.add_argument("--dry-run", action="store_true",
                     help="Print proposed items to stdout as CSV")
    grp.add_argument("--execute", action="store_true",
                     help="PutItem to ratel-hackartist-main (idempotent)")
    args = ap.parse_args()

    rows = []
    with open(args.detail_csv) as f:
        for r in csv.DictReader(f):
            if r.get("status") != "MISSING_IN_BIYARD":
                continue
            if args.user_id and r.get("user_id") != args.user_id:
                continue
            rows.append(r)

    if args.limit:
        rows = rows[: args.limit]

    print(f"MISSING_IN_BIYARD rows to process: {len(rows)}", file=sys.stderr)

    items = [build_item(r, args.owner_pk) for r in rows]
    total = sum(int(it["amount"]["N"]) for it in items)
    by_user = {}
    for it in items:
        u = it["target_pk"]["S"]
        by_user[u] = by_user.get(u, 0) + int(it["amount"]["N"])

    if args.dry_run:
        w = csv.writer(sys.stdout)
        w.writerow(["user_id", "amount", "ts_utc", "reward_key", "sk"])
        for it in items:
            uid = it["target_pk"]["S"].replace("USER#", "")
            ts = int(it["created_at"]["N"])
            ts_str = datetime.fromtimestamp(ts / 1000, tz=timezone.utc).strftime(
                "%Y-%m-%d %H:%M:%S"
            )
            w.writerow([uid, it["amount"]["N"], ts_str, it["reward_key"]["S"], it["sk"]["S"]])
        print("", file=sys.stderr)
        print(f"Distinct users: {len(by_user)}", file=sys.stderr)
        print(f"Total amount:   {total:,} P", file=sys.stderr)
        print(f"Owner referral: {total // 10:,} P (10% bonus to {args.owner_pk})", file=sys.stderr)
        return

    ddb = boto3.client("dynamodb", region_name=REGION)
    ok, dup, fail = 0, 0, 0
    for it in items:
        try:
            ddb.put_item(
                TableName=TABLE,
                Item=it,
                ConditionExpression="attribute_not_exists(pk)",
            )
            ok += 1
        except ClientError as e:
            code = e.response.get("Error", {}).get("Code")
            if code == "ConditionalCheckFailedException":
                dup += 1
            else:
                fail += 1
                print(f"FAIL sk={it['sk']['S']}: {e}", file=sys.stderr)
    print(
        f"\nresult: success={ok} skipped_duplicate={dup} failed={fail}",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
