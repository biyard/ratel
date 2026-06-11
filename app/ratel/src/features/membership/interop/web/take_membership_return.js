// On a WebView/mobile PortOne redirect, the membership purchase flow returns to
// /membership?identityVerificationId=... after identity verification. Dioxus'
// router strips the query on boot, so read the copy index.html captured at
// document-start (window.__ratelInitialSearch), paired with the tier we stashed
// before the redirect. Returns [identityVerificationId, tier] once, then clears
// both markers. Returns null on desktop (popup flow leaves no query/marker).
try {
  var search = window.__ratelInitialSearch;
  if (search == null) search = window.location.search || "";
  window.__ratelInitialSearch = "";
  var params = new URLSearchParams(search);
  var id = params.get("identityVerificationId");
  var code = params.get("code");
  var tier = window.sessionStorage.getItem("ratel_membership_tier");
  window.sessionStorage.removeItem("ratel_membership_tier");
  window.sessionStorage.removeItem("ratel_kyc_hist_len");
  if (id && !code && tier) {
    dioxus.send([id, tier]);
  } else {
    dioxus.send(null);
  }
} catch (e) {
  dioxus.send(null);
}
