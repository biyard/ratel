if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.ratel_user_reward = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_user_reward with config");
    },
  };
}
