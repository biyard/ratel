if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.ratel_admin = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_admin with config");
    },
  };
}
