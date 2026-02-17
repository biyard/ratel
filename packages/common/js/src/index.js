import * as firebase from "./firebase";

if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.common = {
    firebase,
  };
}
