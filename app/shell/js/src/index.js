import { requestIdentityVerification } from "./membership.js";

if (typeof window === "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel = {
    app_shell: {
      initialize: (_conf) => {
        console.debug("Initializing app shell with config");
      },
    },
    membership: {
      request_identity_verification: requestIdentityVerification,
    },

    social: {
      users: {
        credential: {
          request_identity_verification: async (
            storeId,
            channelKey,
            prefix,
          ) => {
            if (
              !window.PortOne ||
              !window.PortOne.requestIdentityVerification
            ) {
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
        },
      },
    }, // Social End
  };
}
