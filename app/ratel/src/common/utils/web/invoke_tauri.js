// Direct Tauri IPC invoke, bypassing the `window.ratel` JS dispatch table
// (which only routes method names that exist as functions on `window.ratel`).
// Used by the tauri-web HTTP transport to call the native `api_request`
// command, so API calls go through a native reqwest cookie jar instead of the
// in-WebView fetch — iOS WKWebView (ITP) strips the cross-site session cookie.
const { method, args } = await dioxus.recv();
try {
  const res = await window.__TAURI_INTERNALS__.invoke(method, args);
  dioxus.send(res);
} catch (e) {
  console.error(`tauri invoke ${method} failed`, e);
  dioxus.send(null);
}
