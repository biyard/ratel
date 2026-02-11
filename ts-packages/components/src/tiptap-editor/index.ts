import { Editor } from "@tiptap/core";
import StarterKit from "@tiptap/starter-kit";
import Placeholder from "@tiptap/extension-placeholder";
import TextStyle from "@tiptap/extension-text-style";
import Color from "@tiptap/extension-color";
import Highlight from "@tiptap/extension-highlight";
import TextAlign from "@tiptap/extension-text-align";
import Underline from "@tiptap/extension-underline";
import Image from "@tiptap/extension-image";
import Link from "@tiptap/extension-link";
import Youtube from "@tiptap/extension-youtube";
import Table from "@tiptap/extension-table";
import TableRow from "@tiptap/extension-table-row";
import TableHeader from "@tiptap/extension-table-header";
import TableCell from "@tiptap/extension-table-cell";

const TEXT_COLORS = [
  "#000000",
  "#434343",
  "#666666",
  "#999999",
  "#b7b7b7",
  "#cccccc",
  "#d9d9d9",
  "#ffffff",
  "#980000",
  "#ff0000",
  "#ff9900",
  "#ffff00",
  "#00ff00",
  "#00ffff",
  "#4a86e8",
  "#0000ff",
  "#9900ff",
  "#ff00ff",
  "#e6b8af",
  "#f4cccc",
  "#fce5cd",
  "#fff2cc",
  "#d9ead3",
  "#d0e0e3",
  "#c9daf8",
  "#cfe2f3",
  "#d9d2e9",
  "#ead1dc",
];

const HIGHLIGHT_COLORS = [
  "#ffff00",
  "#00ff00",
  "#00ffff",
  "#ff00ff",
  "#ff0000",
  "#0000ff",
  "#fce5cd",
  "#d9ead3",
  "#cfe2f3",
  "#d9d2e9",
  "#ead1dc",
  "#f4cccc",
];

function createButton(label: string, title: string): HTMLButtonElement {
  const btn = document.createElement("button");
  btn.type = "button";
  btn.innerHTML = label;
  btn.title = title;
  btn.className =
    "bg-transparent border border-transparent rounded min-w-[28px] h-7 inline-flex items-center justify-center px-2 cursor-pointer text-[13px] leading-none text-inherit hover:bg-gray-500/15";
  return btn;
}

function setActive(btn: HTMLButtonElement, active: boolean): void {
  btn.dataset.active = active ? "1" : "";
  if (active) {
    btn.classList.add("bg-gray-500/25", "border-gray-500/30");
    btn.classList.remove("bg-transparent", "border-transparent");
  } else {
    btn.classList.remove("bg-gray-500/25", "border-gray-500/30");
    btn.classList.add("bg-transparent", "border-transparent");
  }
}

function createSeparator(): HTMLSpanElement {
  const sep = document.createElement("span");
  sep.className = "inline-block w-px h-5 bg-gray-500/30 mx-1 align-middle";
  return sep;
}

function createColorPicker(
  colors: string[],
  onSelect: (color: string) => void,
): HTMLDivElement {
  const wrapper = document.createElement("div");
  wrapper.className = "relative inline-block";

  const trigger = createButton("A", "Color");
  wrapper.appendChild(trigger);

  const popup = document.createElement("div");
  popup.className =
    "hidden absolute top-full left-0 z-[999] bg-white border border-gray-300 rounded-md p-2 shadow-lg w-[200px] flex-wrap gap-1";

  for (const c of colors) {
    const swatch = document.createElement("button");
    swatch.type = "button";
    swatch.className =
      "w-5 h-5 rounded-sm border border-black/15 cursor-pointer p-0";
    swatch.style.background = c;
    swatch.title = c;
    swatch.addEventListener("click", (e) => {
      e.stopPropagation();
      onSelect(c);
      popup.classList.add("hidden");
      popup.classList.remove("flex");
    });
    popup.appendChild(swatch);
  }

  wrapper.appendChild(popup);

  trigger.addEventListener("click", (e) => {
    e.stopPropagation();
    const isHidden = popup.classList.contains("hidden");
    if (isHidden) {
      popup.classList.remove("hidden");
      popup.classList.add("flex");
    } else {
      popup.classList.add("hidden");
      popup.classList.remove("flex");
    }
  });

  document.addEventListener("click", () => {
    popup.classList.add("hidden");
    popup.classList.remove("flex");
  });

  return wrapper;
}

