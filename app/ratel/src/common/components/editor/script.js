// Ratel rich-text editor wiring.
//
// Each `.ratel-editor` root is initialized once. We use a MutationObserver
// to catch CSR navigations where the editor mounts after this script runs,
// and the `data-bound="true"` flag prevents double-initialization across
// re-renders.
//
// Communication with the surrounding Dioxus component:
//   - The component renders the initial HTML directly into `.re-content`
//     via RSX `dangerous_inner_html` so JS doesn't need to touch
//     editor markup at boot.
//   - On every (debounced, IME-safe) input we dispatch a `change`
//     CustomEvent on the root with the latest HTML in `detail`. Dioxus
//     subscribes via `onchange` on the root div.

(function () {
  function init(root) {
    if (!root || root.dataset.bound === "true") return;
    root.dataset.bound = "true";

    var editor = root.querySelector(".re-content");
    if (!editor) return;
    var wordCountEl = root.querySelector(".re-word-count");
    var charCountEl = root.querySelector(".re-char-count");
    var bridge = root.querySelector(".re-bridge");

    // Placeholder + editable state are owned by the Dioxus prop layer
    // and forwarded via data attributes on the root.
    var placeholder = root.dataset.placeholder || "Type here\u2026";
    editor.setAttribute("data-placeholder", placeholder);
    var editable = root.dataset.editable !== "false";
    editor.setAttribute("contenteditable", editable ? "true" : "false");
    refreshEmptyState();

    // Viewer mode skips all toolbar/modal/IME wiring — those DOM nodes
    // are not rendered when `editable=false`, so querying them would
    // throw. Read-only surfaces just need contenteditable=false and the
    // empty-state flag set above.
    if (!editable) return;

    // ── IME-safe input handling ────────────────────────────
    var composing = false;
    editor.addEventListener("compositionstart", function () {
      composing = true;
      root.setAttribute("data-composing", "true");
    });
    editor.addEventListener("compositionend", function () {
      composing = false;
      root.setAttribute("data-composing", "false");
      scheduleUpdate();
    });

    var updateTimer = null;
    function scheduleUpdate() {
      if (composing) return;
      clearTimeout(updateTimer);
      updateTimer = setTimeout(emitChange, 120);
    }

    function refreshEmptyState() {
      var empty = editor.textContent.trim().length === 0 && editor.children.length <= 1;
      editor.dataset.empty = empty ? "true" : "false";
      return empty;
    }

    function emitChange() {
      var empty = refreshEmptyState();
      var html = empty ? "" : editor.innerHTML;
      var text = editor.innerText || "";
      var words = text.trim() ? text.trim().split(/\s+/).length : 0;
      if (wordCountEl) wordCountEl.textContent = words;
      if (charCountEl) charCountEl.textContent = text.length;
      // Bridge to Dioxus via a real <input>: dispatching `input` on a
      // text input is the only event Dioxus serializes consistently
      // across web, mobile, and desktop targets.
      if (bridge) {
        bridge.value = html;
        bridge.dispatchEvent(new Event("input", { bubbles: true }));
      }
      syncToolbarState();
    }

    editor.addEventListener("input", scheduleUpdate);
    editor.addEventListener("paste", function () { setTimeout(scheduleUpdate, 0); });

    // ── Toolbar wiring ─────────────────────────────────────
    function applyCmd(cmd, value) {
      editor.focus();
      document.execCommand(cmd, false, value || null);
      scheduleUpdate();
    }

    root.querySelectorAll(".re-tb-btn").forEach(function (btn) {
      var cmd = btn.dataset.cmd;
      btn.addEventListener("click", function (e) {
        e.preventDefault();
        handleCommand(cmd);
      });
    });

    function handleCommand(cmd) {
      switch (cmd) {
        case "code-inline": return wrapInlineCode();
        case "link":        return openModal("link");
        case "unlink":      return applyCmd("unlink");
        case "image":       return openModal("image");
        case "youtube":     return openModal("youtube");
        case "table":       return openModal("table");
        case "hr":          return applyCmd("insertHorizontalRule");
        default:            return applyCmd(cmd);
      }
    }

    function wrapInlineCode() {
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return;
      var range = sel.getRangeAt(0);
      if (range.collapsed) return;
      var code = document.createElement("code");
      code.appendChild(range.extractContents());
      range.insertNode(code);
      sel.removeAllRanges();
      var after = document.createRange();
      after.setStartAfter(code);
      sel.addRange(after);
      scheduleUpdate();
    }

    // ── Block-format dropdown(s) ──────────────────────────────
    // There may be more than one `.re-block` instance: one in the static
    // top toolbar, one in the selection-triggered bubble. Each gets its
    // own open/close state but they share the active-tag highlight via
    // `syncBlockLabel`.
    var blockLabels = {
      P: "Paragraph",
      H1: "Heading 1",
      H2: "Heading 2",
      H3: "Heading 3",
      BLOCKQUOTE: "Quote",
      PRE: "Code block"
    };

    var blockDropdowns = Array.prototype.slice.call(root.querySelectorAll(".re-block"));
    blockDropdowns.forEach(function (blockDropdown) {
      var blockBtn = blockDropdown.querySelector(".re-block__btn");
      if (!blockBtn) return;
      blockBtn.addEventListener("click", function (e) {
        e.preventDefault();
        e.stopPropagation();
        var open = blockDropdown.dataset.open === "true";
        blockDropdown.dataset.open = open ? "false" : "true";
        blockBtn.setAttribute("aria-expanded", open ? "false" : "true");
      });
      blockDropdown.addEventListener("mousedown", function () {
        savedRange = saveSelection();
      });
      blockDropdown.querySelectorAll("[data-block]").forEach(function (item) {
        item.addEventListener("click", function (e) {
          e.preventDefault();
          var tag = item.dataset.block;
          blockDropdown.dataset.open = "false";
          blockBtn.setAttribute("aria-expanded", "false");
          restoreSelection();
          applyCmd("formatBlock", "<" + tag + ">");
        });
      });
    });
    document.addEventListener("click", function (e) {
      blockDropdowns.forEach(function (blockDropdown) {
        if (blockDropdown.contains(e.target)) return;
        blockDropdown.dataset.open = "false";
        var btn = blockDropdown.querySelector(".re-block__btn");
        if (btn) btn.setAttribute("aria-expanded", "false");
      });
    });

    function syncBlockLabel() {
      if (document.activeElement !== editor) return;
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return;
      var node = sel.getRangeAt(0).startContainer;
      if (node.nodeType === 3) node = node.parentNode;
      var foundTag = null;
      while (node && node !== editor) {
        if (blockLabels[node.nodeName]) { foundTag = node.nodeName; break; }
        node = node.parentNode;
      }
      var label = foundTag ? blockLabels[foundTag] : "Paragraph";
      root.querySelectorAll(".re-block__label").forEach(function (el) {
        el.textContent = label;
      });
      root.querySelectorAll("[data-block]").forEach(function (item) {
        item.classList.toggle(
          "re-block__item--active",
          item.dataset.block === foundTag
        );
      });
    }

    // ── Toolbar active-state sync ──────────────────────────
    var queryMap = {
      bold: "bold",
      italic: "italic",
      underline: "underline",
      strikeThrough: "strikeThrough",
      insertUnorderedList: "insertUnorderedList",
      insertOrderedList: "insertOrderedList",
      justifyLeft: "justifyLeft",
      justifyCenter: "justifyCenter",
      justifyRight: "justifyRight",
      justifyFull: "justifyFull"
    };
    function syncToolbarState() {
      root.querySelectorAll(".re-tb-btn").forEach(function (btn) {
        var name = queryMap[btn.dataset.cmd];
        if (!name) return;
        try {
          btn.setAttribute(
            "aria-pressed",
            document.queryCommandState(name) ? "true" : "false"
          );
        } catch (_e) { /* unsupported in some browsers */ }
      });
    }
    document.addEventListener("selectionchange", function () {
      if (document.activeElement === editor) {
        syncToolbarState();
        syncBlockLabel();
      }
    });

    // ── Modals ──────────────────────────────────────────────
    var savedRange = null;
    function saveSelection() {
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return null;
      return sel.getRangeAt(0).cloneRange();
    }
    function restoreSelection() {
      if (!savedRange) { editor.focus(); return; }
      editor.focus();
      var sel = window.getSelection();
      sel.removeAllRanges();
      sel.addRange(savedRange);
    }
    function openModal(name) {
      savedRange = saveSelection();
      var mask = root.querySelector("[data-modal='" + name + "']");
      if (!mask) return;
      mask.classList.add("open");
      var firstInput = mask.querySelector("input");
      if (firstInput) setTimeout(function () { firstInput.focus(); }, 30);
    }
    function closeModal(mask) { mask.classList.remove("open"); }

    root.querySelectorAll("[data-close-modal]").forEach(function (btn) {
      btn.addEventListener("click", function () {
        closeModal(btn.closest(".re-modal-mask"));
      });
    });
    root.querySelectorAll(".re-modal-mask").forEach(function (mask) {
      mask.addEventListener("click", function (e) {
        if (e.target === mask) closeModal(mask);
      });
    });

    var linkMask = root.querySelector("[data-modal='link']");
    var linkUrl = linkMask.querySelector(".re-link-url");
    linkMask.querySelector(".re-link-confirm").addEventListener("click", function () {
      var url = linkUrl.value.trim();
      closeModal(linkMask);
      linkUrl.value = "";
      if (!url) return;
      restoreSelection();
      applyCmd("createLink", url);
    });

    var imageMask = root.querySelector("[data-modal='image']");
    var imageUrl = imageMask.querySelector(".re-image-url");
    var imageFile = imageMask.querySelector(".re-image-file");
    var imageCamera = imageMask.querySelector(".re-image-camera");
    var dropzone = imageMask.querySelector(".re-dropzone");

    function insertFile(file) {
      if (!file || !file.type || file.type.indexOf("image/") !== 0) return;
      var reader = new FileReader();
      reader.onload = function (ev) {
        closeModal(imageMask);
        imageUrl.value = "";
        if (imageFile) imageFile.value = "";
        if (imageCamera) imageCamera.value = "";
        restoreSelection();
        applyCmd("insertImage", ev.target.result);
      };
      reader.readAsDataURL(file);
    }

    // Click-to-upload + camera-shortcut both flow through `change`.
    imageFile.addEventListener("change", function () {
      if (imageFile.files && imageFile.files[0]) insertFile(imageFile.files[0]);
    });
    if (imageCamera) {
      imageCamera.addEventListener("change", function () {
        if (imageCamera.files && imageCamera.files[0]) insertFile(imageCamera.files[0]);
      });
    }

    // Drag & drop. We have to swallow `dragover` (preventDefault) for the
    // browser to let us listen to `drop` at all.
    if (dropzone) {
      ["dragenter", "dragover"].forEach(function (evt) {
        dropzone.addEventListener(evt, function (e) {
          e.preventDefault();
          e.stopPropagation();
          dropzone.dataset.dragging = "true";
        });
      });
      ["dragleave", "dragend"].forEach(function (evt) {
        dropzone.addEventListener(evt, function (e) {
          e.preventDefault();
          e.stopPropagation();
          dropzone.dataset.dragging = "false";
        });
      });
      dropzone.addEventListener("drop", function (e) {
        e.preventDefault();
        e.stopPropagation();
        dropzone.dataset.dragging = "false";
        var file = e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files[0];
        if (file) insertFile(file);
      });
    }

    // The "Insert URL" button only handles the URL path now; file
    // selection inserts immediately via the change handler above.
    imageMask.querySelector(".re-image-confirm").addEventListener("click", function () {
      var url = imageUrl.value.trim();
      closeModal(imageMask);
      imageUrl.value = "";
      if (!url) return;
      restoreSelection();
      applyCmd("insertImage", url);
    });

    var youtubeMask = root.querySelector("[data-modal='youtube']");
    var youtubeUrl = youtubeMask.querySelector(".re-youtube-url");
    youtubeMask.querySelector(".re-youtube-confirm").addEventListener("click", function () {
      var raw = youtubeUrl.value.trim();
      closeModal(youtubeMask);
      youtubeUrl.value = "";
      if (!raw) return;
      var id = parseYoutubeId(raw);
      if (!id) return;
      var html = '<div class="yt-wrap"><iframe src="https://www.youtube.com/embed/' +
        id + '" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe></div><p><br></p>';
      restoreSelection();
      document.execCommand("insertHTML", false, html);
      scheduleUpdate();
    });
    function parseYoutubeId(input) {
      var m = input.match(/(?:youtu\.be\/|v=|\/embed\/)([\w-]{11})/);
      if (m) return m[1];
      if (/^[\w-]{11}$/.test(input)) return input;
      return null;
    }

    var tableMask = root.querySelector("[data-modal='table']");
    var tableRows = tableMask.querySelector(".re-table-rows");
    var tableCols = tableMask.querySelector(".re-table-cols");
    var tableHeader = tableMask.querySelector(".re-table-header");
    tableMask.querySelector(".re-table-confirm").addEventListener("click", function () {
      var rows = parseInt(tableRows.value, 10) || 1;
      var cols = parseInt(tableCols.value, 10) || 1;
      var useHeader = tableHeader.checked;
      closeModal(tableMask);
      var html = "<table>";
      for (var r = 0; r < rows; r++) {
        html += "<tr>";
        for (var c = 0; c < cols; c++) {
          var cell = useHeader && r === 0 ? "th" : "td";
          html += "<" + cell + ">&nbsp;</" + cell + ">";
        }
        html += "</tr>";
      }
      html += "</table><p><br></p>";
      restoreSelection();
      document.execCommand("insertHTML", false, html);
      scheduleUpdate();
    });

    // ── Keyboard shortcuts ─────────────────────────────────
    editor.addEventListener("keydown", function (e) {
      var meta = e.metaKey || e.ctrlKey;
      if (!meta) return;
      var k = e.key.toLowerCase();
      if (k === "b") { e.preventDefault(); applyCmd("bold"); }
      else if (k === "i") { e.preventDefault(); applyCmd("italic"); }
      else if (k === "u") { e.preventDefault(); applyCmd("underline"); }
      else if (k === "k") { e.preventDefault(); openModal("link"); }
    });

    // ── Selection-triggered bubble toolbar (desktop only) ─────
    // On coarse-pointer (touch) devices the OS provides its own selection
    // menu (Copy / Paste / Look up). Layering our bubble on top would
    // conflict and the static top toolbar remains available, so we skip
    // wiring entirely on touch. The bubble's DOM is rendered regardless;
    // it just stays invisible.
    var bubble = root.querySelector(".re-bubble");
    var coarsePointer = typeof window.matchMedia === "function" &&
      window.matchMedia("(pointer: coarse)").matches;

    if (bubble && !coarsePointer) {
      // Preserving the editor's selection across a bubble button click is
      // the whole point of this toolbar. Without preventDefault here, the
      // mousedown moves focus from the editor to the <button>, the
      // selection collapses, and execCommand has nothing left to act on.
      bubble.addEventListener("mousedown", function (e) {
        e.preventDefault();
      });

      function isFocusInsideEditor() {
        return document.activeElement === editor ||
          bubble.contains(document.activeElement);
      }

      function positionBubble(range) {
        var sr = range.getBoundingClientRect();
        if (sr.width === 0 && sr.height === 0) return false;
        var bw = bubble.offsetWidth;
        var bh = bubble.offsetHeight;
        // Position uses viewport coordinates (position: fixed) so the
        // editor's `overflow: hidden` never clips the bubble.
        var top = sr.bottom + 8;
        if (top + bh + 8 > window.innerHeight) {
          top = sr.top - bh - 8; // auto-flip: place above the selection
        }
        var left = sr.left + sr.width / 2 - bw / 2;
        left = Math.max(8, Math.min(left, window.innerWidth - bw - 8));
        bubble.style.top = top + "px";
        bubble.style.left = left + "px";
        return true;
      }

      function hideBubble() {
        if (bubble.dataset.visible !== "true") return;
        bubble.dataset.visible = "false";
        // Close any open block dropdown owned by the bubble.
        var inner = bubble.querySelector(".re-block");
        if (inner && inner.dataset.open === "true") {
          inner.dataset.open = "false";
          var btn = inner.querySelector(".re-block__btn");
          if (btn) btn.setAttribute("aria-expanded", "false");
        }
      }

      function updateBubble() {
        if (composing) return hideBubble();
        if (!isFocusInsideEditor()) return hideBubble();
        var sel = window.getSelection();
        if (!sel || sel.rangeCount === 0 || sel.isCollapsed) return hideBubble();
        var range = sel.getRangeAt(0);
        if (!editor.contains(range.commonAncestorContainer)) return hideBubble();
        if (!positionBubble(range)) return hideBubble();
        bubble.dataset.visible = "true";
      }

      // selectionchange fires many times during a drag; rAF-throttle to
      // one update per frame so we don't thrash layout.
      var bubbleFrame = null;
      function scheduleBubbleUpdate() {
        if (bubbleFrame !== null) return;
        bubbleFrame = requestAnimationFrame(function () {
          bubbleFrame = null;
          updateBubble();
        });
      }

      editor.addEventListener("mouseup", scheduleBubbleUpdate);
      editor.addEventListener("keyup", scheduleBubbleUpdate);
      document.addEventListener("selectionchange", scheduleBubbleUpdate);
      // Hide on scroll/resize rather than reposition — keeps the bubble
      // anchored to a stable selection rect. Capture: true so we also
      // catch scrolls inside arbitrary scroll containers above the editor.
      window.addEventListener("scroll", hideBubble, { passive: true, capture: true });
      window.addEventListener("resize", hideBubble);
      document.addEventListener("keydown", function (e) {
        if (e.key === "Escape" && bubble.dataset.visible === "true") {
          hideBubble();
          editor.focus();
        }
      });
    }

    // ── Markdown shortcuts ─────────────────────────────────
    // Notion-style block conversions triggered by typing a marker + space at
    // the start of a block. Sibling lifecycle: lives inside init(root) so it
    // shares `composing`, `editor`, `scheduleUpdate` with the rest of the
    // editor. See docs/superpowers/specs/2026-05-21-editor-markdown-shortcuts-design.md.

    var BLOCK_TAGS = ["P", "DIV", "H1", "H2", "H3", "H4", "H5", "H6", "BLOCKQUOTE", "PRE", "LI"];
    var SKIP_BLOCK_TAGS = ["H1", "H2", "H3", "H4", "H5", "H6", "PRE", "BLOCKQUOTE"];
    // U+FEFF (BOM/zero-width no-break space) is invisible and wouldn't be
    // typed by a user, so wrapping the unique tag with it guarantees we can
    // round-trip it through innerHTML restoration without colliding with
    // real content.
    var REVERT_SENTINEL = "﻿__RATEL_REVERT_SENTINEL__﻿";

    var lastConversion = null;

    function mdGetCaretBlock() {
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return null;
      var node = sel.getRangeAt(0).startContainer;
      if (node.nodeType === 3) node = node.parentNode;
      while (node && node !== editor) {
        if (BLOCK_TAGS.indexOf(node.nodeName) >= 0) return node;
        node = node.parentNode;
      }
      return editor;
    }

    function mdAncestorTag(tag) {
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return null;
      var node = sel.getRangeAt(0).startContainer;
      if (node.nodeType === 3) node = node.parentNode;
      while (node && node !== editor) {
        if (node.nodeName === tag) return node;
        node = node.parentNode;
      }
      return null;
    }

    function mdHasAncestorTag(tag) {
      return mdAncestorTag(tag) !== null;
    }

    function mdTextBeforeCaretInBlock(blockEl) {
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return "";
      var range = sel.getRangeAt(0);
      var clone = range.cloneRange();
      clone.selectNodeContents(blockEl);
      clone.setEnd(range.startContainer, range.startOffset);
      return clone.toString();
    }

    function mdMatchBlockMarker(text) {
      // Browsers insert U+00A0 (NBSP) instead of a regular space when the
      // space would otherwise collapse at the end of inline content in a
      // contentEditable. Normalize before matching so /^# $/ catches both.
      // The marker length stays the same — NBSP is one Unicode codepoint
      // just like a regular space.
      text = text.replace(/ /g, " ");
      // Longest patterns first so /^### / wins over /^# /.
      if (/^### $/.test(text)) return { kind: "heading", level: 3, markerLen: 4 };
      if (/^## $/.test(text)) return { kind: "heading", level: 2, markerLen: 3 };
      if (/^# $/.test(text)) return { kind: "heading", level: 1, markerLen: 2 };
      if (/^[\*\-\+] $/.test(text)) return { kind: "ulist", markerLen: 2 };
      var m = text.match(/^(\d+)\. $/);
      if (m) return { kind: "olist", markerLen: m[0].length };
      if (/^> $/.test(text)) return { kind: "blockquote", markerLen: 2 };
      if (/^(---|\*\*\*) $/.test(text)) return { kind: "hr", markerLen: 4 };
      return null;
    }

    function mdDeleteFirstChars(blockEl, count) {
      // Walk forward from the start of blockEl, deleting up to `count`
      // characters of visible text. Empty text nodes are LEFT IN PLACE on
      // purpose: removing the walker's current node and then calling
      // nextNode() is undefined behavior per the DOM spec. Empty text
      // nodes are invisible to the user and rendering passes (the editor's
      // own debounced emitChange + browser layout) will collapse them.
      var walker = document.createTreeWalker(blockEl, NodeFilter.SHOW_TEXT, null);
      var node;
      var remaining = count;
      while ((node = walker.nextNode())) {
        if (node.length === 0) continue;
        var take = node.length < remaining ? node.length : remaining;
        node.deleteData(0, take);
        remaining -= take;
        if (remaining === 0) return true;
      }
      return false;
    }

    function mdPlaceCaretAtBlockStart(blockEl) {
      var walker = document.createTreeWalker(blockEl, NodeFilter.SHOW_TEXT, null);
      var firstText = walker.nextNode();
      var sel = window.getSelection();
      sel.removeAllRanges();
      var caret = document.createRange();
      if (firstText) {
        caret.setStart(firstText, 0);
      } else {
        caret.setStart(blockEl, 0);
      }
      caret.collapse(true);
      sel.addRange(caret);
    }

    function mdSnapshotForRevert(markerText) {
      // Insert a sentinel text node at the current caret so we can locate
      // and remove it after restoring innerHTML.
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return null;
      var range = sel.getRangeAt(0);
      var sentinel = document.createTextNode(REVERT_SENTINEL);
      // NOTE: insertNode splits the caret's text node. We immediately remove
      // the sentinel, but the split halves remain. This is fine because every
      // caller proceeds to mutate the block (formatBlock, indent, etc.) which
      // serializes innerHTML again. Do not call this function on a code path
      // that may not perform a follow-up mutation, or you'll leak the split.
      range.insertNode(sentinel);
      var html = editor.innerHTML;
      sentinel.parentNode.removeChild(sentinel);
      return { markerText: markerText, snapshot: html };
    }

    function mdRevert(c) {
      editor.innerHTML = c.snapshot;
      var walker = document.createTreeWalker(editor, NodeFilter.SHOW_TEXT, null);
      var node;
      while ((node = walker.nextNode())) {
        var idx = node.nodeValue.indexOf(REVERT_SENTINEL);
        if (idx >= 0) {
          var before = node.nodeValue.slice(0, idx);
          var after = node.nodeValue.slice(idx + REVERT_SENTINEL.length);
          node.nodeValue = before + after;
          var sel = window.getSelection();
          sel.removeAllRanges();
          var caret = document.createRange();
          caret.setStart(node, idx);
          caret.collapse(true);
          sel.addRange(caret);
          return;
        }
      }
      editor.focus();
    }

    function mdIsLiEmpty(li) {
      // Empty if no visible text and no nested list. A leftover <br> is fine.
      // Zero-width space (U+200B) is also stripped because some browsers insert
      // it as a placeholder when an <li> is created empty.
      if (li.textContent.replace(/​/g, "").trim() !== "") return false;
      if (li.querySelector("ul, ol")) return false;
      return true;
    }

    function mdTryConvert(inputEvent) {
      var block = mdGetCaretBlock();
      if (!block) return false;
      // block === editor happens when the caret is in bare text directly
      // inside the .re-content root — Chrome leaves the very first line of
      // a fresh contenteditable unwrapped until the user presses Enter.
      // We still want conversion in that case; formatBlock / insertList
      // will wrap the current line into the target tag. The SKIP_BLOCK_TAGS
      // guard below only applies to nested block elements, not the editor
      // itself, so it's harmless when block === editor.
      if (SKIP_BLOCK_TAGS.indexOf(block.nodeName) >= 0) return false;
      // Inside a <pre> ancestor anywhere up the tree? Skip.
      if (mdHasAncestorTag("PRE")) return false;
      // Inside an existing <li>? Skip — list nesting is driven by Tab, not
      // by re-typing a bullet marker inside a list item.
      if (mdHasAncestorTag("LI")) return false;

      var before = mdTextBeforeCaretInBlock(block);
      var info = mdMatchBlockMarker(before);
      if (!info) return false;

      var snap = mdSnapshotForRevert(before);

      // Bare text directly inside the editor root: do the whole conversion
      // by direct DOM manipulation. execCommand("formatBlock") + caret
      // dance is unreliable when the source has no block wrapper — Chrome
      // creates the new element but leaves the caret at the editor root,
      // and formatBlock on an empty <p> won't transform it.
      if (block === editor) {
        var caretRange = window.getSelection().getRangeAt(0);
        var caretNode = caretRange.startContainer;
        if (caretNode.nodeType !== 3 || caretNode.parentNode !== editor) {
          mdRevert(snap);
          return false;
        }
        // Read the surviving text (after the marker) and detach the original
        // bare text node. We rebuild the block from scratch with either the
        // surviving text or a <br> placeholder — Chrome's editor engine
        // refuses to type into an empty text node inside a fresh element, so
        // an empty <h1></h1> would cause subsequent typing to land outside.
        var fullText = caretNode.data;
        var survivingText = fullText.slice(info.markerLen);
        var nextSibling = caretNode.nextSibling;
        if (caretNode.parentNode) caretNode.parentNode.removeChild(caretNode);

        function innerContentNode() {
          return survivingText.length > 0
            ? document.createTextNode(survivingText)
            : document.createElement("br");
        }

        var newBlock;
        var caretAnchor;  // node the caret should land in
        if (info.kind === "heading") {
          newBlock = document.createElement("h" + info.level);
          caretAnchor = innerContentNode();
          newBlock.appendChild(caretAnchor);
        } else if (info.kind === "ulist" || info.kind === "olist") {
          newBlock = document.createElement(info.kind === "ulist" ? "ul" : "ol");
          var li = document.createElement("li");
          caretAnchor = innerContentNode();
          li.appendChild(caretAnchor);
          newBlock.appendChild(li);
        } else if (info.kind === "blockquote") {
          newBlock = document.createElement("blockquote");
          caretAnchor = innerContentNode();
          newBlock.appendChild(caretAnchor);
        } else if (info.kind === "hr") {
          // <hr> + follow-up <p><br></p> for the user to type into.
          newBlock = document.createElement("hr");
          editor.insertBefore(newBlock, nextSibling);
          var followP = document.createElement("p");
          followP.appendChild(document.createElement("br"));
          editor.insertBefore(followP, newBlock.nextSibling);
          var hrCaret = document.createRange();
          hrCaret.setStart(followP, 0);
          hrCaret.collapse(true);
          var hrSel = window.getSelection();
          hrSel.removeAllRanges();
          hrSel.addRange(hrCaret);
          lastConversion = snap;
          scheduleUpdate();
          return true;
        } else {
          mdRevert(snap);
          return false;
        }
        editor.insertBefore(newBlock, nextSibling);
        var startCaret = document.createRange();
        if (caretAnchor.nodeType === 3) {
          // Text-node anchor: caret at offset 0 of the text node.
          startCaret.setStart(caretAnchor, 0);
        } else {
          // <br> placeholder: caret at child index 0 of its parent, which
          // is the position immediately before the <br>.
          startCaret.setStart(caretAnchor.parentNode, 0);
        }
        startCaret.collapse(true);
        var startSel = window.getSelection();
        startSel.removeAllRanges();
        startSel.addRange(startCaret);
        lastConversion = snap;
        scheduleUpdate();
        return true;
      }

      // Block-wrapped path. Strip the marker chars from the block's text,
      // then build the target block via direct DOM and replace the existing
      // block. We avoid execCommand here because nested execCommand calls
      // (this handler was reached via a space `insertText` execCommand from
      // the browser's own input pipeline) can silently no-op in Chrome.
      if (!mdDeleteFirstChars(block, info.markerLen)) {
        mdRevert(snap);
        return false;
      }

      // If the block is now empty (only an empty text node or nothing),
      // give it a <br> placeholder so Chrome's editor lets the user type
      // into it after the conversion.
      var hasContent = false;
      for (var ci = 0; ci < block.childNodes.length; ci++) {
        var cn = block.childNodes[ci];
        if (cn.nodeType === 3 && cn.data.length > 0) { hasContent = true; break; }
        if (cn.nodeType === 1) { hasContent = true; break; }
      }
      if (!hasContent) {
        while (block.firstChild) block.removeChild(block.firstChild);
        block.appendChild(document.createElement("br"));
      }

      var newBlock;
      var caretAnchor;
      if (info.kind === "heading") {
        newBlock = document.createElement("h" + info.level);
        while (block.firstChild) newBlock.appendChild(block.firstChild);
        caretAnchor = newBlock;
      } else if (info.kind === "ulist" || info.kind === "olist") {
        newBlock = document.createElement(info.kind === "ulist" ? "ul" : "ol");
        var liNew = document.createElement("li");
        while (block.firstChild) liNew.appendChild(block.firstChild);
        newBlock.appendChild(liNew);
        caretAnchor = liNew;
      } else if (info.kind === "blockquote") {
        newBlock = document.createElement("blockquote");
        while (block.firstChild) newBlock.appendChild(block.firstChild);
        caretAnchor = newBlock;
      } else if (info.kind === "hr") {
        // Replace the block with <hr> + an empty <p> for typing.
        var hrParent = block.parentNode;
        var hrNextSib = block.nextSibling;
        hrParent.removeChild(block);
        var hrEl = document.createElement("hr");
        var followP = document.createElement("p");
        followP.appendChild(document.createElement("br"));
        hrParent.insertBefore(hrEl, hrNextSib);
        hrParent.insertBefore(followP, hrEl.nextSibling);
        var hrCaret = document.createRange();
        hrCaret.setStart(followP, 0);
        hrCaret.collapse(true);
        var hrSel = window.getSelection();
        hrSel.removeAllRanges();
        hrSel.addRange(hrCaret);
        lastConversion = snap;
        scheduleUpdate();
        return true;
      } else {
        mdRevert(snap);
        return false;
      }

      // Replace the old block with the new one and place the caret at the
      // start of the surviving content.
      block.parentNode.replaceChild(newBlock, block);
      var anchorWalker = document.createTreeWalker(caretAnchor, NodeFilter.SHOW_TEXT, null);
      var anchorText = anchorWalker.nextNode();
      var startCaret = document.createRange();
      if (anchorText) {
        startCaret.setStart(anchorText, 0);
      } else {
        startCaret.setStart(caretAnchor, 0);
      }
      startCaret.collapse(true);
      var startSel = window.getSelection();
      startSel.removeAllRanges();
      startSel.addRange(startCaret);

      lastConversion = snap;
      scheduleUpdate();
      return true;
    }

    editor.addEventListener("input", function (e) {
      if (composing) return;
      if (e.inputType === "insertText" && e.data === " ") {
        if (mdTryConvert(e)) return;
      }
      // Any input that wasn't a successful conversion disarms revert.
      lastConversion = null;
    });

    editor.addEventListener("keydown", function (e) {
      // IME composition: never fire while a CJK candidate is being committed
      // (e.g. Enter is used to commit Korean syllables; intercepting it would
      // corrupt the composition).
      if (composing) return;

      // Backspace immediately after a conversion → revert to literal marker.
      if (
        e.key === "Backspace" &&
        lastConversion &&
        !e.metaKey && !e.ctrlKey && !e.shiftKey && !e.altKey
      ) {
        e.preventDefault();
        mdRevert(lastConversion);
        lastConversion = null;
        scheduleUpdate();
        return;
      }

      // Any non-Backspace, non-modifier-only key disarms the revert window.
      // Modifier-only events (Shift, Ctrl, Alt, Meta, Process for IME) must
      // NOT disarm — the user might be on the way to a real combo.
      var isModifierOnly =
        e.key === "Shift" ||
        e.key === "Control" ||
        e.key === "Alt" ||
        e.key === "Meta" ||
        e.key === "Process";
      if (e.key !== "Backspace" && !isModifierOnly) {
        lastConversion = null;
      }

      // Tab / Shift+Tab inside a list item: nest deeper / un-nest.
      if (e.key === "Tab" && !e.metaKey && !e.ctrlKey && !e.altKey) {
        if (mdAncestorTag("LI")) {
          e.preventDefault();
          if (e.shiftKey) document.execCommand("outdent", false);
          else document.execCommand("indent", false);
          scheduleUpdate();
          return;
        }
      }

      // Enter on an empty <li>: exit the list (Notion-style).
      if (e.key === "Enter" && !e.shiftKey && !e.metaKey && !e.ctrlKey && !e.altKey) {
        var li = mdAncestorTag("LI");
        if (li && mdIsLiEmpty(li)) {
          e.preventDefault();
          document.execCommand("outdent", false);
          // If outdent left us in another list (some browsers leave nested
          // wrappers behind), force a paragraph.
          if (mdAncestorTag("LI")) {
            document.execCommand("formatBlock", false, "<P>");
          }
          scheduleUpdate();
          return;
        }
      }

      // ``` + Enter → <pre>. Trigger is Enter (not space) because the marker
      // is followed by a newline-to-code-block, not a space-to-text.
      // Always followed by an empty paragraph so the user can exit the pre
      // by arrow-down or by clicking below it.
      if (e.key === "Enter" && !e.shiftKey && !e.metaKey && !e.ctrlKey && !e.altKey) {
        var fenceBlock = mdGetCaretBlock();
        if (fenceBlock && !mdHasAncestorTag("PRE")) {
          // Helper: install a <pre><br></pre><p><br></p> pair at the given
          // anchor (either replacing a node or inserting at editor root),
          // placing the caret inside the <pre> ready for code.
          function fenceInstall(replaceNode, parent, nextSibling) {
            var pre = document.createElement("pre");
            pre.appendChild(document.createElement("br"));
            var followP = document.createElement("p");
            followP.appendChild(document.createElement("br"));
            if (replaceNode) {
              parent.replaceChild(pre, replaceNode);
            } else {
              parent.insertBefore(pre, nextSibling);
            }
            parent.insertBefore(followP, pre.nextSibling);
            var preCaret = document.createRange();
            preCaret.setStart(pre, 0);
            preCaret.collapse(true);
            var preSel = window.getSelection();
            preSel.removeAllRanges();
            preSel.addRange(preCaret);
          }

          // Bare text at editor root: the editor's only content is a single
          // bare text node "```".
          if (fenceBlock === editor && editor.textContent === "```") {
            var onlyBareFence = false;
            var bareNode = null;
            for (var ci = 0; ci < editor.childNodes.length; ci++) {
              var cn = editor.childNodes[ci];
              if (cn.nodeType === 3 && cn.data === "```") {
                if (bareNode) { bareNode = null; break; }
                bareNode = cn;
                onlyBareFence = true;
              } else if (cn.nodeType === 1) {
                onlyBareFence = false;
                break;
              }
            }
            if (onlyBareFence && bareNode) {
              e.preventDefault();
              var snap = mdSnapshotForRevert("```");
              fenceInstall(bareNode, editor, null);
              lastConversion = snap;
              scheduleUpdate();
              return;
            }
          }
          // Block-wrapped case: <p>```</p> + Enter. Replace the whole block
          // with <pre><br></pre><p><br></p>.
          if (fenceBlock !== editor && fenceBlock.textContent === "```") {
            e.preventDefault();
            var snap2 = mdSnapshotForRevert("```");
            fenceInstall(fenceBlock, fenceBlock.parentNode, null);
            lastConversion = snap2;
            scheduleUpdate();
            return;
          }
        }
      }
    });

    // Initial paint so word/char counts and toolbar state are correct.
    emitChange();
  }

  function initAll() {
    document.querySelectorAll(".ratel-editor").forEach(init);
  }
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initAll);
  } else {
    initAll();
  }
  // CSR: components mount after this script runs, so observe the body
  // and re-scan on every mutation.
  new MutationObserver(initAll).observe(document.body, {
    childList: true,
    subtree: true
  });
})();
