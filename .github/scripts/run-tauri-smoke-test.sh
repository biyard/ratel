#!/usr/bin/env bash
# Tauri Android smoke test driver — invoked from
# `reactivecircus/android-emulator-runner@v2`'s `script:` block. Wrapping
# the logic in a sourced file (rather than inlining in YAML) is necessary
# because the action runs each line of an inline `script:` block as a
# separate `sh -c <line>` invocation, which breaks multi-line shell
# constructs like `for ... do ... done`.
#
# Required env:
#   APK_PATH            Absolute path to the installed APK
#   TAURI_API_BASE      Backend the APK and smoke test talk to
#   GITHUB_OUTPUT       Set automatically by the action runner

set -eu

APK_PATH="${1:?APK_PATH must be passed as first arg}"

# Always capture logcat + devtools snapshot on exit, even when the spec
# fails — debugging without these is borderline impossible.
trap 'PID="${PID:-$(adb shell pidof co.biyard.ratel 2>/dev/null | tr -d "\r" || true)}"
      adb logcat -d ${PID:+--pid=$PID} > /tmp/tauri-logcat.txt 2>&1 || true
      curl -sf http://localhost:9223/json/list > /tmp/tauri-devtools-list.json 2>&1 || true
      curl -sf http://localhost:9223/json/version > /tmp/tauri-devtools-version.json 2>&1 || true' EXIT

adb wait-for-device

# Wait for the emulator to finish booting before we install.
timeout 180 bash -c 'until adb shell getprop sys.boot_completed | grep -q 1; do sleep 2; done'

adb install -r "$APK_PATH"

# Bridge the host backend into the emulator BEFORE launching the app.
# The APK is built with `MOBILE_API_URL=http://localhost:8080` — every
# first-paint server-fn call (session bootstrap, hot spaces, etc.) hits
# that URL during onCreate. If `adb reverse` is set up AFTER `am start`
# (the previous ordering), those calls connection-refuse, the WebView
# hangs in init, ActivityTaskManager hits the ~10s launch timeout and
# tears the process down — which manifested as the recurring
# "co.biyard.ratel process did not start" smoke failure even though
# the build was healthy. Reversing the port first removes the race.
#
# Chromium treats `localhost` as same-site with `tauri.localhost` (the
# WebView origin), so the SameSite=Lax session cookie set by the local
# backend rides across the cross-origin fetch. The emulator's own
# loopback doesn't reach the runner's docker stack; `adb reverse`
# punches the hole.
adb reverse tcp:8080 tcp:8080

adb shell am start -n co.biyard.ratel/.MainActivity

# Poll for the app process — the WebView devtools socket only exists
# once the app is running. Software-rendered swiftshader emulator on
# ubuntu-latest is slow enough that a tight 30s budget can race the
# WebView's first paint; give it ~90s and capture rolling logcat so a
# real crash (vs. slow boot) is distinguishable in the artifact.
PID=""
for _ in $(seq 1 90); do
  PID=$(adb shell pidof co.biyard.ratel | tr -d '\r' || true)
  [ -n "$PID" ] && break
  sleep 1
done
if [ -z "$PID" ]; then
  echo "::error::co.biyard.ratel process did not start"
  adb logcat -d | tail -200
  exit 1
fi
echo "App PID: $PID"

# Tauri WebView opens its devtools socket as
# `@webview_devtools_remote_<pid>`. adb forward bridges it to a local
# TCP port for Playwright's CDP connection.
adb forward tcp:9223 "localabstract:webview_devtools_remote_$PID"

# Sanity check: the devtools endpoint should list the WebView's page.
for _ in $(seq 1 30); do
  if curl -sf http://localhost:9223/json/version > /dev/null; then
    break
  fi
  sleep 1
done
if ! curl -sf http://localhost:9223/json/list > /dev/null; then
  echo "::error::devtools endpoint not reachable"
  adb logcat -d --pid="$PID" | tail -50
  exit 1
fi

# Print the devtools page listing so the artifact log shows what
# Playwright was looking at.
echo "=== /json/list (before spec) ==="
curl -sf http://localhost:9223/json/list || echo "(unable to fetch)"
echo "================================"

# Run the smoke spec.
cd playwright
TAURI_CDP_PORT=9223 \
  TAURI_API_BASE="${TAURI_API_BASE:-https://dev.ratel.foundation}" \
  CI=true \
  npx playwright test --project=Tauri
