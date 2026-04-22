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

// Sheet expand state is Dioxus-managed (see `sheet_expanded` signal in
// component.rs). The handle's click is wired via Rust `onclick`, and the
// `data-expanded` attribute on `.comments-panel` drives the CSS
// translate. Leaving expand state to JS caused it to reset whenever
// Dioxus re-rendered the panel content (e.g., on Reply tap → thread
// drill-down) because the `.expanded` class lived outside the VDOM.

// Drag-to-resize the comments panel. The width is JS-owned because Dioxus
// doesn't set `style` on `.comments-panel` — the inline width survives
// re-renders. CSS provides `min-width: 420px` (the previous fixed width);
// here we cap the upper bound at 70% of viewport so the discussion body
// always keeps room. Listeners are attached to `document` (not the 6px
// handle) so the cursor doesn't fall off the hit-area mid-drag.
var COMMENTS_PANEL_MIN = 420;
var COMMENTS_PANEL_MAX_PCT = 0.7;

function initCommentsPanelResizer() {
  var resizer = document.getElementById("comments-panel-resizer");
  if (!resizer || resizer.dataset.resizerBound) return;
  var panel = document.getElementById("discussion-comments-sheet");
  if (!panel) return;
  resizer.dataset.resizerBound = "true";

  var dragging = false;
  var startX = 0;
  var startWidth = 0;

  function onPointerMove(e) {
    if (!dragging) return;
    var clientX = e.clientX !== undefined ? e.clientX : e.touches[0].clientX;
    // Panel is on the right; dragging left (smaller clientX) widens it.
    var deltaX = startX - clientX;
    var newWidth = startWidth + deltaX;
    var maxWidth = window.innerWidth * COMMENTS_PANEL_MAX_PCT;
    if (newWidth < COMMENTS_PANEL_MIN) newWidth = COMMENTS_PANEL_MIN;
    if (newWidth > maxWidth) newWidth = maxWidth;
    panel.style.width = newWidth + "px";
  }

  function onPointerUp() {
    if (!dragging) return;
    dragging = false;
    resizer.classList.remove("dragging");
    document.body.classList.remove("comments-panel-resizing");
  }

  function onPointerDown(e) {
    e.preventDefault();
    dragging = true;
    startX = e.clientX !== undefined ? e.clientX : e.touches[0].clientX;
    startWidth = panel.getBoundingClientRect().width;
    resizer.classList.add("dragging");
    document.body.classList.add("comments-panel-resizing");
  }

  resizer.addEventListener("mousedown", onPointerDown);
  resizer.addEventListener("touchstart", onPointerDown, { passive: false });
  document.addEventListener("mousemove", onPointerMove);
  document.addEventListener("touchmove", onPointerMove, { passive: false });
  document.addEventListener("mouseup", onPointerUp);
  document.addEventListener("touchend", onPointerUp);
  document.addEventListener("touchcancel", onPointerUp);
}

function init() {
  initComposerAutogrow();
  initMentionFlip();
  initCommentsPanelResizer();
}

init();
new MutationObserver(function () {
  init();
}).observe(document.body, { childList: true, subtree: true });
