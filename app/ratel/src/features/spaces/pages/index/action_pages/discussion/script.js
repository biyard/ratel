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

function init() {
  initComposerAutogrow();
  initMentionFlip();
}

init();
new MutationObserver(function () {
  init();
}).observe(document.body, { childList: true, subtree: true });
