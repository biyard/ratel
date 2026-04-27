/* Analyze arena DETAIL — interactions.
 *
 * Read-only saved-report view. The only interactions JS owns are:
 *   1. Sidebar item click → swap visible panel + select item.
 *      Reads `data-target-panel` on `.sb-item`, sets `data-active="true"`
 *      on the matching `<section class="panel">`, toggles
 *      `aria-selected` on every sidebar item.
 *   2. Sidebar group collapse/expand. Click the `.sb-group__head` to
 *      toggle `data-collapsed` on the group.
 *
 * Bar-row / tfidf-row / topic-table filter clicks and the LDA edit
 * toggle are intentionally NOT bound — the detail page is the saved
 * snapshot of an already-applied filter set, so per-row drill-down
 * here would just be visual noise without any data effect. Those live
 * with CREATE / preview only.
 *
 * Dioxus owns NONE of this state — it just renders the structure.
 * Wrapped in the standard MutationObserver bind-once pattern so it
 * works under both SSR (defer-loaded) and CSR (Dioxus mounts the page
 * after the script tag has already executed). The bind guard is on
 * `.sidebar` (the unique anchor that exists once per page).
 */
(function () {
  function initDetail() {
    var sidebar = document.querySelector(".analyze-arena .sidebar");
    if (!sidebar || sidebar.dataset.analyzeDetailBound) return;
    sidebar.dataset.analyzeDetailBound = "true";

    // (1) Sidebar item click → swap active panel + select item.
    var items = document.querySelectorAll(".analyze-arena .sb-item");
    var panels = document.querySelectorAll(".analyze-arena .panel");
    items.forEach(function (item) {
      item.addEventListener("click", function () {
        var target = item.getAttribute("data-target-panel");
        items.forEach(function (x) { x.setAttribute("aria-selected", "false"); });
        item.setAttribute("aria-selected", "true");
        panels.forEach(function (p) {
          p.setAttribute(
            "data-active",
            p.getAttribute("data-panel") === target ? "true" : "false"
          );
        });
      });
    });

    // (2) Sidebar group collapse/expand.
    document
      .querySelectorAll(".analyze-arena [data-group-toggle]")
      .forEach(function (head) {
        head.addEventListener("click", function () {
          var group = head.closest(".sb-group");
          if (!group) return;
          var collapsed = group.getAttribute("data-collapsed") === "true";
          group.setAttribute("data-collapsed", collapsed ? "false" : "true");
        });
      });
  }

  // Try immediately (SSR with defer).
  initDetail();
  // Also observe for CSR rendering (Dioxus adds elements after script runs).
  new MutationObserver(function () {
    var sidebar = document.querySelector(".analyze-arena .sidebar");
    if (sidebar && !sidebar.dataset.analyzeDetailBound) {
      initDetail();
    }
  }).observe(document.body, { childList: true, subtree: true });
})();
