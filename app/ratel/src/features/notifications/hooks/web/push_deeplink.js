// Stream notification-tap deep links to Rust. MainActivity sets
// `window.__RATEL_PENDING_URL__` (cold start) and/or fires `ratel-deeplink`
// (live tap). Forward the cold-start value once, then every live event.
(function () {
  if (window.__RATEL_PENDING_URL__) {
    var u = window.__RATEL_PENDING_URL__;
    window.__RATEL_PENDING_URL__ = null;
    dioxus.send(u);
  }
  window.addEventListener("ratel-deeplink", function (e) {
    dioxus.send(e.detail);
  });
})();
