if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.space_common = {
    initialize: (_conf) => {
      console.debug("Initializing space_common with config");
    },
  };
}
