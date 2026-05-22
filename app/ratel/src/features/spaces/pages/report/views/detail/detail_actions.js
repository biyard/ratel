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
  if (
    window.ratel &&
    window.ratel.report &&
    window.ratel.report.__actionsBound
  ) {
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
      const btn =
        e.target && e.target.closest && e.target.closest("[data-act]");
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
    true
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
    true
  );

  // Wait one frame for Dioxus to re-render the popup with the new
  // aria-selected button, then scroll it into view. `block: "nearest"`
  // means the list only scrolls when the active row is actually off-
  // screen — single-page lists don't get nudged for no reason.
  function scrollSelectedSlashIntoView() {
    requestAnimationFrame(function () {
      const active = document.querySelector(
        '.report-detail__slash-pop [aria-selected="true"]'
      );
      if (active && active.scrollIntoView) {
        active.scrollIntoView({ block: "nearest", behavior: "smooth" });
      }
    });
  }

  function ensureChartLeadingParagraph(editor, observer) {
    if (!editor) return;
    const figures = editor.querySelectorAll('figure[contenteditable="false"]');
    let inserted = false;
    figures.forEach(function (fig) {
      const prev = fig.previousElementSibling;
      const needs =
        !prev ||
        (prev.tagName === "FIGURE" &&
          prev.getAttribute("contenteditable") === "false");
      if (!needs) return;
      const p = document.createElement("p");
      p.appendChild(document.createElement("br"));
      fig.parentNode.insertBefore(p, fig);
      inserted = true;
    });
    return inserted;
  }

  function initChartCaretAnchors() {
    const editor = document.querySelector(
      ".report-detail .ratel-editor .re-content"
    );
    if (!editor || editor.dataset.chartCaretBound) return;
    editor.dataset.chartCaretBound = "true";

    // Pass 1 — fix the SSR / initial state immediately.
    ensureChartLeadingParagraph(editor);

    // Pass 2 — watch for subsequent figure insertions (slash popup,
    // data picker, chart-type swap that re-renders innerHTML).
    //
    // The observer fires on every text-node mutation during typing,
    // which calls `querySelectorAll('figure[contenteditable="false"]')`
    // every keystroke when there are many charts. Batch the ensure
    // call through `requestIdleCallback` so it only runs once per
    // idle window, not once per mutation — typing stays snappy while
    // a freshly inserted figure still gets its leading paragraph
    // before the user clicks above it.
    let suspended = false;
    let scheduled = false;
    const idle =
      window.requestIdleCallback ||
      function (cb) {
        return setTimeout(cb, 100);
      };
    function scheduleEnsure() {
      if (suspended || scheduled) return;
      scheduled = true;
      idle(
        function () {
          scheduled = false;
          suspended = true;
          try {
            ensureChartLeadingParagraph(editor);
          } finally {
            suspended = false;
          }
        },
        { timeout: 500 }
      );
    }
    const observer = new MutationObserver(function (mutations) {
      if (suspended) return;
      // Fast path: bail out as soon as any added node is a figure.
      // Avoids the O(N) figure scan on plain text-node mutations.
      for (let i = 0; i < mutations.length; i++) {
        const m = mutations[i];
        if (m.type !== "childList") continue;
        for (let j = 0; j < m.addedNodes.length; j++) {
          const n = m.addedNodes[j];
          if (
            n.nodeType === 1 &&
            (n.tagName === "FIGURE" ||
              (n.querySelector && n.querySelector("figure")))
          ) {
            scheduleEnsure();
            return;
          }
        }
      }
    });
    observer.observe(editor, { childList: true, subtree: true });
  }

  // SSR — try immediately; CSR — retry on mutations until the editor
  // mounts.
  initChartCaretAnchors();
  new MutationObserver(function () {
    initChartCaretAnchors();
  }).observe(document.body, { childList: true, subtree: true });
})();
