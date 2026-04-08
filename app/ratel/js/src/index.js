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

        const userAgent = navigator.userAgent.toLowerCase();
        const isKakaoInApp = userAgent.includes("kakaotalk");

        if (isKakaoInApp) {
          const targetUrl = window.location.href;
          window.location.replace(
            `kakaotalk://web/openExternal?url=${encodeURIComponent(targetUrl)}`,
          );
        }
      },
    },
    ratel_team_setting: {
      initialize: (_conf) => {},
    },
    ratel_team_reward: {
      initialize: (_conf) => {},
    },
    ratel_user_reward: {
      initialize: (_conf) => {},
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
