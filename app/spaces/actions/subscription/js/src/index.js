if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.space_action_subscription = {
    initialize: (_conf) => {
      console.debug("Initializing space_action_subscription with config");
    },
  };
}
