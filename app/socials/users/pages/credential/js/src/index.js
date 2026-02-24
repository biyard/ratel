if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.ratel_user_credential = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_user_credential with config");
    },
    requestIdentityVerification: (storeId, channelKey, prefix) => {
      if (!window.PortOne || !window.PortOne.requestIdentityVerification) {
        return Promise.reject(new Error("PortOne SDK not loaded"));
      }
      const randomId =
        typeof crypto !== "undefined" && crypto.randomUUID
          ? crypto.randomUUID()
          : `${Date.now()}-${Math.floor(Math.random() * 1e9)}`;
      const identityVerificationId = `iv-${prefix}-${randomId}`;
      return window.PortOne.requestIdentityVerification({
        storeId,
        identityVerificationId,
        channelKey,
      }).then(() => identityVerificationId);
    },
  };
}
