const STORAGE_KEY = "ratel-common-theme";

export function load_theme() {
  return window.localStorage.getItem(STORAGE_KEY);
}

export function save_theme(theme) {
  window.localStorage.setItem(STORAGE_KEY, theme);
}

export function apply_theme(theme) {
  if (theme === "system") {
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      theme = "dark";
    } else {
      theme = "light";
    }
  }

  window.document.documentElement.setAttribute("data-theme", theme);
}
