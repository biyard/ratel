import os
import json
import time
from typing import Optional, List
from concurrent.futures import ThreadPoolExecutor, as_completed
from urllib.request import Request, urlopen
from urllib.error import URLError, HTTPError

def join_url(base: str, path: str) -> str:
    base = base.rstrip("/")
    if not path.startswith("/"):
        path = "/" + path
    return base + path

BASE = os.environ.get("PRESENCE_BASE", "http://localhost:3000")
START_PATH = os.environ.get("START_PATH", "/v3/presence/start")
PAGE_KEY = os.environ.get("PAGE_KEY", "")
N = int(os.environ.get("N", "2000"))
START_CONCURRENCY = int(os.environ.get("START_CONCURRENCY", "60"))
BATCH = int(os.environ.get("BATCH", "200"))
BATCH_SLEEP_SECS = float(os.environ.get("BATCH_SLEEP_SECS", "1"))
SID_FILE = os.environ.get("SID_FILE", "/tmp/presence_sids.txt")

URL = join_url(BASE, START_PATH)

def post_start() -> Optional[str]:
    payload = json.dumps({"page_key": PAGE_KEY}).encode("utf-8")
    req = Request(
        URL,
        data=payload,
        method="POST",
        headers={"Content-Type": "application/json"},
    )
    try:
        with urlopen(req, timeout=10) as r:
            body = r.read().decode("utf-8", errors="replace")
        sid = json.loads(body).get("session_id")
        if not sid or sid == "IGNORED":
            return None
        return str(sid)
    except (URLError, HTTPError, TimeoutError, json.JSONDecodeError, ValueError):
        return None

def run_batch(batch_n: int) -> List[str]:
    sids: List[str] = []
    workers = max(1, min(START_CONCURRENCY, batch_n))

    with ThreadPoolExecutor(max_workers=workers) as ex:
        futs = [ex.submit(post_start) for _ in range(batch_n)]
        for f in as_completed(futs):
            sid = f.result()
            if sid:
                sids.append(sid)

    return sids

def main():
    all_sids: List[str] = []
    remaining = N

    while remaining > 0:
        take = min(BATCH, remaining)
        got = run_batch(take)
        all_sids.extend(got)
        remaining -= take
        if remaining > 0 and BATCH_SLEEP_SECS > 0:
            time.sleep(BATCH_SLEEP_SECS)

    with open(SID_FILE, "w", encoding="utf-8") as fp:
        fp.write("\n".join(all_sids))
        if all_sids:
            fp.write("\n")

    print("started={} target={} sid_file={}".format(len(all_sids), N, SID_FILE))

if __name__ == "__main__":
    main()
