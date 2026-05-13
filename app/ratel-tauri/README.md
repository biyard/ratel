# ratel-tauri

Tauri 2.x native shell that hosts the dioxus-web bundle from `app/ratel/`.

## Build & install (Android emulator)

```bash
# Build a release APK
make build-release

# Install on a running AVD
make install-debug
```

## Manual smoke-test checklist

- [ ] Cold start under 3s on Pixel 6 AVD (API 34)
- [ ] WASM bundle loads, hydration completes (check `adb logcat` for WebView errors)
- [ ] Sign-in flow completes with bypass code `000000`
- [ ] Session cookie persists across app kill/relaunch
- [ ] Authenticated routes load (e.g. user profile)
- [ ] `open_external_url` bridge: external link from a post opens system browser
- [ ] Back button closes app cleanly (no JNI crash in logcat)
- [ ] No third-party-cookie warnings in `adb logcat`
