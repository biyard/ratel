import teams from "./teams";
import users from "./users";
import membership from "./membership";
import spaces from "./spaces";
import common from "./common";
import tokens from "./tokens";

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
    tokens,
    user_credential: users.credential,

    social: {
      teams,
      users,
    },
  };
}
