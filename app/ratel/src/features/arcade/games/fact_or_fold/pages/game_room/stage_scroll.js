// Keep the active stage-step pill horizontally centered inside the
// stage-timeline strip. Runs once on initial mount and re-runs every
// time any `.stage-step` flips its `data-state` (which is how the
// game_room component signals stage transitions). No-op on desktop —
// the strip isn't horizontally scrollable so `scrollIntoView` does
// nothing visible.
(function () {
  function scrollActiveCenter() {
    var el = document.querySelector(
      '#globalTimeline .stage-step[data-state="active"]'
    );
    if (!el || typeof el.scrollIntoView !== "function") return;
    try {
      el.scrollIntoView({
        behavior: "smooth",
        inline: "center",
        block: "nearest",
      });
    } catch (_e) {
      // Older Safari rejects the options bag — fall back to default
      // (instant, leading edge). Better than nothing.
      el.scrollIntoView();
    }
  }

  // Initial centering — `defer` on the <script> tag means this runs
  // after the SSR DOM is parsed, so the strip is already present.
  scrollActiveCenter();

  // SPA / CSR safety net — watch the entire document for `data-state`
  // attribute flips on stage-steps. Each round status change in
  // Dioxus rewrites that attribute, which is our cue to re-center.
  if (window.__ratelStageScrollObserver) return;
  var observer = new MutationObserver(function (mutations) {
    for (var i = 0; i < mutations.length; i++) {
      var m = mutations[i];
      if (
        m.type === "attributes" &&
        m.attributeName === "data-state" &&
        m.target &&
        m.target.classList &&
        m.target.classList.contains("stage-step")
      ) {
        scrollActiveCenter();
        return;
      }
      // Also catch the initial mount on client-side navigation —
      // the strip is added to the DOM after the page settles.
      if (m.type === "childList" && m.addedNodes && m.addedNodes.length) {
        for (var j = 0; j < m.addedNodes.length; j++) {
          var n = m.addedNodes[j];
          if (
            n.nodeType === 1 &&
            (n.id === "globalTimeline" ||
              (n.querySelector && n.querySelector("#globalTimeline")))
          ) {
            scrollActiveCenter();
            return;
          }
        }
      }
    }
  });
  observer.observe(document.body, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ["data-state"],
  });
  window.__ratelStageScrollObserver = observer;
})();
