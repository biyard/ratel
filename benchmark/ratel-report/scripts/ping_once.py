import os
import json
from concurrent.futures import ThreadPoolExecutor, as_completed
from urllib.request import Request, urlopen
from urllib.error import URLError, HTTPError

def join_url(base: str, path: str) -> str:
    base = base.rstrip("/")
    if not path.startswith("/"):
        path = "/" + path
    return base + path

BASE = os.environ.get("PRESENCE_BASE", "http://localhost:3000")
PING_PATH = os.environ.get("PING_PATH", "/v3/presence/ping")
PAGE_KEY = os.environ.get("PAGE_KEY", "")
SID_FILE = os.environ.get("SID_FILE", "/tmp/presence_sids.txt")
PING_CONCURRENCY = int(os.environ.get("PING_CONCURRENCY", "200"))
TIMEOUT_SECS = float(os.environ.get("TIMEOUT_SECS", "10"))

URL = join_url(BASE, PING_PATH)

def ping_one(sid: str) -> bool:
    payload = json.dumps({"session_id": sid, "page_key": PAGE_KEY}).encode("utf-8")
    req = Request(URL, data=payload, method="POST", headers={"Content-Type": "application/json"})
    try:
        with urlopen(req, timeout=TIMEOUT_SECS) as r:
            body = r.read().decode("utf-8", errors="replace")
        try:
            return bool(json.loads(body).get("ok", False))
        except json.JSONDecodeError:
            return False
    except (URLError, HTTPError, TimeoutError):
        return False
    except Exception:
        return False

def main():
    with open(SID_FILE, "r", encoding="utf-8") as fp:
        sids = [line.strip() for line in fp if line.strip()]

    if not sids:
        print("pinged=0 total=0 ok=0")
        return

    workers = max(1, min(PING_CONCURRENCY, len(sids)))

    ok = 0
    with ThreadPoolExecutor(max_workers=workers) as ex:
        futs = [ex.submit(ping_one, sid) for sid in sids]
        for f in as_completed(futs):
            try:
                if f.result():
                    ok += 1
            except Exception:
                pass

    print(f"pinged={len(sids)} total={len(sids)} ok={ok}")

if __name__ == "__main__":
    main()
