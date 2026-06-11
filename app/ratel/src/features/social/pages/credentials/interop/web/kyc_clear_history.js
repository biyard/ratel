// Runs AFTER the credential is finalized on a WebView/mobile redirect return.
// The PortOne redirect navigates the whole webview to the PG page(s) and back,
// which leaves the iamport pages stacked in webview history — so pressing
// "back" from the returned credentials page lands on the verification screen
// instead of the page the user came from.
//
// `request_identity_verification` stashed `history.length` (in sessionStorage)
// right before the redirect. The number of entries added during the round trip
// is `history.length - saved`; going back that many lands on the original
// credentials entry (clean URL, no query → no re-finalize), so a further back
// exits to the pre-credentials page as the user expects.
try {
  const key = "ratel_kyc_hist_len";
  const savedRaw = window.sessionStorage.getItem(key);
  window.sessionStorage.removeItem(key);
  const clean = window.location.origin + window.location.pathname;
  if (savedRaw != null) {
    const saved = parseInt(savedRaw, 10);
    const delta = window.history.length - saved;
    if (Number.isFinite(delta) && delta > 0) {
      // Strip the query on the current entry first so that if the traversal is
      // ever interrupted we are not left on a re-triggering URL, then pop the
      // PG + return entries off the stack.
      window.history.replaceState(null, "", clean);
      window.history.go(-delta);
      dioxus.send(true);
    } else {
      window.history.replaceState(null, "", clean);
      dioxus.send(false);
    }
  } else {
    // No saved marker (e.g. desktop popup path) — just clean the URL.
    window.history.replaceState(null, "", clean);
    dioxus.send(false);
  }
} catch (e) {
  dioxus.send(false);
}
