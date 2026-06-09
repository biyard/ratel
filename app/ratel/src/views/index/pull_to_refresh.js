// Pull-to-refresh gesture driver for the home arena (mobile / Tauri WebView
// only — the Rust side that runs this is cfg-gated to `tauri-web`).
//
// Attaches touch handlers to `.home-arena__scroll`. When the user drags down
// while the list is already scrolled to the top, an arena-tone spinner is
// revealed; releasing past the threshold sends a signal to Rust via
// `dioxus.send(...)`, which re-runs the page loaders. Rust calls
// `window.__ratelPtrDone()` afterwards to retract the spinner.
(function () {
  var SCROLL_SEL = ".home-arena__scroll";
  var THRESHOLD = 64; // px of pull (after damping) needed to trigger
  var MAX = 96; // max visual pull
  var DAMP = 0.5; // finger travel → visual pull ratio

  function bind() {
    var el = document.querySelector(SCROLL_SEL);
    if (!el || el.dataset.ptrBound) return;
    el.dataset.ptrBound = "true";

    var indicator = document.createElement("div");
    indicator.className = "ptr-indicator";
    indicator.innerHTML = '<div class="ptr-spinner" aria-hidden="true"></div>';
    el.insertBefore(indicator, el.firstChild);

    var startY = null;
    var pulling = false;
    var pull = 0;
    var refreshing = false;

    function reset(animated) {
      indicator.style.transition = animated ? "height 0.25s ease, opacity 0.25s ease" : "";
      indicator.style.height = "0px";
      indicator.style.opacity = "0";
      indicator.classList.remove("ptr-ready", "ptr-refreshing");
    }

    window.__ratelPtrDone = function () {
      refreshing = false;
      reset(true);
    };

    el.addEventListener("touchstart", function (e) {
      if (refreshing) return;
      if (el.scrollTop <= 0) {
        startY = e.touches[0].clientY;
        pulling = true;
        indicator.style.transition = "";
      }
    }, { passive: true });

    el.addEventListener("touchmove", function (e) {
      if (!pulling || startY === null || refreshing) return;
      var dy = e.touches[0].clientY - startY;
      if (dy <= 0 || el.scrollTop > 0) { pull = 0; return; }
      e.preventDefault();
      pull = Math.min(dy * DAMP, MAX);
      indicator.style.height = pull + "px";
      indicator.style.opacity = String(Math.min(pull / THRESHOLD, 1));
      indicator.classList.toggle("ptr-ready", pull >= THRESHOLD);
    }, { passive: false });

    function onEnd() {
      if (!pulling) return;
      pulling = false;
      startY = null;
      if (pull >= THRESHOLD && !refreshing) {
        refreshing = true;
        indicator.classList.remove("ptr-ready");
        indicator.classList.add("ptr-refreshing");
        indicator.style.transition = "height 0.2s ease";
        indicator.style.height = THRESHOLD + "px";
        indicator.style.opacity = "1";
        try { dioxus.send(true); } catch (_) {}
        // Safety: retract after 6s even if Rust never calls __ratelPtrDone.
        setTimeout(function () { if (refreshing) window.__ratelPtrDone(); }, 6000);
      } else {
        reset(true);
      }
      pull = 0;
    }
    el.addEventListener("touchend", onEnd, { passive: true });
    el.addEventListener("touchcancel", onEnd, { passive: true });
  }

  bind();
  new MutationObserver(bind).observe(document.body, { childList: true, subtree: true });
})();
