import teams from "./teams";
import users from "./users";
import membership from "./membership";
import spaces from "./spaces";
import common from "./common";
import tokens from "./tokens";
import * as f from "./auth/firebase";

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
    common,
    tokens,
    user_credential: users.credential,

    social: {
      teams,
      users,
    },
    ...f,
    invoke: (method, args) => {
      if (method in window.ratel) {
        const func = window.ratel[method];
        if (typeof func === "function") {
          return func(args);
        } else {
          console.error(`Method ${method} is not a function`);
        }
      }
    }
  };
}
