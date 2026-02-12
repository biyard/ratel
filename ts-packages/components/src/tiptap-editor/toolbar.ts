import type { Editor } from "@tiptap/core";
import {
  TEXT_COLORS,
  HIGHLIGHT_COLORS,
  type TiptapColorPicker,
  type TiptapHeadingSelect,
} from "./components";

// Ensure all web components are registered
import "./components/toolbar-button";
import "./components/toolbar-separator";
import "./components/color-picker";
import "./components/heading-select";

function btn(
  label: string,
  title: string,
  fontSize?: string,
): HTMLElement {
  const el = document.createElement("tiptap-toolbar-btn");
  el.setAttribute("label", label);
  el.title = title;
  if (fontSize) el.setAttribute("font-size", fontSize);
  return el;
}

function sep(): HTMLElement {
  return document.createElement("tiptap-separator");
}

export function buildToolbar(editor: Editor): HTMLDivElement {
  const toolbar = document.createElement("div");
  toolbar.className =
    "flex items-center gap-2 p-1 border-b border-[var(--stroke-default,#333)] bg-[var(--surface-raised,#1a1a1a)] overflow-x-auto";

  // ── Text formatting ────────────────────────────────────────────────

  const boldBtn = btn("<b>B</b>", "Bold (Ctrl+B)");
  boldBtn.addEventListener("click", () =>
    editor.chain().focus().toggleBold().run(),
  );
  toolbar.appendChild(boldBtn);

  const italicBtn = btn("<i>I</i>", "Italic (Ctrl+I)");
  italicBtn.addEventListener("click", () =>
    editor.chain().focus().toggleItalic().run(),
  );
  toolbar.appendChild(italicBtn);

  const underlineBtn = btn("<u>U</u>", "Underline (Ctrl+U)");
  underlineBtn.addEventListener("click", () =>
    editor.chain().focus().toggleUnderline().run(),
  );
  toolbar.appendChild(underlineBtn);

  const strikeBtn = btn("<s>S</s>", "Strikethrough");
  strikeBtn.addEventListener("click", () =>
    editor.chain().focus().toggleStrike().run(),
  );
  toolbar.appendChild(strikeBtn);

  toolbar.appendChild(sep());

  // ── Colors ─────────────────────────────────────────────────────────

  const textColorPicker = document.createElement(
    "tiptap-color-picker",
  ) as TiptapColorPicker;
  textColorPicker.colors = TEXT_COLORS;
  toolbar.appendChild(textColorPicker);
  // Set trigger label/title after mount
  requestAnimationFrame(() => {
    textColorPicker.triggerLabel =
      '<span class="border-b-[3px] border-current pb-px">A</span>';
    textColorPicker.triggerTitle = "Text Color";
  });
  textColorPicker.addEventListener("color-select", ((e: CustomEvent) => {
    editor.chain().focus().setColor(e.detail).run();
  }) as EventListener);

  const highlightPicker = document.createElement(
    "tiptap-color-picker",
  ) as TiptapColorPicker;
  highlightPicker.colors = HIGHLIGHT_COLORS;
  toolbar.appendChild(highlightPicker);
  requestAnimationFrame(() => {
    highlightPicker.triggerLabel =
      '<span class="bg-yellow-300 px-1 rounded-sm">A</span>';
    highlightPicker.triggerTitle = "Highlight";
  });
  highlightPicker.addEventListener("color-select", ((e: CustomEvent) => {
    editor.chain().focus().setHighlight({ color: e.detail }).run();
  }) as EventListener);

  toolbar.appendChild(sep());

  // ── Heading ────────────────────────────────────────────────────────

  const headingSelect = document.createElement(
    "tiptap-heading-select",
  ) as TiptapHeadingSelect;
  headingSelect.addEventListener("heading-change", ((e: CustomEvent) => {
    const v = e.detail;
    if (v === "p") editor.chain().focus().setParagraph().run();
    else
      editor
        .chain()
        .focus()
        .toggleHeading({ level: parseInt(v) as 1 | 2 | 3 })
        .run();
  }) as EventListener);
  toolbar.appendChild(headingSelect);

  toolbar.appendChild(sep());

  // ── Alignment ──────────────────────────────────────────────────────

  const alignLeftBtn = btn("\u2261", "Align Left", "16px");
  alignLeftBtn.addEventListener("click", () => {
    if (editor.isActive({ textAlign: "left" }))
      editor.chain().focus().unsetTextAlign().run();
    else editor.chain().focus().setTextAlign("left").run();
  });
  toolbar.appendChild(alignLeftBtn);

  const alignCenterBtn = btn("\u2261", "Align Center", "16px");
  alignCenterBtn.addEventListener("click", () => {
    if (editor.isActive({ textAlign: "center" }))
      editor.chain().focus().unsetTextAlign().run();
    else editor.chain().focus().setTextAlign("center").run();
  });
  toolbar.appendChild(alignCenterBtn);

  const alignRightBtn = btn("\u2261", "Align Right", "16px");
  alignRightBtn.addEventListener("click", () => {
    if (editor.isActive({ textAlign: "right" }))
      editor.chain().focus().unsetTextAlign().run();
    else editor.chain().focus().setTextAlign("right").run();
  });
  toolbar.appendChild(alignRightBtn);

  toolbar.appendChild(sep());

  // ── Lists ──────────────────────────────────────────────────────────

  const bulletBtn = btn("\u2022", "Bullet List");
  bulletBtn.addEventListener("click", () =>
    editor.chain().focus().toggleBulletList().run(),
  );
  toolbar.appendChild(bulletBtn);

  const orderedBtn = btn("1.", "Numbered List");
  orderedBtn.addEventListener("click", () =>
    editor.chain().focus().toggleOrderedList().run(),
  );
  toolbar.appendChild(orderedBtn);

  toolbar.appendChild(sep());

  // ── Link ───────────────────────────────────────────────────────────

  const linkBtn = btn("\uD83D\uDD17", "Link", "14px");
  linkBtn.addEventListener("click", () => {
    const current = editor.getAttributes("link")?.href ?? "";
    const href = window.prompt("Input Link URL", current || "https://");
    if (href === null) return;
    if (!href.trim()) {
      editor.chain().focus().extendMarkRange("link").unsetMark("link").run();
      return;
    }
    const url = /^https?:\/\//i.test(href.trim())
      ? href.trim()
      : `https://${href.trim()}`;
    const { empty } = editor.state.selection;
    if (empty) {
      editor
        .chain()
        .focus()
        .insertContent([
          {
            type: "text",
            text: url,
            marks: [
              {
                type: "link",
                attrs: {
                  href: url,
                  target: "_blank",
                  rel: "noopener noreferrer nofollow",
                },
              },
            ],
          },
        ])
        .run();
    } else {
      editor
        .chain()
        .focus()
        .extendMarkRange("link")
        .setMark("link", {
          href: url,
          target: "_blank",
          rel: "noopener noreferrer nofollow",
        })
        .run();
    }
  });
  toolbar.appendChild(linkBtn);

  const unlinkBtn = btn("\u274C", "Remove Link", "10px");
  unlinkBtn.addEventListener("click", () => {
    editor.chain().focus().extendMarkRange("link").unsetMark("link").run();
  });
  toolbar.appendChild(unlinkBtn);

  toolbar.appendChild(sep());

  // ── Image ──────────────────────────────────────────────────────────

  const imageBtn = btn("\uD83D\uDDBC", "Upload Image", "14px");
  const fileInput = document.createElement("input");
  fileInput.type = "file";
  fileInput.accept = "image/*";
  fileInput.className = "hidden";
  fileInput.addEventListener("change", () => {
    const file = fileInput.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (e) => {
      const base64 = e.target?.result as string;
      editor.chain().focus().setImage({ src: base64, alt: file.name }).run();
    };
    reader.readAsDataURL(file);
    fileInput.value = "";
  });
  imageBtn.addEventListener("click", () => fileInput.click());
  toolbar.appendChild(imageBtn);
  toolbar.appendChild(fileInput);

  toolbar.appendChild(sep());

  // ── Table ──────────────────────────────────────────────────────────

  const tableBtn = btn("\u229E", "Insert Table");
  tableBtn.addEventListener("click", () => {
    editor
      .chain()
      .focus()
      .insertTable({ rows: 3, cols: 3, withHeaderRow: true })
      .run();
  });
  toolbar.appendChild(tableBtn);

  const tableOps = document.createElement("span");
  tableOps.className = "hidden items-center gap-1";

  const tableActions: [string, string, () => void][] = [
    ["+R\u2191", "Add Row Before", () => editor.chain().focus().addRowBefore().run()],
    ["+R\u2193", "Add Row After", () => editor.chain().focus().addRowAfter().run()],
    ["-R", "Delete Row", () => editor.chain().focus().deleteRow().run()],
    ["+C\u2190", "Add Col Before", () => editor.chain().focus().addColumnBefore().run()],
    ["+C\u2192", "Add Col After", () => editor.chain().focus().addColumnAfter().run()],
    ["-C", "Delete Col", () => editor.chain().focus().deleteColumn().run()],
    ["\u29C9", "Merge", () => editor.chain().focus().mergeCells().run()],
    ["\u29C8", "Split", () => editor.chain().focus().splitCell().run()],
    ["\u2715Tbl", "Delete Table", () => editor.chain().focus().deleteTable().run()],
  ];
  for (const [label, title, cmd] of tableActions) {
    const b = btn(label, title, "11px");
    b.addEventListener("click", cmd);
    tableOps.appendChild(b);
  }
  toolbar.appendChild(tableOps);

  // ── Active state sync ──────────────────────────────────────────────

  const setActive = (el: HTMLElement, on: boolean) =>
    el.setAttribute("active", String(on));

  const updateState = () => {
    setActive(boldBtn, editor.isActive("bold"));
    setActive(italicBtn, editor.isActive("italic"));
    setActive(underlineBtn, editor.isActive("underline"));
    setActive(strikeBtn, editor.isActive("strike"));
    setActive(alignLeftBtn, editor.isActive({ textAlign: "left" }));
    setActive(alignCenterBtn, editor.isActive({ textAlign: "center" }));
    setActive(alignRightBtn, editor.isActive({ textAlign: "right" }));
    setActive(bulletBtn, editor.isActive("bulletList"));
    setActive(orderedBtn, editor.isActive("orderedList"));
    setActive(linkBtn, editor.isActive("link"));

    headingSelect.value = editor.isActive("heading", { level: 1 })
      ? "1"
      : editor.isActive("heading", { level: 2 })
        ? "2"
        : editor.isActive("heading", { level: 3 })
          ? "3"
          : "p";

    const inTable = editor.isActive("table");
    tableOps.classList.toggle("hidden", !inTable);
    tableOps.classList.toggle("inline-flex", inTable);
    tableBtn.classList.toggle("hidden", inTable);
    tableBtn.classList.toggle("inline-flex", !inTable);
  };

  editor.on("selectionUpdate", updateState);
  editor.on("update", updateState);

  return toolbar;
}
