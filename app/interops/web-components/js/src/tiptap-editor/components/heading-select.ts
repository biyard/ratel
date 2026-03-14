const HEADINGS = [
  { value: "p", label: "Paragraph" },
  { value: "1", label: "Heading 1" },
  { value: "2", label: "Heading 2" },
  { value: "3", label: "Heading 3" },
];

export class TiptapHeadingSelect extends HTMLElement {
  static get observedAttributes(): string[] {
    return ["value"];
  }

  private _select: HTMLSelectElement | null = null;

  get value(): string {
    return this._select?.value || "p";
  }

  set value(v: string) {
    if (this._select && this._select.value !== v) {
      this._select.value = v;
    }
  }

  connectedCallback(): void {
    this.style.display = "inline-flex";

    const select = document.createElement("select");
    select.className =
      "bg-transparent border border-[var(--stroke-default,#333)] rounded px-1 text-[13px] h-8 cursor-pointer text-[var(--content-base,#fff)]";
    select.title = "Heading";

    for (const h of HEADINGS) {
      const opt = document.createElement("option");
      opt.value = h.value;
      opt.textContent = h.label;
      select.appendChild(opt);
    }

    select.addEventListener("change", () => {
      this.dispatchEvent(
        new CustomEvent("heading-change", {
          detail: select.value,
          bubbles: true,
          composed: true,
        }),
      );
    });

    this._select = select;
    this.appendChild(select);
  }

  attributeChangedCallback(
    name: string,
    _old: string | null,
    val: string | null,
  ): void {
    if (name === "value" && this._select && val !== null) {
      this._select.value = val;
    }
  }
}

if (!customElements.get("tiptap-heading-select")) {
  customElements.define("tiptap-heading-select", TiptapHeadingSelect);
}
