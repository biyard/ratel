if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.ratel_user_space = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_user_space with config");
    },
  };
}
