const credential = {
  request_identity_verification: async (storeId, channelKey, prefix) => {
    if (!window.PortOne || !window.PortOne.requestIdentityVerification) {
      return Promise.reject(new Error("PortOne SDK not loaded"));
    }
    const randomId =
      typeof crypto !== "undefined" && crypto.randomUUID
        ? crypto.randomUUID()
        : `${Date.now()}-${Math.floor(Math.random() * 1e9)}`;
    const identityVerificationId = `iv-${prefix}-${randomId}`;
    const res = await window.PortOne.requestIdentityVerification({
      storeId,
      identityVerificationId,
      channelKey,
    });

    return res.identityVerificationId;
  },
};

export default credential;
