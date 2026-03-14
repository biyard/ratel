export const TEXT_COLORS = [
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

export const HIGHLIGHT_COLORS = [
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

const BTN_CLS =
  "flex items-center justify-center size-8 rounded transition-all text-[13px] leading-none cursor-pointer bg-transparent border border-transparent text-[var(--content-base,#fff)] hover:bg-[var(--action-ghost-hover,rgba(0,0,0,0.2))]";

export class TiptapColorPicker extends HTMLElement {
  private _trigger: HTMLButtonElement | null = null;
  private _popup: HTMLDivElement | null = null;
  private _colors: string[] = [];
  private _onDocClick: (() => void) | null = null;

  set colors(val: string[]) {
    this._colors = val;
  }

  set triggerLabel(html: string) {
    if (this._trigger) this._trigger.innerHTML = html;
  }

  set triggerTitle(text: string) {
    if (this._trigger) this._trigger.title = text;
  }

  connectedCallback(): void {
    this.style.display = "inline-block";
    this.style.position = "relative";

    // Trigger button
    const trigger = document.createElement("button");
    trigger.type = "button";
    trigger.tabIndex = -1;
    trigger.innerHTML = "A";
    trigger.className = BTN_CLS;
    trigger.addEventListener("mousedown", (e) => e.preventDefault());
    trigger.addEventListener("click", (e) => {
      e.stopPropagation();
      this._togglePopup();
    });
    this._trigger = trigger;
    this.appendChild(trigger);

    // Popup
    const popup = document.createElement("div");
    popup.className =
      "hidden absolute top-full left-0 z-[999] bg-[var(--surface-raised,#1a1a1a)] border border-[var(--stroke-default,#333)] rounded-md p-2 shadow-lg w-[200px] flex-wrap gap-1";
    this._popup = popup;

    for (const c of this._colors) {
      const swatch = document.createElement("button");
      swatch.type = "button";
      swatch.className =
        "w-5 h-5 rounded-sm border border-black/15 cursor-pointer p-0";
      swatch.style.background = c;
      swatch.title = c;
      swatch.addEventListener("click", (e) => {
        e.stopPropagation();
        this._closePopup();
        this.dispatchEvent(
          new CustomEvent("color-select", {
            detail: c,
            bubbles: true,
            composed: true,
          }),
        );
      });
      popup.appendChild(swatch);
    }

    this.appendChild(popup);

    // Close popup on outside click
    this._onDocClick = () => this._closePopup();
    document.addEventListener("click", this._onDocClick);
  }

  disconnectedCallback(): void {
    if (this._onDocClick) {
      document.removeEventListener("click", this._onDocClick);
      this._onDocClick = null;
    }
  }

  private _togglePopup(): void {
    if (!this._popup) return;
    const isHidden = this._popup.classList.contains("hidden");
    if (isHidden) {
      this._popup.classList.remove("hidden");
      this._popup.classList.add("flex");
    } else {
      this._closePopup();
    }
  }

  private _closePopup(): void {
    if (!this._popup) return;
    this._popup.classList.add("hidden");
    this._popup.classList.remove("flex");
  }
}

if (!customElements.get("tiptap-color-picker")) {
  customElements.define("tiptap-color-picker", TiptapColorPicker);
}
