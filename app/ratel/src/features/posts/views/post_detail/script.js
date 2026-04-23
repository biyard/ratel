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
