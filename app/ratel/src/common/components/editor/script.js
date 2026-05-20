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

    // ── Slash-command watcher (opt-in) ─────────────────────
    // Fires whenever the caret-prefixing token matches `/<word>`. The
    // consumer decides what's a valid command (e.g. /data) — the
    // watcher just surfaces the raw token and caret position.
    var slashBridge = root.querySelector(".re-slash-bridge");
    var slashEnabled = root.dataset.slash === "true" && !!slashBridge;
    var lastSlashRaw = "";
    function emitSlash(payload) {
      if (!slashBridge) return;
      var v = payload ? JSON.stringify(payload) : "";
      if (v === slashBridge.value) return;
      slashBridge.value = v;
      slashBridge.dispatchEvent(new Event("input", { bubbles: true }));
      lastSlashRaw = payload ? payload.raw : "";
    }
    function clearSlash() {
      if (lastSlashRaw === "") return;
      emitSlash(null);
    }
    function parseSlashAtCaret() {
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return null;
      var range = sel.getRangeAt(0);
      if (!editor.contains(range.startContainer)) return null;
      var node = range.startContainer;
      if (node.nodeType !== 3) return null; // need a text node
      var text = node.textContent || "";
      var caret = range.startOffset;
      // Walk back from caret looking for `/<word>` with no whitespace.
      var before = text.slice(0, caret);
      // Match the last slash-prefixed token at the end of `before`.
      var m = before.match(/(^|[\s　\xa0])(\/[^\s　\xa0]*)$/);
      if (!m) return null;
      var raw = m[2];
      // Need at least one non-`/` char after `/` to make a "word" — bare
      // `/` shouldn't auto-open. Allow trailing `:` so multi-level
      // tokens like `/data:` qualify.
      if (raw.length < 2) return null;
      // Caret position — viewport-relative rect.
      var rect = range.getBoundingClientRect();
      var px = rect.left;
      var py = rect.bottom;
      var ESTIMATED_POPUP_H = 340;
      var PAD = 6;
      var spaceBelow = window.innerHeight - py;
      var placement = spaceBelow >= ESTIMATED_POPUP_H + PAD ? "below" : "above";
      var y = placement === "below" ? py + PAD : rect.top - PAD;
      return { raw: raw, caret_x: px, caret_y: y, placement: placement };
    }
    function checkSlash() {
      if (!slashEnabled) return;
      if (composing) return; // wait for IME commit
      var payload = parseSlashAtCaret();
      if (!payload) {
        clearSlash();
        return;
      }
      emitSlash(payload);
    }
    if (slashEnabled) {
      editor.addEventListener("input", checkSlash);
      editor.addEventListener("keyup", checkSlash);
      editor.addEventListener("focusout", clearSlash);
      document.addEventListener("selectionchange", function () {
        if (document.activeElement === editor) checkSlash();
      });
    }

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
        // Drag-selecting table cells leaves an artifact text Range
        // that's non-collapsed but contains no meaningful inline text
        // (the selection straddles cell boundaries). Showing the
        // text-format bubble there is confusing — the user is picking
        // cells for a future merge, not formatting text.
        if (editor.querySelector(".re-cell-selected")) return hideBubble();
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

    // ── Table actions (insert row/col, delete row/col/table) ─────
    // Floating mini-toolbar that appears above the table the caret is
    // inside. Operations are pure DOM mutations — execCommand doesn't
    // ship anything for row/column manipulation, and these need
    // index-based logic anyway.
    var tableActions = root.querySelector(".re-table-actions");
    if (tableActions) {
      // Prevent toolbar mousedown from collapsing the caret.
      tableActions.addEventListener("mousedown", function (e) { e.preventDefault(); });

      var currentCell = null;

      function findCell(node) {
        while (node && node !== editor) {
          if (node.nodeType === 1 && (node.tagName === "TD" || node.tagName === "TH")) return node;
          node = node.parentNode;
        }
        return null;
      }

      // ── Drag-selection across cells ─────────────────────
      // Track mousedown → mousemove → mouseup inside a table and
      // mark the rectangle of cells the user is dragging across with
      // `.re-cell-selected`. The merge action consumes this set, and
      // any caret movement or click outside clears it.
      var dragAnchor = null; // first cell where mouse went down
      var dragActive = false;

      function clearCellSelection() {
        editor
          .querySelectorAll(".re-cell-selected")
          .forEach(function (c) { c.classList.remove("re-cell-selected"); });
      }

      function markSelectionRect(startCell, endCell) {
        clearCellSelection();
        if (!startCell || !endCell) return;
        var table = startCell.closest("table");
        if (!table || endCell.closest("table") !== table) return;
        var rows = Array.prototype.slice.call(table.rows);
        var s = { r: rows.indexOf(startCell.parentNode), c: Array.prototype.indexOf.call(startCell.parentNode.cells, startCell) };
        var e2 = { r: rows.indexOf(endCell.parentNode), c: Array.prototype.indexOf.call(endCell.parentNode.cells, endCell) };
        var minR = Math.min(s.r, e2.r), maxR = Math.max(s.r, e2.r);
        var minC = Math.min(s.c, e2.c), maxC = Math.max(s.c, e2.c);
        for (var rr = minR; rr <= maxR; rr++) {
          var rrow = rows[rr];
          if (!rrow) continue;
          for (var cc = minC; cc <= maxC; cc++) {
            var ce = rrow.cells[cc];
            if (ce) ce.classList.add("re-cell-selected");
          }
        }
      }

      var multiCell = false; // true once drag has crossed cell boundary

      editor.addEventListener("mousedown", function (e) {
        var cell = findCell(e.target);
        if (!cell) {
          clearCellSelection();
          dragAnchor = null;
          dragActive = false;
          multiCell = false;
          return;
        }
        dragAnchor = cell;
        dragActive = true;
        multiCell = false;
        clearCellSelection();
      });
      editor.addEventListener("mousemove", function (e) {
        if (!dragActive || !dragAnchor) return;
        var over = findCell(e.target);
        if (!over) return;
        if (over !== dragAnchor) {
          multiCell = true;
        }
        if (multiCell) {
          // Suppress the browser's native text selection: it overlays
          // our purple cell highlight and triggers the .re-bubble
          // text-format popup, which is confusing when the user is
          // really selecting cells (not text).
          //
          // We collapse the selection INSIDE the anchor cell rather
          // than removing all ranges — keeping a valid editor-bound
          // range means `document.activeElement === editor` stays true,
          // `syncTableActions` finds a current cell, and the merge
          // toolbar above the table stays visible after the user
          // releases the mouse.
          e.preventDefault();
          var sel = window.getSelection();
          if (sel) {
            var anchorRange = document.createRange();
            anchorRange.selectNodeContents(dragAnchor);
            anchorRange.collapse(true);
            sel.removeAllRanges();
            sel.addRange(anchorRange);
          }
          markSelectionRect(dragAnchor, over);
        }
      });
      // mouseup may land outside the editor (user releases over the
      // toolbar etc.), so listen on document.
      document.addEventListener("mouseup", function () {
        dragActive = false;
      });
      // Click events after a multi-cell drag fire on the COMMON
      // ANCESTOR of mousedown/mouseup targets (often the <table> or
      // <tbody>), not on a specific cell — so we can't trust them to
      // tell us whether the user is "still inside a cell". Instead,
      // clear marks only when a fresh mousedown starts somewhere
      // outside the cell grid (the editor's `mousedown` handler above
      // wipes marks at the start of every new gesture; this guards
      // taps that miss the editor entirely).
      document.addEventListener("mousedown", function (e) {
        if (editor.contains(e.target)) return;
        if (tableActions && tableActions.contains(e.target)) return;
        clearCellSelection();
      });

      function positionTableActions(cell) {
        var table = cell.closest("table");
        if (!table) return false;
        var tr = table.getBoundingClientRect();
        var pad = 6;
        var th = tableActions.offsetHeight || 36;
        var tw = tableActions.offsetWidth || 220;
        var top = tr.top - th - pad;
        // Flip below if there's no room above.
        if (top < 8) top = tr.bottom + pad;
        var left = tr.left;
        // Keep within viewport horizontally.
        left = Math.max(8, Math.min(left, window.innerWidth - tw - 8));
        tableActions.style.top = top + "px";
        tableActions.style.left = left + "px";
        return true;
      }

      function syncTableActions() {
        if (document.activeElement !== editor) {
          tableActions.dataset.visible = "false";
          currentCell = null;
          return;
        }
        var sel = window.getSelection();
        if (!sel || sel.rangeCount === 0) {
          tableActions.dataset.visible = "false";
          currentCell = null;
          return;
        }
        var cell = findCell(sel.getRangeAt(0).startContainer);
        if (!cell) {
          tableActions.dataset.visible = "false";
          currentCell = null;
          return;
        }
        currentCell = cell;
        if (positionTableActions(cell)) {
          tableActions.dataset.visible = "true";
        }
      }

      document.addEventListener("selectionchange", syncTableActions);
      window.addEventListener("scroll", syncTableActions, { passive: true, capture: true });
      window.addEventListener("resize", syncTableActions);

      function makeRow(colCount, useTh) {
        var tr = document.createElement("tr");
        var tag = useTh ? "th" : "td";
        for (var i = 0; i < colCount; i++) {
          var c = document.createElement(tag);
          c.innerHTML = "&nbsp;";
          tr.appendChild(c);
        }
        return tr;
      }

      function focusCell(cell) {
        if (!cell) return;
        var range = document.createRange();
        range.selectNodeContents(cell);
        range.collapse(true);
        var sel = window.getSelection();
        sel.removeAllRanges();
        sel.addRange(range);
        scheduleUpdate();
      }

      function doTableAction(act) {
        if (!currentCell || !editor.contains(currentCell)) return;
        var cell = currentCell;
        var row = cell.parentNode;
        var table = cell.closest("table");
        if (!table) return;
        var allRows = Array.prototype.slice.call(table.rows);
        var rowIdx = allRows.indexOf(row);
        var colIdx = Array.prototype.slice.call(row.cells).indexOf(cell);
        var colCount = row.cells.length;
        switch (act) {
          case "row-above": {
            var newRow = makeRow(colCount, false);
            row.parentNode.insertBefore(newRow, row);
            focusCell(newRow.cells[colIdx] || newRow.cells[0]);
            break;
          }
          case "row-below": {
            var newRow2 = makeRow(colCount, false);
            if (row.nextSibling) row.parentNode.insertBefore(newRow2, row.nextSibling);
            else row.parentNode.appendChild(newRow2);
            focusCell(newRow2.cells[colIdx] || newRow2.cells[0]);
            break;
          }
          case "col-left":
          case "col-right": {
            var insertAt = act === "col-left" ? colIdx : colIdx + 1;
            for (var i = 0; i < allRows.length; i++) {
              var r = allRows[i];
              var tag = i === 0 && r.cells[0] && r.cells[0].tagName === "TH" ? "th" : "td";
              var nc = document.createElement(tag);
              nc.innerHTML = "&nbsp;";
              if (insertAt >= r.cells.length) r.appendChild(nc);
              else r.insertBefore(nc, r.cells[insertAt]);
            }
            focusCell(allRows[rowIdx].cells[insertAt] || allRows[rowIdx].cells[allRows[rowIdx].cells.length - 1]);
            break;
          }
          case "row-delete": {
            if (allRows.length <= 1) {
              // Last row — remove the whole table.
              return doTableAction("table-delete");
            }
            var nextRow = allRows[rowIdx + 1] || allRows[rowIdx - 1];
            row.parentNode.removeChild(row);
            focusCell(nextRow ? nextRow.cells[colIdx] || nextRow.cells[0] : null);
            break;
          }
          case "col-delete": {
            if (colCount <= 1) {
              // Last column — remove the whole table.
              return doTableAction("table-delete");
            }
            for (var j = 0; j < allRows.length; j++) {
              var rr = allRows[j];
              if (rr.cells[colIdx]) rr.removeChild(rr.cells[colIdx]);
            }
            var sameRow = allRows[rowIdx];
            var nextCell = sameRow.cells[colIdx] || sameRow.cells[sameRow.cells.length - 1];
            focusCell(nextCell);
            break;
          }
          case "table-delete": {
            var after = table.nextSibling;
            table.parentNode.removeChild(table);
            // Place caret in the following block, or append a fresh
            // paragraph if there's nothing after the table.
            if (after && after.nodeType === 1) {
              var ra = document.createRange();
              ra.selectNodeContents(after);
              ra.collapse(true);
              var sa = window.getSelection();
              sa.removeAllRanges();
              sa.addRange(ra);
            } else {
              var p = document.createElement("p");
              p.appendChild(document.createElement("br"));
              editor.appendChild(p);
              var rb = document.createRange();
              rb.selectNodeContents(p);
              rb.collapse(true);
              var sb = window.getSelection();
              sb.removeAllRanges();
              sb.addRange(rb);
            }
            currentCell = null;
            tableActions.dataset.visible = "false";
            scheduleUpdate();
            return;
          }
          case "merge": {
            // Source for the rectangular range:
            //   1. preferred: cells the user drag-selected (visible
            //      `.re-cell-selected` highlight)
            //   2. fallback: Range start/end cells from the current
            //      text selection
            // Cell positions are taken as `cellIndex` within each row —
            // existing `rowSpan` / `colSpan` are NOT resolved; we treat
            // the selection's corners as a clean rectangle. Good enough
            // for the common case.
            var selectedCells = Array.prototype.slice.call(
              editor.querySelectorAll(".re-cell-selected")
            ).filter(function (c) { return c.closest("table") === table; });
            var sCell, eCell;
            if (selectedCells.length >= 2) {
              sCell = selectedCells[0];
              eCell = selectedCells[selectedCells.length - 1];
            } else {
              var sel = window.getSelection();
              if (!sel || sel.rangeCount === 0) return;
              var r0 = sel.getRangeAt(0);
              sCell = findCell(r0.startContainer);
              eCell = findCell(r0.endContainer);
            }
            if (!sCell || !eCell || sCell.closest("table") !== table) return;
            if (sCell === eCell) return;
            var sRow = sCell.parentNode;
            var eRow = eCell.parentNode;
            var sRowIdx = allRows.indexOf(sRow);
            var eRowIdx = allRows.indexOf(eRow);
            var sColIdx = Array.prototype.indexOf.call(sRow.cells, sCell);
            var eColIdx = Array.prototype.indexOf.call(eRow.cells, eCell);
            var minRow = Math.min(sRowIdx, eRowIdx);
            var maxRow = Math.max(sRowIdx, eRowIdx);
            var minCol = Math.min(sColIdx, eColIdx);
            var maxCol = Math.max(sColIdx, eColIdx);
            var anchor = allRows[minRow].cells[minCol];
            if (!anchor) return;
            var collected = [];
            // Walk in reverse column order so removeChild doesn't shift
            // indices we still need.
            for (var rr = minRow; rr <= maxRow; rr++) {
              var rrow = allRows[rr];
              for (var cc = maxCol; cc >= minCol; cc--) {
                if (rr === minRow && cc === minCol) continue;
                var cellToMerge = rrow.cells[cc];
                if (!cellToMerge) continue;
                var inner = cellToMerge.innerHTML;
                if (inner && inner.replace(/&nbsp;|\s/g, "") !== "") {
                  collected.unshift(inner);
                }
                rrow.removeChild(cellToMerge);
              }
            }
            var rs = maxRow - minRow + 1;
            var cs = maxCol - minCol + 1;
            if (rs > 1) anchor.rowSpan = rs;
            if (cs > 1) anchor.colSpan = cs;
            if (collected.length > 0) {
              var anchorInner = anchor.innerHTML;
              if (anchorInner && anchorInner.replace(/&nbsp;|\s/g, "") !== "") {
                collected.unshift(anchorInner);
              }
              anchor.innerHTML = collected.join(" ");
            }
            clearCellSelection();
            focusCell(anchor);
            break;
          }
          case "split": {
            // Split a merged cell back into its individual cells.
            // Only handles cells whose rowSpan / colSpan > 1. We
            // re-create the hidden cells in their original positions,
            // making the simplifying assumption that there are no
            // other merged cells overlapping this one's column band in
            // the rows below.
            var rspan = cell.rowSpan || 1;
            var cspan = cell.colSpan || 1;
            if (rspan === 1 && cspan === 1) return;
            cell.rowSpan = 1;
            cell.colSpan = 1;
            // Add (cspan - 1) cells to the right of the anchor in its
            // own row.
            for (var k = 1; k < cspan; k++) {
              var nc = document.createElement(cell.tagName === "TH" ? "th" : "td");
              nc.innerHTML = "&nbsp;";
              cell.parentNode.insertBefore(nc, cell.nextSibling);
            }
            // For each subsequent row covered by the original rowspan,
            // insert `cspan` cells at the original column position.
            for (var rr2 = 1; rr2 < rspan; rr2++) {
              var nextRow = allRows[rowIdx + rr2];
              if (!nextRow) continue;
              var insertBefore = nextRow.cells[colIdx] || null;
              for (var kk = 0; kk < cspan; kk++) {
                var nc2 = document.createElement("td");
                nc2.innerHTML = "&nbsp;";
                if (insertBefore) nextRow.insertBefore(nc2, insertBefore);
                else nextRow.appendChild(nc2);
              }
            }
            focusCell(cell);
            break;
          }
        }
        scheduleUpdate();
        syncTableActions();
      }

      tableActions.querySelectorAll("[data-act]").forEach(function (btn) {
        btn.addEventListener("click", function (e) {
          e.preventDefault();
          doTableAction(btn.getAttribute("data-act"));
        });
      });
    }

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
