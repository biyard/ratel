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

  // Helpers are registered on `window.ratel.<module>` per the
  // project's JS namespace convention (see
  // `conventions/html-first-components.md` § "JS Namespace
  // Convention"). Stash the singleton so re-running this script on
  // SPA navigation doesn't pile up duplicate observers.
  window.ratel = window.ratel || {};
  window.ratel.factOrFold = window.ratel.factOrFold || {};
  if (window.ratel.factOrFold.stageScrollState) return;

  // Two-tier observer:
  //   - `timelineObserver` watches a specific `#globalTimeline` node
  //     for `data-state` flips. Scoped tight so chat-message inserts
  //     and other body mutations don't fire this callback.
  //   - `mountObserver` watches body only for the timeline appearing
  //     or being removed across SPA navigation. It re-attaches /
  //     detaches the timeline observer accordingly so a stale
  //     reference doesn't linger after the player leaves the room.
  var state = {
    timeline: null,
    timelineObserver: null,
  };

  function attachToTimeline(node) {
    if (state.timelineObserver) state.timelineObserver.disconnect();
    state.timeline = node;
    state.timelineObserver = new MutationObserver(function (mutations) {
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
      }
    });
    state.timelineObserver.observe(node, {
      attributes: true,
      attributeFilter: ["data-state"],
      subtree: true,
    });
    scrollActiveCenter();
  }

  function detachTimeline() {
    if (state.timelineObserver) {
      state.timelineObserver.disconnect();
      state.timelineObserver = null;
    }
    state.timeline = null;
  }

  // `defer` on the <script> tag means this runs after the SSR DOM is
  // parsed, so the strip is already present on initial load.
  var initial = document.getElementById("globalTimeline");
  if (initial) attachToTimeline(initial);

  // SPA / CSR safety net — body-level childList observer just for
  // detecting timeline mount/unmount. The heavy `data-state` work
  // happens on the scoped observer above.
  var mountObserver = new MutationObserver(function () {
    var current = document.getElementById("globalTimeline");
    if (current && current !== state.timeline) {
      attachToTimeline(current);
    } else if (!current && state.timeline) {
      detachTimeline();
    }
  });
  mountObserver.observe(document.body, { childList: true, subtree: true });

  window.ratel.factOrFold.stageScrollState = state;
})();
