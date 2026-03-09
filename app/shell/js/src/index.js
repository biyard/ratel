import teams from "./teams";
import users from "./users";
import membership from "./membership";

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
    membership,

    social: {
      teams,
      users,
    },
  };
}
