enum Variant {
  Title1 = "title-1",
  Title2 = "title-2",
  Title3 = "title-3",
  H1 = "h1",
  H2 = "h2",
  H3 = "h3",
  H4 = "h4",
  Label1 = "label-1",
  Label2 = "label-2",
  Label3 = "label-3",
  Label4 = "label-4",
  Label5 = "label-5",
  Body1 = "body-1",
  Body2 = "body-2",
  Body3 = "body-3",
  Body4 = "body-4",
  Btn1 = "btn-1",
  Btn2 = "btn-2",
  Num1 = "num-1",
  Num2 = "num-2",
  Num3 = "num-3",
  TinyExt = "tiny-ext",
  Tiny = "tiny",
  Micro = "micro",
}

enum Weight {
  Extrabold = "extrabold",
  Bold = "bold",
  Semibold = "semibold",
  Medium = "medium",
  Regular = "regular",
}

const VARIANT_CLASSES: Record<Variant, string> = {
  [Variant.Title1]: "text-[64px]/[70px] tracking-[-0.8px]",
  [Variant.Title2]: "text-[40px]/[48px] tracking-[-0.64px]",
  [Variant.Title3]: "text-[32px]/[36px] tracking-[-0.6px]",
  [Variant.H1]: "text-[28px]/[32px] tracking-[-0.56px]",
  [Variant.H2]: "text-[26px]/[30px] tracking-[-0.26px]",
  [Variant.H3]: "text-[24px]/[28px] tracking-[-0.24px]",
  [Variant.H4]: "text-[20px]/[24px] tracking-[-0.2px]",
  [Variant.Label1]: "text-[17px]/[20px] tracking-[-0.18px]",
  [Variant.Label2]: "text-[15px]/[18px] tracking-[-0.16px]",
  [Variant.Label3]: "text-[13px]/[16px] tracking-[-0.14px]",
  [Variant.Label4]: "text-[12px]/[14px] tracking-[-0.12px]",
  [Variant.Label5]: "text-[11px]/[14px] tracking-[-0.1px]",
  [Variant.Body1]: "text-[17px]/[28px]",
  [Variant.Body2]: "text-[15px]/[22px]",
  [Variant.Body3]: "text-[13px]/[20px]",
  [Variant.Body4]: "text-[12px]/[16px]",
  [Variant.Btn1]: "text-[14px]/[16px]",
  [Variant.Btn2]: "text-[12px]/[14px]",
  [Variant.Num1]: "text-[17px]/[20px]",
  [Variant.Num2]: "text-[15px]/[22px]",
  [Variant.Num3]: "text-[12px]/[16px]",
  [Variant.TinyExt]: "text-[10px]/[12px] tracking-[1px]",
  [Variant.Tiny]: "text-[10px]/[12px]",
  [Variant.Micro]: "text-[9px]/[10px]",
};

const WEIGHT_CLASSES: Record<Weight, string> = {
  [Weight.Extrabold]: "font-extrabold",
  [Weight.Bold]: "font-bold",
  [Weight.Semibold]: "font-semibold",
  [Weight.Medium]: "font-medium",
  [Weight.Regular]: "font-normal",
};

class RatelTypo extends HTMLElement {
  private _appliedClasses: string[] = [];
  private _isInternalUpdating = false;
  static get observedAttributes() {
    return ["variant", "weight", "class"];
  }

  connectedCallback() {
    this._applyStyle();
  }

  attributeChangedCallback(name: string, oldValue: string, newValue: string) {
    if (this._isInternalUpdating) return;

    if (oldValue !== newValue) {
      this._applyStyle();
    }
  }

  private _applyStyle() {
    const variant = this.getAttribute("variant") as Variant;
    const weight = this.getAttribute("weight") as Weight;

    const variantClassString = VARIANT_CLASSES[variant] || "";
    const weightClassString = WEIGHT_CLASSES[weight] || "";

    const nextClasses = `${variantClassString} ${weightClassString}`
      .split(/\s+/)
      .filter(Boolean);

    this._isInternalUpdating = true;
    try {
      if (this._appliedClasses.length > 0) {
        this.classList.remove(...this._appliedClasses);
      }

      if (nextClasses.length > 0) {
        this.classList.add(...nextClasses);
      }

      this._appliedClasses = nextClasses;
    } finally {
      this._isInternalUpdating = false;
    }
  }
}

if (!customElements.get("ratel-typo")) {
  customElements.define("ratel-typo", RatelTypo);
}
