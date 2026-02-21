import * as theme from "./theme";

if (typeof window !== "undefined") {
  if (typeof window.ratel === "undefined") {
    window.ratel = {};
  }

  window.ratel.common = {
    theme,
  };
}
