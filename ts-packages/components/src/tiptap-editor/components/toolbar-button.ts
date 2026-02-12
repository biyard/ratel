const BASE =
  "flex items-center justify-center size-8 rounded transition-all text-[13px] leading-none cursor-pointer";
const INACTIVE =
  "bg-transparent border border-transparent text-[var(--content-base,#fff)] hover:bg-[var(--action-ghost-hover,rgba(0,0,0,0.2))]";
const ACTIVE =
  "bg-[color-mix(in_srgb,var(--action-primary,#fcb300)_10%,transparent)] border border-[color-mix(in_srgb,var(--action-primary,#fcb300)_20%,transparent)] text-[var(--action-primary,#fcb300)]";

export class TiptapToolbarButton extends HTMLElement {
  static get observedAttributes(): string[] {
    return ["active"];
  }

  connectedCallback(): void {
    this.setAttribute("role", "button");
    this.tabIndex = -1;
    this.innerHTML = this.getAttribute("label") || "";
    this.className = `${BASE} ${INACTIVE}`;

    const fs = this.getAttribute("font-size");
    if (fs) this.style.fontSize = fs;

    this.addEventListener("mousedown", (e) => e.preventDefault());
  }

  attributeChangedCallback(name: string): void {
    if (name === "active") {
      const on = this.getAttribute("active") === "true";
      const fs = this.style.fontSize;
      this.className = `${BASE} ${on ? ACTIVE : INACTIVE}`;
      if (fs) this.style.fontSize = fs;
    }
  }
}

if (!customElements.get("tiptap-toolbar-btn")) {
  customElements.define("tiptap-toolbar-btn", TiptapToolbarButton);
}
