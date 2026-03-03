if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  const requestIdentityVerification = (storeId, channelKey, prefix) => {
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
    };
    return window.PortOne.requestIdentityVerification(payload).then(
      () => identityVerificationId
    );
  };

  window.ratel.ratel_membership = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_membership with config");
    },
    request_identity_verification: requestIdentityVerification,
  };
}
