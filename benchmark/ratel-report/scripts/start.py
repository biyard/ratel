import os
import json
from concurrent.futures import ThreadPoolExecutor, as_completed
from urllib.request import Request, urlopen

base = os.environ["PRESENCE_BASE"]
start_path = os.environ.get("START_PATH", "/v3/presence/start")
page_key = os.environ["PAGE_KEY"]
n = int(os.environ.get("N", "2000"))
workers = int(os.environ.get("START_CONCURRENCY", "200"))
sid_file = os.environ.get("SID_FILE", "/tmp/presence_sids.txt")

url = base + start_path
payload = json.dumps({"page_key": page_key}).encode("utf-8")

def one(_: int) -> str:
    req = Request(
        url,
        data=payload,
        method="POST",
        headers={"Content-Type": "application/json"},
    )
    with urlopen(req, timeout=10) as r:
        body = r.read().decode("utf-8")
    return json.loads(body)["session_id"]

sids = []
with ThreadPoolExecutor(max_workers=workers) as ex:
    futs = [ex.submit(one, i) for i in range(n)]
    for f in as_completed(futs):
        try:
            sid = f.result()
            if sid and sid != "IGNORED":
                sids.append(sid)
        except Exception:
            pass

with open(sid_file, "w", encoding="utf-8") as fp:
    fp.write("\n".join(sids) + ("\n" if sids else ""))

print(f"started={len(sids)} target={n} sid_file={sid_file}")
