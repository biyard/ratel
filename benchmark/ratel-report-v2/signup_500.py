import os
import csv
import time
import random
import string
import asyncio
import httpx

BASE_URL = os.getenv("BASE_URL", "https://dev.ratel.foundation").rstrip("/")
COUNT = int(os.getenv("COUNT", "5000"))
PASSWORD = os.getenv("PASSWORD", "0x1111")
VERIFY_CODE = os.getenv("VERIFY_CODE", "000000")  # bypass일 때만 의미 있음
CONCURRENCY = int(os.getenv("CONCURRENCY", "10"))

PROFILE_URL = os.getenv(
    "PROFILE_URL",
    "https://metadata.ratel.foundation/ratel/default-profile.png"
)

def rid(n: int = 12) -> str:
    return "".join(random.choices(string.ascii_lowercase + string.digits, k=n))

async def signup_one(client: httpx.AsyncClient, idx: int) -> dict:
    u = rid(14)
    email = f"load+{int(time.time()*1000)}_{idx}_{u}@ratel.foundation"
    username = f"user{u[:12]}"
    display_name = f"load{u[:8]}"

    r1 = await client.post(
        f"{BASE_URL}/v3/auth/verification/send-verification-code",
        json={"email": email},
    )
    if r1.status_code != 200:
        return {"ok": False, "step": "send", "email": email, "status": r1.status_code, "body": r1.text}

    r2 = await client.post(
        f"{BASE_URL}/v3/auth/verification/verify-code",
        json={"email": email, "code": VERIFY_CODE},
    )
    if r2.status_code != 200:
        return {"ok": False, "step": "verify", "email": email, "status": r2.status_code, "body": r2.text}

    r3 = await client.post(
        f"{BASE_URL}/v3/auth/signup",
        json={
            "email": email,
            "password": PASSWORD,
            "code": VERIFY_CODE,
            "display_name": display_name,
            "username": username,
            "profile_url": PROFILE_URL,
            "description": "load-test-user",
            "term_agreed": True,
            "informed_agreed": True,
        },
    )
    if r3.status_code != 200:
        return {"ok": False, "step": "signup", "email": email, "status": r3.status_code, "body": r3.text}

    set_cookie = r3.headers.get("set-cookie", "")
    return {
        "ok": True,
        "email": email,
        "password": PASSWORD,
        "username": username,
        "display_name": display_name,
        "set_cookie": set_cookie,
    }

async def main():
    limits = httpx.Limits(max_keepalive_connections=200, max_connections=500)
    timeout = httpx.Timeout(30.0, connect=30.0)
    headers = {"Content-Type": "application/json", "Accept": "application/json"}

    results = []
    sem = asyncio.Semaphore(CONCURRENCY)

    async with httpx.AsyncClient(
        headers=headers,
        limits=limits,
        timeout=timeout,
        http2=True,
        verify=True,
    ) as client:
        async def run(i: int):
            async with sem:
                return await signup_one(client, i)

        tasks = [asyncio.create_task(run(i)) for i in range(COUNT)]
        for t in asyncio.as_completed(tasks):
            res = await t
            results.append(res)
            if res.get("ok"):
                print(f"[OK] {res['email']}")
            else:
                print(f"[FAIL] step={res.get('step')} status={res.get('status')} email={res.get('email')} body={res.get('body')[:200]}")

    ok = [r for r in results if r.get("ok")]
    fail = [r for r in results if not r.get("ok")]

    with open("users.csv", "w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["email", "password", "username", "display_name", "set_cookie"])
        for r in ok:
            w.writerow([r["email"], r["password"], r["username"], r["display_name"], r["set_cookie"]])

    with open("fails.csv", "w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["step", "status", "email", "body"])
        for r in fail:
            w.writerow([r.get("step"), r.get("status"), r.get("email"), r.get("body")])

    print(f"\nDONE: ok={len(ok)} fail={len(fail)}")
    print("-> users.csv 생성됨 (로그인용)")
    print("-> fails.csv 생성됨 (실패 원인 확인용)")

if __name__ == "__main__":
    asyncio.run(main())
