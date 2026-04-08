import teams from "./teams";
import users from "./users";
import membership from "./membership";
import spaces from "./spaces";
import auth from "./auth";
import common from "./common";

if (typeof window !== "undefined") {
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
    spaces,
    auth,
    common,
    user_credential: users.credential,

    social: {
      teams,
      users,
    },
  };
}
