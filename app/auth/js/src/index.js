import * as firebase from "./firebase";
import * as wallet from "./wallet";

if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.auth = {
    firebase,
    wallet,
  };
}
