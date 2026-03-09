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
  };
  return window.PortOne.requestIdentityVerification(payload).then(
    () => identityVerificationId,
  );
}

const membership = {
  requestIdentityVerification,
};

export default membership;