function createHeadingSelect(editor: Editor): HTMLSelectElement {
  const select = document.createElement("select");
  select.className =
    "bg-transparent border border-gray-500/30 rounded px-1 text-[13px] h-7 cursor-pointer text-inherit";
  select.title = "Heading";

  const options = [
    { value: "p", label: "Paragraph" },
    { value: "1", label: "Heading 1" },
    { value: "2", label: "Heading 2" },
    { value: "3", label: "Heading 3" },
  ];

  for (const o of options) {
    const opt = document.createElement("option");
    opt.value = o.value;
    opt.textContent = o.label;
    select.appendChild(opt);
  }

  select.addEventListener("change", () => {
    const v = select.value;
    if (v === "p") {
      editor.chain().focus().setParagraph().run();
    } else {
      editor
        .chain()
        .focus()
        .toggleHeading({ level: parseInt(v) as 1 | 2 | 3 })
        .run();
    }
  });

  return select;
}

function buildToolbar(editor: Editor): HTMLDivElement {
  const toolbar = document.createElement("div");
  toolbar.className =
    "flex items-center gap-0.5 px-2 py-1 border-b border-gray-500/20 flex-wrap overflow-x-auto";

  // Bold
  const boldBtn = createButton("<b>B</b>", "Bold (Ctrl+B)");
  boldBtn.addEventListener("click", () =>
    editor.chain().focus().toggleBold().run(),
  );
  toolbar.appendChild(boldBtn);

  // Italic
  const italicBtn = createButton("<i>I</i>", "Italic (Ctrl+I)");
  italicBtn.addEventListener("click", () =>
    editor.chain().focus().toggleItalic().run(),
  );
  toolbar.appendChild(italicBtn);

  // Underline
  const underlineBtn = createButton("<u>U</u>", "Underline (Ctrl+U)");
  underlineBtn.addEventListener("click", () =>
    editor.chain().focus().toggleUnderline().run(),
  );
  toolbar.appendChild(underlineBtn);

  // Strike
  const strikeBtn = createButton("<s>S</s>", "Strikethrough");
  strikeBtn.addEventListener("click", () =>
    editor.chain().focus().toggleStrike().run(),
  );
  toolbar.appendChild(strikeBtn);

  toolbar.appendChild(createSeparator());

  // Text Color
  const textColorPicker = createColorPicker(TEXT_COLORS, (color) => {
    editor.chain().focus().setColor(color).run();
  });
  const textColorTrigger = textColorPicker.querySelector("button")!;
  textColorTrigger.innerHTML =
    '<span class="border-b-[3px] border-current pb-px">A</span>';
  textColorTrigger.title = "Text Color";
  toolbar.appendChild(textColorPicker);

  // Highlight Color
  const highlightPicker = createColorPicker(HIGHLIGHT_COLORS, (color) => {
    editor.chain().focus().setHighlight({ color }).run();
  });
  const highlightTrigger = highlightPicker.querySelector("button")!;
  highlightTrigger.innerHTML =
    '<span class="bg-yellow-300 px-1 rounded-sm">A</span>';
  highlightTrigger.title = "Highlight";
  toolbar.appendChild(highlightPicker);

  toolbar.appendChild(createSeparator());

  // Heading
  const headingSelect = createHeadingSelect(editor);
  toolbar.appendChild(headingSelect);

  toolbar.appendChild(createSeparator());

  // Align Left
  const alignLeftBtn = createButton("\u2261", "Align Left");
  alignLeftBtn.className += " text-base";
  alignLeftBtn.addEventListener("click", () => {
    if (editor.isActive({ textAlign: "left" })) {
      editor.chain().focus().unsetTextAlign().run();
    } else {
      editor.chain().focus().setTextAlign("left").run();
    }
  });
  toolbar.appendChild(alignLeftBtn);

  // Align Center
  const alignCenterBtn = createButton("\u2261", "Align Center");
  alignCenterBtn.className += " text-base text-center";
  alignCenterBtn.addEventListener("click", () => {
    if (editor.isActive({ textAlign: "center" })) {
      editor.chain().focus().unsetTextAlign().run();
    } else {
      editor.chain().focus().setTextAlign("center").run();
    }
  });
  toolbar.appendChild(alignCenterBtn);

  // Align Right
  const alignRightBtn = createButton("\u2261", "Align Right");
  alignRightBtn.className += " text-base text-right";
  alignRightBtn.addEventListener("click", () => {
    if (editor.isActive({ textAlign: "right" })) {
      editor.chain().focus().unsetTextAlign().run();
    } else {
      editor.chain().focus().setTextAlign("right").run();
    }
  });
  toolbar.appendChild(alignRightBtn);

  toolbar.appendChild(createSeparator());

  // Bullet List
  const bulletBtn = createButton("\u2022", "Bullet List");
  bulletBtn.addEventListener("click", () =>
    editor.chain().focus().toggleBulletList().run(),
  );
  toolbar.appendChild(bulletBtn);

  // Ordered List
  const orderedBtn = createButton("1.", "Numbered List");
  orderedBtn.addEventListener("click", () =>
    editor.chain().focus().toggleOrderedList().run(),
  );
  toolbar.appendChild(orderedBtn);

  toolbar.appendChild(createSeparator());

  // Link
  const linkBtn = createButton("\uD83D\uDD17", "Link");
  linkBtn.className += " text-sm";
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

  // Unlink
  const unlinkBtn = createButton("\u274C", "Remove Link");
  unlinkBtn.className += " text-[10px]";
  unlinkBtn.addEventListener("click", () => {
    editor.chain().focus().extendMarkRange("link").unsetMark("link").run();
  });
  toolbar.appendChild(unlinkBtn);

  toolbar.appendChild(createSeparator());

  // Image
  const imageBtn = createButton("\uD83D\uDDBC", "Upload Image");
  imageBtn.className += " text-sm";
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

  toolbar.appendChild(createSeparator());

  // Insert Table
  const tableBtn = createButton("\u229E", "Insert Table");
  tableBtn.addEventListener("click", () => {
    editor
      .chain()
      .focus()
      .insertTable({ rows: 3, cols: 3, withHeaderRow: true })
      .run();
  });
  toolbar.appendChild(tableBtn);

  // Table operations (shown/hidden based on cursor)
  const tableOps = document.createElement("span");
  tableOps.className = "hidden items-center gap-0.5";

  const tableActions: { label: string; title: string; cmd: () => void }[] = [
    {
      label: "+R\u2191",
      title: "Add Row Before",
      cmd: () => editor.chain().focus().addRowBefore().run(),
    },
    {
      label: "+R\u2193",
      title: "Add Row After",
      cmd: () => editor.chain().focus().addRowAfter().run(),
    },
    {
      label: "-R",
      title: "Delete Row",
      cmd: () => editor.chain().focus().deleteRow().run(),
    },
    {
      label: "+C\u2190",
      title: "Add Column Before",
      cmd: () => editor.chain().focus().addColumnBefore().run(),
    },
    {
      label: "+C\u2192",
      title: "Add Column After",
      cmd: () => editor.chain().focus().addColumnAfter().run(),
    },
    {
      label: "-C",
      title: "Delete Column",
      cmd: () => editor.chain().focus().deleteColumn().run(),
    },
    {
      label: "\u29C9",
      title: "Merge Cells",
      cmd: () => editor.chain().focus().mergeCells().run(),
    },
    {
      label: "\u29C8",
      title: "Split Cell",
      cmd: () => editor.chain().focus().splitCell().run(),
    },
    {
      label: "\u2715Tbl",
      title: "Delete Table",
      cmd: () => editor.chain().focus().deleteTable().run(),
    },
  ];

  for (const a of tableActions) {
    const b = createButton(a.label, a.title);
    b.className += " text-[11px]";
    b.addEventListener("click", () => a.cmd());
    tableOps.appendChild(b);
  }
  toolbar.appendChild(tableOps);

  // Update active states on selection/content change
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

    // Heading select
    if (editor.isActive("heading", { level: 1 })) headingSelect.value = "1";
    else if (editor.isActive("heading", { level: 2 }))
      headingSelect.value = "2";
    else if (editor.isActive("heading", { level: 3 }))
      headingSelect.value = "3";
    else headingSelect.value = "p";

    // Table context
    const inTable = editor.isActive("table");
    if (inTable) {
      tableOps.classList.remove("hidden");
      tableOps.classList.add("inline-flex");
      tableBtn.classList.add("hidden");
      tableBtn.classList.remove("inline-flex");
    } else {
      tableOps.classList.add("hidden");
      tableOps.classList.remove("inline-flex");
      tableBtn.classList.remove("hidden");
      tableBtn.classList.add("inline-flex");
    }
  };

  editor.on("selectionUpdate", updateState);
  editor.on("update", updateState);

  return toolbar;
}

