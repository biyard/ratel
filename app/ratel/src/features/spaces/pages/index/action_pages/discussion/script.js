// ESM module. Loaded via `<script type="module">` for auto-init and
// imported by wasm_bindgen for `resetComposerHeight`. Both paths are
// idempotent thanks to `data-*-bound` flags so a double-load is safe.

function resizeTextarea(el) {
  // Reset to `auto` first — `scrollHeight` is clamped by the current
  // height, so reading it without the reset returns the previous value
  // and the textarea never shrinks after backspace.
  el.style.height = "auto";
  el.style.height = el.scrollHeight + "px";
}

function initComposerAutogrow() {
  var inputs = document.querySelectorAll(
    ".comment-input__textarea:not([data-autogrow-bound]), .reply-input__field:not([data-autogrow-bound])",
  );
  inputs.forEach(function (el) {
    el.dataset.autogrowBound = "true";
    el.addEventListener("input", function () {
      resizeTextarea(el);
    });
    // Size for pre-filled content (draft restore) after layout settles.
    requestAnimationFrame(function () {
      resizeTextarea(el);
    });
  });
}

// Called from Rust after a successful submit. Programmatic value changes
// from Dioxus's signal binding do not fire `input`, so the autogrow
// listener never sees the clear and the inline height stays stretched.
export function resetComposerHeight() {
  var inputs = document.querySelectorAll(
    ".comment-input__textarea, .reply-input__field",
  );
  inputs.forEach(function (el) {
    if (!el.value) {
      el.style.height = "";
    }
  });
}

var DROPDOWN_MAX_HEIGHT = 220;
function initMentionFlip() {
  var dropdowns = document.querySelectorAll(
    '[role="listbox"]:not([data-mention-flip-bound])',
  );
  dropdowns.forEach(function (el) {
    var anchor = el.parentElement;
    if (!anchor || !anchor.classList.contains("relative")) return;
    el.dataset.mentionFlipBound = "true";
    var rect = anchor.getBoundingClientRect();
    var spaceBelow = window.innerHeight - rect.bottom;
    if (spaceBelow < DROPDOWN_MAX_HEIGHT) {
      anchor.setAttribute("data-mention-flip", "up");
    } else {
      anchor.removeAttribute("data-mention-flip");
    }
  });
}

function initBottomSheet() {
  var sheet = document.getElementById("discussion-comments-sheet");
  if (!sheet || sheet.dataset.bound) return;
  sheet.dataset.bound = "true";

  var handle = sheet.querySelector(".sheet-handle");
  if (!handle) return;

  var expanded = false;
  var startY = 0;
  var startTranslate = 0;
  var dragging = false;
  var collapsedOffset = 0;

  function getCollapsedOffset() {
    return sheet.offsetHeight - 64;
  }

  function toggle() {
    expanded = !expanded;
    sheet.classList.toggle("expanded", expanded);
  }

  handle.addEventListener("click", function () {
    if (dragging) return;
    toggle();
  });

  handle.addEventListener(
    "touchstart",
    function (e) {
      startY = e.touches[0].clientY;
      collapsedOffset = getCollapsedOffset();
      startTranslate = expanded ? 0 : collapsedOffset;
      dragging = false;
      sheet.classList.add("dragging");
    },
    { passive: true },
  );

  handle.addEventListener(
    "touchmove",
    function (e) {
      var dy = e.touches[0].clientY - startY;
      if (Math.abs(dy) < 5 && !dragging) return;
      dragging = true;
      e.preventDefault();
      var next = Math.max(0, Math.min(collapsedOffset, startTranslate + dy));
      sheet.style.transform = "translateY(" + next + "px)";
    },
    { passive: false },
  );

  handle.addEventListener("touchend", function (e) {
    sheet.classList.remove("dragging");
    if (!dragging) return;

    var dy = e.changedTouches[0].clientY - startY;

    if (expanded) {
      if (dy > 60) expanded = false;
    } else {
      if (dy < -60) expanded = true;
    }

    sheet.style.transform = "";
    sheet.classList.toggle("expanded", expanded);
    setTimeout(function () {
      dragging = false;
    }, 50);
  });

  handle.addEventListener("mousedown", function (e) {
    startY = e.clientY;
    collapsedOffset = getCollapsedOffset();
    startTranslate = expanded ? 0 : collapsedOffset;
    dragging = false;
    sheet.classList.add("dragging");

    function onMouseMove(e) {
      var dy = e.clientY - startY;
      if (Math.abs(dy) < 5 && !dragging) return;
      dragging = true;
      var next = Math.max(0, Math.min(collapsedOffset, startTranslate + dy));
      sheet.style.transform = "translateY(" + next + "px)";
    }

    function onMouseUp(e) {
      document.removeEventListener("mousemove", onMouseMove);
      document.removeEventListener("mouseup", onMouseUp);
      sheet.classList.remove("dragging");
      if (!dragging) return;

      var dy = e.clientY - startY;
      if (expanded) {
        if (dy > 60) expanded = false;
      } else {
        if (dy < -60) expanded = true;
      }

      sheet.style.transform = "";
      sheet.classList.toggle("expanded", expanded);
      setTimeout(function () {
        dragging = false;
      }, 50);
    }

    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);
  });
}

function init() {
  initComposerAutogrow();
  initMentionFlip();
  initBottomSheet();
}

init();
new MutationObserver(function () {
  init();
}).observe(document.body, { childList: true, subtree: true });
