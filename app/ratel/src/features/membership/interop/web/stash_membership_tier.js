// Before the PortOne identity-verification redirect (WebView/mobile), record
// which membership tier the user is purchasing so the return handler can resume
// the purchase flow after the page reloads. Only inside Tauri — desktop resolves
// the verification inline (popup) and never leaves the page.
const tier = await dioxus.recv();
try {
  if (window.__TAURI_INTERNALS__ || window.isTauri) {
    window.sessionStorage.setItem("ratel_membership_tier", tier);
  }
} catch (e) {}
dioxus.send(true);
