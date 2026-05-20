// Mockup-only theme toggle for Fact-or-Fold design files.
// Persists choice in localStorage so the chosen theme survives page navigation
// between sibling mockup HTML files. Dropped when porting to Dioxus (the
// production app reads the theme from its own context).
(function () {
  var html = document.documentElement;
  var saved = null;
  try {
    saved = localStorage.getItem("ff-theme");
  } catch (e) {
    // localStorage may throw in some sandboxed contexts — fall through.
  }
  if (saved === "dark" || saved === "light") {
    html.setAttribute("data-theme", saved);
  }

  function syncIcon(btn) {
    btn.textContent =
      html.getAttribute("data-theme") === "light" ? "☾" : "☼";
    btn.setAttribute(
      "aria-label",
      html.getAttribute("data-theme") === "light"
        ? "Switch to dark theme"
        : "Switch to light theme"
    );
  }

  function mount() {
    if (document.getElementById("ff-theme-toggle")) return;
    var btn = document.createElement("button");
    btn.id = "ff-theme-toggle";
    btn.type = "button";
    btn.style.cssText = [
      "position: fixed",
      "bottom: 18px",
      "right: 18px",
      "z-index: 9999",
      "width: 44px",
      "height: 44px",
      "border-radius: 50%",
      "border: 1px solid var(--border-strong, rgba(255,255,255,0.2))",
      "background: var(--surface, rgba(0,0,0,0.6))",
      "color: var(--text-primary, #fff)",
      "cursor: pointer",
      "font-size: 18px",
      "line-height: 1",
      "box-shadow: var(--shadow-md, 0 8px 24px rgba(0,0,0,0.3))",
      "display: grid",
      "place-items: center",
      "backdrop-filter: blur(8px)",
      "transition: transform 0.15s",
    ].join(";");
    btn.addEventListener("mouseenter", function () {
      btn.style.transform = "scale(1.08)";
    });
    btn.addEventListener("mouseleave", function () {
      btn.style.transform = "scale(1)";
    });
    btn.addEventListener("click", function () {
      var next =
        html.getAttribute("data-theme") === "light" ? "dark" : "light";
      html.setAttribute("data-theme", next);
      try {
        localStorage.setItem("ff-theme", next);
      } catch (e) {
        // ignore
      }
      syncIcon(btn);
    });
    syncIcon(btn);
    document.body.appendChild(btn);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", mount);
  } else {
    mount();
  }
})();
