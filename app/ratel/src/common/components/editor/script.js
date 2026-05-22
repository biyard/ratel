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
      var empty =
        editor.textContent.trim().length === 0 && editor.children.length <= 1;
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
    editor.addEventListener("paste", function () {
      setTimeout(scheduleUpdate, 0);
    });

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
      // A bare `/` also qualifies — the report detail page wants the
      // command picker to open immediately on `/` rather than only
      // after the first letter. Trailing `:` for multi-level tokens
      // (`/data:`, `/data:analyze:`) keeps working since `:` is a
      // non-whitespace char captured by the regex.
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
        case "code-inline":
          return wrapInlineCode();
        case "link":
          return openModal("link");
        case "unlink":
          return applyCmd("unlink");
        case "image":
          return openModal("image");
        case "youtube":
          return openModal("youtube");
        case "table":
          return openModal("table");
        case "hr":
          return applyCmd("insertHorizontalRule");
        default:
          return applyCmd(cmd);
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
      PRE: "Code block",
    };

    var blockDropdowns = Array.prototype.slice.call(
      root.querySelectorAll(".re-block")
    );
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
        if (blockLabels[node.nodeName]) {
          foundTag = node.nodeName;
          break;
        }
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
      justifyFull: "justifyFull",
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
        } catch (_e) {
          /* unsupported in some browsers */
        }
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
      if (!savedRange) {
        editor.focus();
        return;
      }
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
      if (firstInput)
        setTimeout(function () {
          firstInput.focus();
        }, 30);
    }
    function closeModal(mask) {
      mask.classList.remove("open");
    }

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
    linkMask
      .querySelector(".re-link-confirm")
      .addEventListener("click", function () {
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
        if (imageCamera.files && imageCamera.files[0])
          insertFile(imageCamera.files[0]);
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
        var file =
          e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files[0];
        if (file) insertFile(file);
      });
    }

    // The "Insert URL" button only handles the URL path now; file
    // selection inserts immediately via the change handler above.
    imageMask
      .querySelector(".re-image-confirm")
      .addEventListener("click", function () {
        var url = imageUrl.value.trim();
        closeModal(imageMask);
        imageUrl.value = "";
        if (!url) return;
        restoreSelection();
        applyCmd("insertImage", url);
      });

    var youtubeMask = root.querySelector("[data-modal='youtube']");
    var youtubeUrl = youtubeMask.querySelector(".re-youtube-url");
    youtubeMask
      .querySelector(".re-youtube-confirm")
      .addEventListener("click", function () {
        var raw = youtubeUrl.value.trim();
        closeModal(youtubeMask);
        youtubeUrl.value = "";
        if (!raw) return;
        var id = parseYoutubeId(raw);
        if (!id) return;
        var html =
          '<div class="yt-wrap"><iframe src="https://www.youtube.com/embed/' +
          id +
          '" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe></div><p><br></p>';
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
    tableMask
      .querySelector(".re-table-confirm")
      .addEventListener("click", function () {
        var rows = parseInt(tableRows.value, 10) || 1;
        var cols = parseInt(tableCols.value, 10) || 1;
        var useHeader = tableHeader.checked;
        closeModal(tableMask);
        var html = "<table>";
        // Native <caption> element — semantically belongs at the top
        // of a table; users type the figure/표 label here ("표 1. …").
        // contenteditable=true overrides the row above which may have
        // set the editor to non-editable in some flows. Intentionally
        // empty so the CSS `:empty::before` placeholder hint shows.
        html +=
          '<caption class="re-table-caption" contenteditable="true" data-placeholder="캡션 (예: 표 1. 항목 비교)"></caption>';
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
      if (k === "b") {
        e.preventDefault();
        applyCmd("bold");
      } else if (k === "i") {
        e.preventDefault();
        applyCmd("italic");
      } else if (k === "u") {
        e.preventDefault();
        applyCmd("underline");
      } else if (k === "k") {
        e.preventDefault();
        openModal("link");
      }
    });

    // ── Selection-triggered bubble toolbar (desktop only) ─────
    // On coarse-pointer (touch) devices the OS provides its own selection
    // menu (Copy / Paste / Look up). Layering our bubble on top would
    // conflict and the static top toolbar remains available, so we skip
    // wiring entirely on touch. The bubble's DOM is rendered regardless;
    // it just stays invisible.
    var bubble = root.querySelector(".re-bubble");
    var coarsePointer =
      typeof window.matchMedia === "function" &&
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
        return (
          document.activeElement === editor ||
          bubble.contains(document.activeElement)
        );
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
        if (!sel || sel.rangeCount === 0 || sel.isCollapsed)
          return hideBubble();
        var range = sel.getRangeAt(0);
        if (!editor.contains(range.commonAncestorContainer))
          return hideBubble();
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
      window.addEventListener("scroll", hideBubble, {
        passive: true,
        capture: true,
      });
      window.addEventListener("resize", hideBubble);
      document.addEventListener("keydown", function (e) {
        if (e.key === "Escape" && bubble.dataset.visible === "true") {
          hideBubble();
          editor.focus();
        }
      });
    }

    // ── Table actions (insert row/col, delete row/col/table, merge, split) ──
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
      var dragAnchor = null;
      var dragActive = false;

      function clearCellSelection() {
        editor
          .querySelectorAll(".re-cell-selected")
          .forEach(function (c) { c.classList.remove("re-cell-selected"); });
      }

      // Build the rendered grid for a `<table>` — a 2D matrix of cell
      // references that accounts for rowSpan/colSpan. Each grid slot
      // points to the anchor <td>/<th> that visually occupies it, so
      // a 2x3 merged cell appears as the same reference in all six
      // slots. Without this, drag-selection / merge math falls back to
      // `cellIndex` which DOESN'T match visual columns once any cell
      // has colSpan > 1.
      function buildTableGrid(table) {
        var grid = [];
        var rows = Array.prototype.slice.call(table.rows);
        for (var r = 0; r < rows.length; r++) {
          if (!grid[r]) grid[r] = [];
          var col = 0;
          var cells = rows[r].cells;
          for (var i = 0; i < cells.length; i++) {
            var cell = cells[i];
            // Skip ahead past columns already occupied by a rowspan
            // from a previous row.
            while (grid[r][col]) col++;
            var rs = cell.rowSpan || 1;
            var cs = cell.colSpan || 1;
            for (var rr = 0; rr < rs; rr++) {
              var gr = r + rr;
              if (!grid[gr]) grid[gr] = [];
              for (var cc = 0; cc < cs; cc++) {
                grid[gr][col + cc] = cell;
              }
            }
            col += cs;
          }
        }
        return grid;
      }

      function findGridPosForCell(grid, cell) {
        for (var r = 0; r < grid.length; r++) {
          if (!grid[r]) continue;
          for (var c = 0; c < grid[r].length; c++) {
            if (grid[r][c] === cell) return { r: r, c: c };
          }
        }
        return null;
      }

      // Bounding rectangle of a cell on the rendered grid — accounts
      // for the cell's rowSpan / colSpan. Used to expand a selection
      // rectangle so it never bisects a merged cell.
      function findGridBoundsForCell(grid, cell) {
        var minR = -1, maxR = -1, minC = -1, maxC = -1;
        for (var r = 0; r < grid.length; r++) {
          if (!grid[r]) continue;
          for (var c = 0; c < grid[r].length; c++) {
            if (grid[r][c] !== cell) continue;
            if (minR === -1 || r < minR) minR = r;
            if (maxR === -1 || r > maxR) maxR = r;
            if (minC === -1 || c < minC) minC = c;
            if (maxC === -1 || c > maxC) maxC = c;
          }
        }
        if (minR === -1) return null;
        return { minR: minR, maxR: maxR, minC: minC, maxC: maxC };
      }

      function markSelectionRect(startCell, endCell) {
        clearCellSelection();
        if (!startCell || !endCell) return;
        var table = startCell.closest("table");
        if (!table || endCell.closest("table") !== table) return;
        // Use the rendered grid so dragging between rows ALWAYS picks
        // up the same visual columns, even when a row contains merged
        // cells. The cellIndex-based path used `parentNode.cells` which
        // counts each <td> as one column regardless of colSpan — a row
        // beneath a 2-col merged cell would line up with the wrong
        // physical column.
        var grid = buildTableGrid(table);
        var sp = findGridPosForCell(grid, startCell);
        var ep = findGridPosForCell(grid, endCell);
        if (!sp || !ep) return;
        var minR = Math.min(sp.r, ep.r), maxR = Math.max(sp.r, ep.r);
        var minC = Math.min(sp.c, ep.c), maxC = Math.max(sp.c, ep.c);
        // Expand the rectangle so that any merged cell partially
        // covered by it is fully included — otherwise the visible
        // selection rectangle slices through merged cells, which is
        // both ugly and impossible to merge cleanly afterwards.
        var changed = true;
        while (changed) {
          changed = false;
          for (var rr = minR; rr <= maxR; rr++) {
            if (!grid[rr]) continue;
            for (var cc = minC; cc <= maxC; cc++) {
              var cell = grid[rr][cc];
              if (!cell) continue;
              var bounds = findGridBoundsForCell(grid, cell);
              if (!bounds) continue;
              if (bounds.minR < minR) { minR = bounds.minR; changed = true; }
              if (bounds.maxR > maxR) { maxR = bounds.maxR; changed = true; }
              if (bounds.minC < minC) { minC = bounds.minC; changed = true; }
              if (bounds.maxC > maxC) { maxC = bounds.maxC; changed = true; }
            }
          }
        }
        // Mark each unique anchor cell intersecting the rectangle.
        var seen = [];
        for (var rr2 = minR; rr2 <= maxR; rr2++) {
          if (!grid[rr2]) continue;
          for (var cc2 = minC; cc2 <= maxC; cc2++) {
            var c2 = grid[rr2][cc2];
            if (!c2 || seen.indexOf(c2) !== -1) continue;
            seen.push(c2);
            c2.classList.add("re-cell-selected");
          }
        }
      }

      var multiCell = false;

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
          // Suppress the browser's native text selection so it doesn't
          // overlay our purple cell highlight + trigger the text-format
          // bubble. We keep a collapsed range inside the anchor cell so
          // `document.activeElement === editor` stays true and the
          // table toolbar remains visible after mouseup.
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
      document.addEventListener("mouseup", function () {
        dragActive = false;
      });
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
        if (top < 8) top = tr.bottom + pad;
        var left = tr.left;
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
              return doTableAction("table-delete");
            }
            var nextRow = allRows[rowIdx + 1] || allRows[rowIdx - 1];
            row.parentNode.removeChild(row);
            focusCell(nextRow ? nextRow.cells[colIdx] || nextRow.cells[0] : null);
            break;
          }
          case "col-delete": {
            if (colCount <= 1) {
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
            // Pull the user's drag-selection (preferred) or fall back
            // to the text Range's start/end cells. Either way, the
            // selection's bounding rectangle is computed on the
            // RENDERED GRID — `cellIndex` math breaks once any cell
            // has rowSpan/colSpan, because the same grid column number
            // points to different `<td>`s across rows.
            var selectedCells = Array.prototype.slice.call(
              editor.querySelectorAll(".re-cell-selected")
            ).filter(function (c) { return c.closest("table") === table; });
            var sCell, eCell;
            if (selectedCells.length >= 1) {
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

            var mergeGrid = buildTableGrid(table);
            var sBounds = findGridBoundsForCell(mergeGrid, sCell);
            var eBounds = findGridBoundsForCell(mergeGrid, eCell);
            if (!sBounds || !eBounds) return;
            var minRow = Math.min(sBounds.minR, eBounds.minR);
            var maxRow = Math.max(sBounds.maxR, eBounds.maxR);
            var minCol = Math.min(sBounds.minC, eBounds.minC);
            var maxCol = Math.max(sBounds.maxC, eBounds.maxC);

            // Expand the rectangle so we never bisect a merged cell
            // that's partially inside it — same fixed-point loop the
            // drag highlighter uses.
            var expanded = true;
            while (expanded) {
              expanded = false;
              for (var rr0 = minRow; rr0 <= maxRow; rr0++) {
                if (!mergeGrid[rr0]) continue;
                for (var cc0 = minCol; cc0 <= maxCol; cc0++) {
                  var hit = mergeGrid[rr0][cc0];
                  if (!hit) continue;
                  var hb = findGridBoundsForCell(mergeGrid, hit);
                  if (!hb) continue;
                  if (hb.minR < minRow) { minRow = hb.minR; expanded = true; }
                  if (hb.maxR > maxRow) { maxRow = hb.maxR; expanded = true; }
                  if (hb.minC < minCol) { minCol = hb.minC; expanded = true; }
                  if (hb.maxC > maxCol) { maxCol = hb.maxC; expanded = true; }
                }
              }
            }

            var anchor = mergeGrid[minRow][minCol];
            if (!anchor) return;

            // Collect every unique cell intersecting the rect (excl.
            // the anchor), in document order, harvest its text, then
            // remove it from the DOM.
            var toMerge = [];
            for (var rr = minRow; rr <= maxRow; rr++) {
              if (!mergeGrid[rr]) continue;
              for (var cc = minCol; cc <= maxCol; cc++) {
                var c = mergeGrid[rr][cc];
                if (!c || c === anchor) continue;
                if (toMerge.indexOf(c) === -1) toMerge.push(c);
              }
            }
            if (toMerge.length === 0) return;
            var collected = [];
            for (var mi = 0; mi < toMerge.length; mi++) {
              var cellToMerge = toMerge[mi];
              var inner = cellToMerge.innerHTML;
              if (inner && inner.replace(/&nbsp;|\s/g, "") !== "") {
                collected.push(inner);
              }
              if (cellToMerge.parentNode) {
                cellToMerge.parentNode.removeChild(cellToMerge);
              }
            }
            var rs = maxRow - minRow + 1;
            var cs = maxCol - minCol + 1;
            if (rs > 1) anchor.rowSpan = rs; else anchor.removeAttribute("rowspan");
            if (cs > 1) anchor.colSpan = cs; else anchor.removeAttribute("colspan");
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
            var rspan = cell.rowSpan || 1;
            var cspan = cell.colSpan || 1;
            if (rspan === 1 && cspan === 1) return;
            cell.rowSpan = 1;
            cell.colSpan = 1;
            for (var k = 1; k < cspan; k++) {
              var nc = document.createElement(cell.tagName === "TH" ? "th" : "td");
              nc.innerHTML = "&nbsp;";
              cell.parentNode.insertBefore(nc, cell.nextSibling);
            }
            for (var rr2 = 1; rr2 < rspan; rr2++) {
              var nextRow2 = allRows[rowIdx + rr2];
              if (!nextRow2) continue;
              var insertBefore = nextRow2.cells[colIdx] || null;
              for (var kk = 0; kk < cspan; kk++) {
                var nc2 = document.createElement("td");
                nc2.innerHTML = "&nbsp;";
                if (insertBefore) nextRow2.insertBefore(nc2, insertBefore);
                else nextRow2.appendChild(nc2);
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

    // Nest `li` one level deeper by moving it into a nested <ul>/<ol> that
    // becomes a child of the previous-sibling <li>. Producing the same shape
    // as `<ul><li>Outer<ul><li>Inner</li></ul></li></ul>` so semantic
    // selectors like `ul > li > ul > li` match. Caret is restored to where
    // it was inside `li` before the move (offset 0 of the first text node,
    // or offset 0 of the li if there's none).
    function mdIndentLi(li) {
      var prevLi = li.previousElementSibling;
      if (!prevLi || prevLi.nodeName !== "LI") return;
      var nested = prevLi.lastElementChild;
      var parentList = li.parentNode;
      var listTag = parentList && (parentList.nodeName === "UL" || parentList.nodeName === "OL")
        ? parentList.nodeName.toLowerCase()
        : "ul";
      if (!nested || (nested.nodeName !== "UL" && nested.nodeName !== "OL")) {
        nested = document.createElement(listTag);
        prevLi.appendChild(nested);
      }
      nested.appendChild(li);
      mdPlaceCaretAtBlockStart(li);
    }

    // Un-nest `li` one level. If its containing list is nested inside another
    // <li>, lift it out so it becomes a sibling of that outer <li>. If the
    // list is already at the top level (its parent isn't an <li>), exit the
    // list entirely into a fresh <p>.
    function mdOutdentLi(li) {
      var parentList = li.parentNode;
      if (!parentList || (parentList.nodeName !== "UL" && parentList.nodeName !== "OL")) return;
      var grandparent = parentList.parentNode;
      if (!grandparent) return;
      if (grandparent.nodeName === "LI") {
        // Nested case: move li out as sibling of grandparent li.
        var outerList = grandparent.parentNode;
        outerList.insertBefore(li, grandparent.nextSibling);
        if (parentList.children.length === 0) grandparent.removeChild(parentList);
        mdPlaceCaretAtBlockStart(li);
        return;
      }
      // Top-level: exit the list into a new <p>.
      var p = document.createElement("p");
      while (li.firstChild) p.appendChild(li.firstChild);
      if (!p.firstChild) p.appendChild(document.createElement("br"));
      var listNextSib = parentList.nextSibling;
      grandparent.insertBefore(p, listNextSib);
      parentList.removeChild(li);
      if (parentList.children.length === 0) grandparent.removeChild(parentList);
      mdPlaceCaretAtBlockStart(p);
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

      // Compute the text to test against the marker regex. For a real block
      // the marker must be at the start of the block's content. For bare
      // text at the editor root the "line" is just the caret's text node —
      // `mdTextBeforeCaretInBlock(editor)` would include text from preceding
      // sibling blocks (e.g. a <ul> above) and break the start-of-line
      // assumption, so use the text node's data up to the caret offset.
      var before;
      if (block === editor) {
        var sel0 = window.getSelection();
        if (!sel0 || !sel0.rangeCount) return false;
        var caretRange0 = sel0.getRangeAt(0);
        var caretNode0 = caretRange0.startContainer;
        if (caretNode0.nodeType !== 3 || caretNode0.parentNode !== editor) {
          return false;
        }
        // Require the text node to be at the start of its line — i.e. no
        // adjacent text node immediately before it (a <br> or block sibling
        // before it is fine and is what we want).
        if (caretNode0.previousSibling && caretNode0.previousSibling.nodeType === 3) {
          return false;
        }
        before = caretNode0.data.slice(0, caretRange0.startOffset);
      } else {
        before = mdTextBeforeCaretInBlock(block);
      }
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

    // Install a <pre><br></pre><p><br></p> pair (the code block + a typeable
    // follow-up paragraph) by either replacing `replaceNode` or inserting at
    // `parent`'s `nextSibling`. Caret is placed inside the <pre>. Shared by
    // the inline (third-backtick) and the legacy Enter trigger.
    function mdInstallFence(replaceNode, parent, nextSibling) {
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

    // ``` typed as the third backtick at the start of a line → <pre>.
    // No Enter required. Fires from the input handler on the backtick.
    function mdTryFence() {
      if (mdHasAncestorTag("PRE")) return false;
      if (mdHasAncestorTag("LI")) return false;
      var block = mdGetCaretBlock();
      if (!block) return false;
      // Bare text at editor root: the line is the caret's text node.
      if (block === editor) {
        var sel = window.getSelection();
        if (!sel || !sel.rangeCount) return false;
        var caretNode = sel.getRangeAt(0).startContainer;
        if (caretNode.nodeType !== 3 || caretNode.parentNode !== editor) return false;
        if (caretNode.previousSibling && caretNode.previousSibling.nodeType === 3) return false;
        if (caretNode.data !== "```") return false;
        var snap = mdSnapshotForRevert("```");
        mdInstallFence(caretNode, editor, null);
        lastConversion = snap;
        scheduleUpdate();
        return true;
      }
      // Block-wrapped: the block's text content is exactly "```".
      if (block.textContent === "```") {
        var snap2 = mdSnapshotForRevert("```");
        mdInstallFence(block, block.parentNode, null);
        lastConversion = snap2;
        scheduleUpdate();
        return true;
      }
      return false;
    }

    editor.addEventListener("input", function (e) {
      if (composing) return;
      if (e.inputType === "insertText") {
        if (e.data === " ") {
          if (mdTryConvert(e)) return;
        } else if (e.data === "`") {
          if (mdTryFence()) return;
        }
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
      // execCommand("indent") on a list item in Chrome produces invalid HTML —
      // the new <ul>/<ol> ends up as a SIBLING of the outer <li> rather than
      // a child of it, so semantic selectors like `ul > li > ul > li` don't
      // match. Do the nesting by hand to produce well-formed lists.
      if (e.key === "Tab" && !e.metaKey && !e.ctrlKey && !e.altKey) {
        var tabLi = mdAncestorTag("LI");
        if (tabLi) {
          e.preventDefault();
          if (e.shiftKey) mdOutdentLi(tabLi);
          else mdIndentLi(tabLi);
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
    subtree: true,
  });
})();
