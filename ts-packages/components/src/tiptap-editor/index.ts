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
import { buildToolbar } from "./toolbar";

const PROSEMIRROR_CSS = `
.ProseMirror {
  outline: none;
  min-height: 100%;
  height: 100%;
  overflow-wrap: break-word;
  max-width: 100%;
  font-size: 15px;
  color: var(--content-base, #fff);
}
.ProseMirror h1 {
  font-size: 1.5rem;
  font-weight: 700;
  margin-top: 1.5rem;
  margin-bottom: 1rem;
}
.ProseMirror h2 {
  font-size: 1.25rem;
  font-weight: 700;
  margin-top: 1.25rem;
  margin-bottom: 0.75rem;
}
.ProseMirror h3 {
  font-size: 1.125rem;
  font-weight: 600;
  margin-top: 1rem;
  margin-bottom: 0.5rem;
}
.ProseMirror ul {
  list-style: disc;
  padding-left: 1.5rem;
  margin: 0.5rem 0;
}
.ProseMirror ol {
  list-style: decimal;
  padding-left: 1.5rem;
  margin: 0.5rem 0;
}
.ProseMirror li {
  margin: 0.25rem 0;
}
.ProseMirror p {
  margin: 0.5rem 0;
}
.ProseMirror blockquote {
  border-left: 3px solid var(--stroke-default, #333);
  padding-left: 1rem;
  margin: 1rem 0;
  color: var(--content-muted, #999);
}
.ProseMirror mark {
  background: #fef08a;
  padding: 0 0.125rem;
}
.ProseMirror a {
  color: #3b82f6;
  text-decoration: underline;
  text-underline-offset: 2px;
}
.ProseMirror a:hover {
  color: #2563eb;
}
.ProseMirror img {
  border-radius: 0.5rem;
  max-width: 100%;
  height: auto;
  margin: 1rem auto;
  display: block;
}
.ProseMirror table {
  border-collapse: collapse;
  table-layout: fixed;
  width: 100%;
  min-width: 100%;
  margin: 1rem 0;
}
.ProseMirror td {
  border: 1px solid var(--stroke-default, #333);
  padding: 0.5rem;
  min-width: 100px;
  position: relative;
}
.ProseMirror th {
  border: 1px solid var(--stroke-default, #333);
  padding: 0.5rem;
  min-width: 100px;
  position: relative;
  font-weight: 600;
  background: var(--surface-sunken, #262626);
}
.ProseMirror .selectedCell {
  background: color-mix(in srgb, var(--action-primary, #fcb300) 20%, transparent);
  border: 2px solid var(--action-primary, #fcb300);
  outline: 2px solid color-mix(in srgb, var(--action-primary, #fcb300) 40%, transparent);
  outline-offset: -1px;
}
.ProseMirror .column-resize-handle {
  position: absolute;
  right: -2px;
  top: 0;
  bottom: 0;
  width: 6px;
  background: color-mix(in srgb, var(--action-primary, #fcb300) 70%, transparent);
  pointer-events: auto;
  cursor: col-resize;
  z-index: 50;
  touch-action: none;
}
.ProseMirror.resize-cursor {
  cursor: col-resize;
}
.ProseMirror iframe {
  width: 100%;
  max-width: 100%;
}
.ProseMirror p.is-editor-empty:first-child::before {
  content: attr(data-placeholder);
  color: var(--content-muted, #999);
  float: left;
  pointer-events: none;
  height: 0;
}
`;

let styleInjected = false;
function injectStyles(): void {
  if (styleInjected) return;
  const style = document.createElement("style");
  style.textContent = PROSEMIRROR_CSS;
  document.head.appendChild(style);
  styleInjected = true;
}

class TiptapEditor extends HTMLElement {
  private _editor: Editor | null = null;
  private _value: string = "";
  private _debounceTimer: ReturnType<typeof setTimeout> | null = null;

  get value(): string {
    return this._value;
  }

  set editable(val: string | boolean) {
    const str = String(val);
    if (this.getAttribute("editable") !== str) {
      this.setAttribute("editable", str);
    }
  }
  get editable(): string {
    return this.getAttribute("editable") || "true";
  }

  set content(val: string) {
    if (this._editor) return;
    if (this.getAttribute("content") !== val) {
      this.setAttribute("content", val);
    }
  }
  get content(): string {
    return this.getAttribute("content") || "";
  }

  set placeholder(val: string) {
    if (this.getAttribute("placeholder") !== val) {
      this.setAttribute("placeholder", val);
    }
  }
  get placeholder(): string {
    return this.getAttribute("placeholder") || "Type here...";
  }

  static get observedAttributes(): string[] {
    return ["content", "editable", "placeholder", "class"];
  }

  connectedCallback(): void {
    injectStyles();

    if (!this.classList.contains("block")) {
      this.classList.add("block");
    }

    // Outer wrapper: matches React's rounded container with border
    const wrapper = document.createElement("div");
    wrapper.className =
      "flex flex-col w-full h-full rounded-lg border border-transparent transition-colors p-1 bg-[var(--surface-raised,#1a1a1a)] text-[var(--content-base,#fff)] focus-within:border-[var(--action-primary,#fcb300)]";

    const editorEl = document.createElement("div");
    editorEl.className = "flex-1 w-full px-5 py-3 outline-none overflow-y-auto";

    const placeholder = this.getAttribute("placeholder") || "Type here...";
    const editable = this.getAttribute("editable") !== "false";
    const content = this.getAttribute("content") || "";

    this._editor = new Editor({
      element: editorEl,
      extensions: [
        StarterKit.configure({
          heading: { levels: [1, 2, 3] },
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
        }),
        Link.configure({
          autolink: true,
          linkOnPaste: true,
          openOnClick: false,
          HTMLAttributes: {
            rel: "noopener noreferrer nofollow",
          },
        }),
        Youtube.configure({
          controls: true,
          nocookie: true,
          allowFullscreen: true,
        }),
        Table.configure({
          resizable: true,
        }),
        TableRow,
        TableHeader,
        TableCell,
      ],
      content,
      editable,
      onUpdate: ({ editor }) => {
        this._value = editor.getHTML();

        if (this._debounceTimer) clearTimeout(this._debounceTimer);
        this._debounceTimer = setTimeout(() => {
          this._debounceTimer = null;
          const changeEvent = new CustomEvent("change", {
            detail: this._value,
            bubbles: true,
            composed: true,
          });
          this.dispatchEvent(changeEvent);
        }, 250);
      },
    });

    if (editable) {
      const toolbar = buildToolbar(this._editor);
      toolbar.setAttribute("data-tiptap-toolbar", "");
      wrapper.appendChild(toolbar);
    }
    wrapper.appendChild(editorEl);
    this.appendChild(wrapper);

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
      this._value = this._editor.getHTML();
    }

    if (name === "editable") {
      const isEditable = newVal !== "false";
      this._editor.setEditable(isEditable);

      const wrapper = this.querySelector(":scope > div");
      const existing = wrapper?.querySelector("[data-tiptap-toolbar]");
      if (isEditable && !existing && wrapper) {
        const toolbar = buildToolbar(this._editor);
        toolbar.setAttribute("data-tiptap-toolbar", "");
        wrapper.prepend(toolbar);
      } else if (!isEditable && existing) {
        existing.remove();
      }
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
