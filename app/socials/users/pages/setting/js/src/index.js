if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.ratel_user_setting = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_user_setting with config");
    },
    connectWallet: async () => {
      const ethereum = window.ethereum;
      if (!ethereum || !ethereum.request) {
        throw new Error("Wallet not found");
      }
      const accounts = await ethereum.request({ method: "eth_requestAccounts" });
      if (!accounts || accounts.length === 0) {
        return null;
      }
      return accounts[0];
    },
  };
}
