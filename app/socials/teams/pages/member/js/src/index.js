if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.ratel_team_member = {
    initialize: (_conf) => {
      console.debug("Initializing ratel_team_member with config");
    },
  };
}
