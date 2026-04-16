const STORAGE_KEY = "ratel-common-theme";

// The user's *preference* — "light" | "dark" | "system" | null.
// Kept separate from the resolved `data-theme` attribute so that when
// the OS flips light/dark while the app is open we can re-resolve "system"
// on the fly instead of being stuck on whatever was first applied.
let userPreference = null;

const systemDarkQuery =
  typeof window !== "undefined" && window.matchMedia
    ? window.matchMedia("(prefers-color-scheme: dark)")
    : null;

function resolveTheme(theme) {
  if (theme !== "system") return theme;
  if (systemDarkQuery && systemDarkQuery.matches) return "dark";
  return "light";
}

function setAttr(theme) {
  window.document.documentElement.setAttribute("data-theme", theme);
}

// Re-resolve the "system" preference whenever the OS flips.
// `addEventListener("change", ...)` is the modern API; the older
// `addListener` is kept as a fallback for very old WebViews.
if (systemDarkQuery) {
  const onChange = () => {
    if (userPreference === "system" || userPreference === null) {
      setAttr(resolveTheme("system"));
    }
  };
  if (typeof systemDarkQuery.addEventListener === "function") {
    systemDarkQuery.addEventListener("change", onChange);
  } else if (typeof systemDarkQuery.addListener === "function") {
    systemDarkQuery.addListener(onChange);
  }
}

export function load_theme() {
  return window.localStorage.getItem(STORAGE_KEY);
}

export function save_theme(theme) {
  window.localStorage.setItem(STORAGE_KEY, theme);
}

export function apply_theme(theme) {
  userPreference = theme;
  setAttr(resolveTheme(theme));
}
