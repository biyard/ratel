function requestIdentityVerification(storeId, channelKey, prefix) {
  if (!window.PortOne || !window.PortOne.requestIdentityVerification) {
    return Promise.reject(new Error("PortOne SDK is not available"));
  }
  const randomId =
    typeof crypto !== "undefined" && crypto.randomUUID
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.floor(Math.random() * 1000000)}`;
  const identityVerificationId = `iv-${prefix}-${randomId}`;
  const payload = {
    storeId,
    identityVerificationId,
    channelKey,
    // bypass: {
    //   inicisUnified: {
    //     flgFixedUser: "N",
    //     directAgency: "PASS",
    //     logoUrl: "https://metadata.ratel.foundation/logos/logo-symbol.png",
    //   },
    // },
  };
  // WebView (Tauri): PortOne redirects the whole window to the PG page and
  // returns via redirectUrl. Without it the webview is stuck on iamport (white
  // screen). Only set inside Tauri so desktop keeps its popup. See credential.js.
  if (window.__TAURI_INTERNALS__ || window.isTauri) {
    payload.redirectUrl = window.location.origin + window.location.pathname;
    try {
      window.sessionStorage.setItem(
        "ratel_kyc_hist_len",
        String(window.history.length),
      );
    } catch (e) {}
  }
  return window.PortOne.requestIdentityVerification(payload).then(
    () => identityVerificationId,
  );
}

const membership = {
  requestIdentityVerification,
  request_identity_verification: requestIdentityVerification,
};

export default membership;
