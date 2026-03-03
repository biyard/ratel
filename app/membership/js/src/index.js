if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.ratel_membership = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_membership with config");
    },
  };
}