class TiptapEditor extends HTMLElement {
  private _editor: Editor | null = null;
  private _value: string = "";
  private _debounceTimer: ReturnType<typeof setTimeout> | null = null;

  get value(): string {
    return this._value;
  }

  static get observedAttributes(): string[] {
    return ["content", "editable", "placeholder", "class"];
  }

  connectedCallback(): void {
    if (!this.classList.contains("block")) {
      this.classList.add("block");
    }

    const editorEl = document.createElement("div");
    editorEl.className = "w-full h-full px-4 py-3 outline-none";

    const placeholder = this.getAttribute("placeholder") || "Type here...";
    const editable = this.getAttribute("editable") !== "false";
    const content = this.getAttribute("content") || "";

    this._editor = new Editor({
      element: editorEl,
      extensions: [
        StarterKit.configure({
          heading: { levels: [1, 2, 3] },
          bulletList: { HTMLAttributes: { class: "list-disc pl-4" } },
          orderedList: { HTMLAttributes: { class: "list-decimal pl-4" } },
        }),
        Placeholder.configure({ placeholder }),
        TextStyle,
        Color,
        Highlight.configure({ multicolor: true }),
        TextAlign.configure({
          types: ["heading", "paragraph"],
          alignments: ["left", "center", "right"],
        }),
        Underline,
        Image.configure({
          inline: true,
          allowBase64: true,
          HTMLAttributes: {
            class: "rounded-lg max-w-full h-auto my-4 mx-auto block",
          },
        }),
        Link.configure({
          autolink: true,
          linkOnPaste: true,
          openOnClick: false,
          HTMLAttributes: {
            rel: "noopener noreferrer nofollow",
            class:
              "text-blue-500 underline underline-offset-2 decoration-blue-500 hover:text-blue-600",
          },
        }),
        Youtube.configure({
          controls: true,
          nocookie: true,
          allowFullscreen: true,
          HTMLAttributes: {
            class: "w-full max-w-[640px] aspect-video mx-auto",
          },
        }),
        Table.configure({
          resizable: true,
          HTMLAttributes: {
            class: "border-collapse table-fixed w-full min-w-full my-4",
          },
        }),
        TableRow,
        TableHeader.configure({
          HTMLAttributes: { class: "bg-muted font-semibold" },
        }),
        TableCell.configure({
          HTMLAttributes: { class: "border border-border p-2" },
        }),
      ],
      content,
      editable,
      onUpdate: ({ editor }) => {
        this._value = editor.getHTML();
        if (this._debounceTimer) clearTimeout(this._debounceTimer);
        this._debounceTimer = setTimeout(() => {
          this._debounceTimer = null;
          this.dispatchEvent(new Event("input", { bubbles: true }));
        }, 250);
      },
    });

    // Build toolbar and prepend before editor
    if (editable) {
      const toolbar = buildToolbar(this._editor);
      this.appendChild(toolbar);
    }
    this.appendChild(editorEl);

    this._value = this._editor.getHTML();
  }

  disconnectedCallback(): void {
    if (this._debounceTimer) clearTimeout(this._debounceTimer);
    this._editor?.destroy();
    this._editor = null;
  }

  attributeChangedCallback(
    name: string,
    oldVal: string | null,
    newVal: string | null,
  ): void {
    if (!this._editor || oldVal === newVal) return;

    if (
      name === "content" &&
      !this._editor.isFocused &&
      newVal !== this._editor.getHTML()
    ) {
      this._editor.commands.setContent(newVal || "", false);
      this._value = this._editor.getHTML();
    }

    if (name === "editable") {
      this._editor.setEditable(newVal !== "false");
    }
  }

  getHTML(): string {
    return this._editor?.getHTML() || "";
  }

  setContent(html: string): void {
    this._editor?.commands.setContent(html || "", false);
    this._value = this._editor?.getHTML() || "";
  }
}

if (!customElements.get("tiptap-editor")) {
  customElements.define("tiptap-editor", TiptapEditor);
}
