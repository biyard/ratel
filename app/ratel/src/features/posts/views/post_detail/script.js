// Mobile bottom-sheet drag-to-dismiss for the post detail comments panel.
// Opens by Dioxus toggling `comments_open` (slides up via CSS animation).
// This script handles the downward-drag gesture: while dragging, the sheet
// follows the finger/mouse; release past a threshold fires a click on the
// backdrop so Dioxus's `close_comments` handler runs and tears down the
// sheet state. Below the threshold, the sheet snaps back to position 0.
//
// Desktop keeps the panel as a static right-side column — the handle is
// CSS-hidden above 720px so this gesture effectively only runs on mobile.
(function () {
  var DISMISS_THRESHOLD_PX = 120;

  function init() {
    var sheet = document.getElementById("post-comments-sheet");
    if (!sheet || sheet.dataset.bound) return;
    sheet.dataset.bound = "true";

    var handle = sheet.querySelector(".sheet-handle");
    if (!handle) return;

    var startY = 0;
    var currentDy = 0;
    var dragging = false;

    function clickBackdrop() {
      var backdrop = document.querySelector(".pd-drawer-backdrop");
      if (backdrop) backdrop.click();
    }

    function begin(clientY) {
      startY = clientY;
      currentDy = 0;
      dragging = false;
      sheet.classList.add("dragging");
    }

    function move(clientY) {
      var dy = clientY - startY;
      if (!dragging && Math.abs(dy) < 5) return;
      dragging = true;
      // Only follow downward drags — ignore upward since the sheet is
      // already at its open position.
      currentDy = Math.max(0, dy);
      sheet.style.transform = "translateY(" + currentDy + "px)";
    }

    function end() {
      sheet.classList.remove("dragging");
      if (!dragging) return;
      if (currentDy > DISMISS_THRESHOLD_PX) {
        // Animate fully off-screen before Dioxus unmounts, so the exit
        // feels continuous with the drag.
        sheet.style.transform = "translateY(100%)";
        setTimeout(clickBackdrop, 180);
      } else {
        sheet.style.transform = "";
      }
      setTimeout(function () {
        dragging = false;
      }, 50);
    }

    handle.addEventListener(
      "touchstart",
      function (e) {
        begin(e.touches[0].clientY);
      },
      { passive: true }
    );

    // Passive listener — the handle has `touch-action: none` in CSS, so
    // the browser never starts a native scroll on this region and there's
    // nothing to preventDefault. Calling preventDefault here would log
    // "Ignored attempt to cancel a touchmove event with cancelable=false"
    // on devices where the UA commits to scrolling before JS runs.
    handle.addEventListener(
      "touchmove",
      function (e) {
        move(e.touches[0].clientY);
      },
      { passive: true }
    );

    handle.addEventListener("touchend", function () {
      end();
    });

    handle.addEventListener("mousedown", function (e) {
      begin(e.clientY);

      function onMouseMove(ev) {
        move(ev.clientY);
      }

      function onMouseUp() {
        document.removeEventListener("mousemove", onMouseMove);
        document.removeEventListener("mouseup", onMouseUp);
        end();
      }

      document.addEventListener("mousemove", onMouseMove);
      document.addEventListener("mouseup", onMouseUp);
    });
  }

  init();
  new MutationObserver(function () {
    init();
  }).observe(document.body, { childList: true, subtree: true });
})();

// AC-18 — referral banner for signed-out viewers landing from a syndicated
// copy. Reads `?utm_source=` and `document.referrer` to pick a tier:
//   tier-1: utm_source matches a known platform (bluesky / linkedin / threads)
//   tier-2: referrer host matches a known platform domain
//   tier-3: any other referrer (generic "from somewhere" copy)
// No referrer + no utm_source → bar stays hidden.
//
// The bar is rendered only when the viewer is signed-out (Dioxus controls
// the `[data-show="true"]` toggle via session check). This script only
// owns the platform classification.
(function () {
  function classify() {
    try {
      var qs = new URLSearchParams(window.location.search);
      var utm = (qs.get("utm_source") || "").toLowerCase();
      var known = { bluesky: 1, linkedin: 1, threads: 1 };
      if (known[utm]) return { tier: 1, platform: utm };

      var ref = document.referrer || "";
      if (ref) {
        var host = "";
        try {
          host = new URL(ref).host.toLowerCase();
        } catch (_) {}
        if (host.indexOf("bsky.app") !== -1 || host.indexOf("bsky.social") !== -1)
          return { tier: 2, platform: "bluesky" };
        if (host.indexOf("linkedin.com") !== -1) return { tier: 2, platform: "linkedin" };
        if (host.indexOf("threads.net") !== -1 || host.indexOf("threads.com") !== -1)
          return { tier: 2, platform: "threads" };
        return { tier: 3, platform: "" };
      }
      return null;
    } catch (e) {
      return null;
    }
  }

  function paint() {
    var bar = document.querySelector(".pd-refer-bar");
    if (!bar || bar.dataset.painted) return;
    bar.dataset.painted = "true";
    var c = classify();
    if (!c) {
      bar.setAttribute("data-show", "false");
      return;
    }
    bar.setAttribute("data-show", "true");
    bar.setAttribute("data-tier", String(c.tier));
    if (c.platform) bar.setAttribute("data-platform", c.platform);

    var closeBtn = bar.querySelector(".pd-refer-bar__close");
    if (closeBtn) {
      closeBtn.addEventListener("click", function () {
        bar.setAttribute("data-show", "false");
      });
    }
  }

  paint();
  new MutationObserver(function () {
    paint();
  }).observe(document.body, { childList: true, subtree: true });
})();
