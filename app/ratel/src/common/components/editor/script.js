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

    // ── Block-format dropdown ──────────────────────────────
    var blockDropdown = root.querySelector(".re-block");
    var blockBtn = blockDropdown.querySelector(".re-block__btn");
    var blockBtnLabel = blockDropdown.querySelector(".re-block__label");
    var blockLabels = {
      P: "Paragraph",
      H1: "Heading 1",
      H2: "Heading 2",
      H3: "Heading 3",
      BLOCKQUOTE: "Quote",
      PRE: "Code block"
    };

    blockBtn.addEventListener("click", function (e) {
      e.preventDefault();
      e.stopPropagation();
      var open = blockDropdown.dataset.open === "true";
      blockDropdown.dataset.open = open ? "false" : "true";
      blockBtn.setAttribute("aria-expanded", open ? "false" : "true");
    });
    document.addEventListener("click", function (e) {
      if (!blockDropdown.contains(e.target)) {
        blockDropdown.dataset.open = "false";
        blockBtn.setAttribute("aria-expanded", "false");
      }
    });
    blockDropdown.addEventListener("mousedown", function () {
      savedRange = saveSelection();
    });
    root.querySelectorAll("[data-block]").forEach(function (item) {
      item.addEventListener("click", function (e) {
        e.preventDefault();
        var tag = item.dataset.block;
        blockDropdown.dataset.open = "false";
        blockBtn.setAttribute("aria-expanded", "false");
        restoreSelection();
        applyCmd("formatBlock", "<" + tag + ">");
      });
    });

    function syncBlockLabel() {
      if (document.activeElement !== editor) return;
      var sel = window.getSelection();
      if (!sel || sel.rangeCount === 0) return;
      var node = sel.getRangeAt(0).startContainer;
      if (node.nodeType === 3) node = node.parentNode;
      while (node && node !== editor) {
        var tag = node.nodeName;
        if (blockLabels[tag]) {
          blockBtnLabel.textContent = blockLabels[tag];
          root.querySelectorAll("[data-block]").forEach(function (item) {
            item.classList.toggle("re-block__item--active", item.dataset.block === tag);
          });
          return;
        }
        node = node.parentNode;
      }
      blockBtnLabel.textContent = "Paragraph";
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
