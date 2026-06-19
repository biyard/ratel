// Hand the native-injected FCM token to Rust. MainActivity (Android) sets
// `window.__RATEL_FCM__ = {token, deviceId, platform}` and fires
// `ratel-fcm-ready`. Send it once it exists (now or on the event).
(function () {
  function grab() {
    if (window.__RATEL_FCM__) {
      dioxus.send(window.__RATEL_FCM__);
      return true;
    }
    return false;
  }
  if (!grab()) {
    window.addEventListener("ratel-fcm-ready", function () { grab(); }, { once: true });
  }
})();
