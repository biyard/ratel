if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.ratel_my_follower = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_my_follower with config");
    },
  };
}
