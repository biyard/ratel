// Reads the PortOne identity-verification result that the PG page appended to
// the URL after a WebView/mobile redirect. Returns the identityVerificationId
// (string) when present, otherwise null. History cleanup happens AFTER the
// credential is finalized (see kyc_clear_history.js) so we do not navigate away
// before the server call completes.
//
// On desktop the verification resolves via popup (no redirect), so the URL has
// no `identityVerificationId` and this always returns null — a harmless no-op.
try {
  // Dioxus' router strips the query from the URL on boot, so read the copy that
  // index.html captured at document-start. Consume it (clear) so later
  // /credentials visits within the same session do not re-trigger finalize.
  // Fall back to live location.search for the desktop/SSR path.
  var search = window.__ratelInitialSearch;
  if (search == null) search = window.location.search || "";
  window.__ratelInitialSearch = "";
  const params = new URLSearchParams(search);
  const id = params.get("identityVerificationId");
  const txType = params.get("transactionType");
  const code = params.get("code");
  // Success returns identityVerificationId with no error code. A failure
  // carries `code`/`message` — skip finalize in that case.
  if (id && !code && (!txType || txType === "IDENTITY_VERIFICATION")) {
    dioxus.send(id);
  } else {
    dioxus.send(null);
  }
} catch (e) {
  dioxus.send(null);
}
