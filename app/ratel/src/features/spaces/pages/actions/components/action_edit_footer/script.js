// Smooth-scroll the action editor's `.pager` to a specific page index.
// Called from Rust (wasm_bindgen) when the user clicks Previous/Next in
// `ActionEditFooter`. The pager is `scroll-snap-type: x mandatory`, so
// any offset within a page snaps to it; we compute the canonical page
// boundary by `index * pager.clientWidth` and let the browser snap.
//
// Conversely, when the user *swipes* the pager directly we detect the
// settled snap point and call back into Rust via the window-attached
// closure `__ratel_aef_set_page` (registered by the Rust component) so
// the Previous/Next disabled state stays in sync with the actual
// scroll position.

(function () {
  function getPager() {
    return document.querySelector(".pager");
  }

  function pageIndex(pager) {
    var width = pager.clientWidth;
    if (!width) return 0;
    return Math.round(pager.scrollLeft / width);
  }

  function goToPage(index) {
    var pager = getPager();
    if (!pager) return;
    var width = pager.clientWidth;
    if (!width) return;
    pager.scrollTo({ left: index * width, behavior: "smooth" });
  }

  // Debounced sync from scroll → Rust signal. We wait ~120ms after the
  // last scroll event so the user's swipe (or the smooth-scroll
  // animation triggered by goToPage) has time to settle on a snap
  // point before we report the final index.
  var SCROLL_SETTLE_MS = 120;
  var scrollTimer = null;

  function notifyPageChange() {
    var pager = getPager();
    if (!pager) return;
    var idx = pageIndex(pager);
    var fn = window.__ratel_aef_set_page;
    if (typeof fn === "function") {
      try {
        fn(idx);
      } catch (e) {
        // Closure was dropped (e.g. component unmounted). Stale call
        // is harmless — swallow so other listeners keep working.
      }
    }
  }

  function bindScroll() {
    var pager = getPager();
    if (!pager || pager.dataset.aefScrollBound) return;
    pager.dataset.aefScrollBound = "true";
    pager.addEventListener(
      "scroll",
      function () {
        if (scrollTimer) clearTimeout(scrollTimer);
        scrollTimer = setTimeout(notifyPageChange, SCROLL_SETTLE_MS);
      },
      { passive: true }
    );
  }

  bindScroll();

  // CSR (Dioxus mounts the pager after this script first runs). Re-bind
  // on each mutation; the data-flag prevents double-binding.
  new MutationObserver(function () {
    bindScroll();
  }).observe(document.body, { childList: true, subtree: true });

  window.ratel = window.ratel || {};
  window.ratel.actionEditor = window.ratel.actionEditor || {};
  window.ratel.actionEditor.goToPage = goToPage;
})();
