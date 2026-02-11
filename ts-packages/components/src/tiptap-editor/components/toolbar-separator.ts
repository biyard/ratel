export class TiptapToolbarSeparator extends HTMLElement {
  connectedCallback(): void {
    this.className =
      "inline-block w-px h-5 bg-[var(--stroke-default,#333)] mx-1 align-middle";
  }
}

if (!customElements.get("tiptap-separator")) {
  customElements.define("tiptap-separator", TiptapToolbarSeparator);
}
