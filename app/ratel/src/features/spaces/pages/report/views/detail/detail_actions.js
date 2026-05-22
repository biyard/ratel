// Report detail page — chart figure action delegation.
//
// Chart figures embedded in the editor's body carry two action buttons
// (settings, trash) that Rust generates as plain HTML inside
// `<figure contenteditable="false">…</figure>`. Buttons aren't React-
// style components; they're inert HTML. This script delegates clicks
// from any chart-action button to the page-level command bridge so the
// Rust context can react (open swap panel, delete figure).
//
// Idempotent — `__bound` flag prevents double-init across CSR mounts.

(function () {
  if (window.ratel && window.ratel.report && window.ratel.report.__actionsBound) {
    return;
  }
  window.ratel = window.ratel || {};
  window.ratel.report = window.ratel.report || {};
  window.ratel.report.__actionsBound = true;

  function commandBridge() {
    return document.querySelector(".report-detail__cmd-bridge");
  }

  function send(message) {
    const bridge = commandBridge();
    if (!bridge) return;
    bridge.value = message;
    bridge.dispatchEvent(new Event("input", { bubbles: true }));
  }

  document.addEventListener(
    "click",
    function (e) {
      const btn = e.target && e.target.closest && e.target.closest("[data-act]");
      if (!btn) return;
      const editor = btn.closest(".report-detail .ratel-editor .re-content");
      if (!editor) return;
      const figure = btn.closest("figure[data-chart-id]");
      if (!figure) return;
      const act = btn.getAttribute("data-act");
      const chartId = figure.getAttribute("data-chart-id");
      if (!act || !chartId) return;
      e.preventDefault();
      e.stopPropagation();
      send(act + ":" + chartId);
    },
    true,
  );

  // ── Slash popup keyboard navigation ───────────────────────
  // When the slash popup is rendered (Rust shows it whenever the
  // `slash` signal is `Some`), arrow / Enter / Esc must drive the
  // popup, NOT the underlying contenteditable (which would otherwise
  // move the caret or insert a newline). The capture-phase listener
  // intercepts the key first; preventDefault stops the editor from
  // seeing it.
  document.addEventListener(
    "keydown",
    function (e) {
      const popup = document.querySelector(".report-detail__slash-pop");
      if (!popup) return;
      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          e.stopPropagation();
          send("slash-down:");
          scrollSelectedSlashIntoView();
          break;
        case "ArrowUp":
          e.preventDefault();
          e.stopPropagation();
          send("slash-up:");
          scrollSelectedSlashIntoView();
          break;
        case "Enter":
          // Skip when the user is mid-IME composition — Enter then
          // commits the composition instead of selecting the option.
          if (e.isComposing) return;
          e.preventDefault();
          e.stopPropagation();
          send("slash-enter:");
          break;
        case "Escape":
          e.preventDefault();
          e.stopPropagation();
          send("slash-close:");
          break;
        default:
          break;
      }
    },
    true,
  );

  // Wait one frame for Dioxus to re-render the popup with the new
  // aria-selected button, then scroll it into view. `block: "nearest"`
  // means the list only scrolls when the active row is actually off-
  // screen — single-page lists don't get nudged for no reason.
  function scrollSelectedSlashIntoView() {
    requestAnimationFrame(function () {
      const active = document.querySelector(
        '.report-detail__slash-pop [aria-selected="true"]',
      );
      if (active && active.scrollIntoView) {
        active.scrollIntoView({ block: "nearest", behavior: "smooth" });
      }
    });
  }
})();
