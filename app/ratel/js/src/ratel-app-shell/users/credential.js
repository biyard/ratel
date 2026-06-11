const credential = {
  initialize: (_conf) => {},
  request_identity_verification: async (storeId, channelKey, prefix) => {
    if (!window.PortOne || !window.PortOne.requestIdentityVerification) {
      return Promise.reject(new Error("PortOne SDK not loaded"));
    }
    const randomId =
      typeof crypto !== "undefined" && crypto.randomUUID
        ? crypto.randomUUID()
        : `${Date.now()}-${Math.floor(Math.random() * 1e9)}`;
    const identityVerificationId = `iv-${prefix}-${randomId}`;
    const payload = { storeId, identityVerificationId, channelKey };
    // In a WebView (Tauri Android) PortOne cannot open a popup, so it navigates
    // the WHOLE window to the PG page and must come back via redirectUrl —
    // without it the webview gets stuck on the iamport page (white screen) with
    // no way back into the app. PortOne appends its own params
    // (`identityVerificationId`, `transactionType`, ...) to this URL on return.
    // Only set it inside Tauri so desktop web keeps its exact popup payload.
    if (window.__TAURI_INTERNALS__ || window.isTauri) {
      payload.redirectUrl = window.location.origin + window.location.pathname;
      // Record the history depth so the return handler can pop the PG pages the
      // redirect is about to stack (see kyc_clear_history.js). Otherwise "back"
      // from the returned page lands on the verification screen.
      try {
        window.sessionStorage.setItem(
          "ratel_kyc_hist_len",
          String(window.history.length),
        );
      } catch (e) {}
    }
    const res = await window.PortOne.requestIdentityVerification(payload);

    return res.identityVerificationId;
  },
};

export default credential;
