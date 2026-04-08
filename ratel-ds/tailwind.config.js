/** @type {import('tailwindcss').Config} */
//
// All color values reference CSS custom properties from assets/tokens.css.
// That file is GENERATED from variants/ — do NOT add hardcoded values here.
// Variable names follow the flattened path of each token, e.g.:
//   generic.primary → --ratel-color-generic-primary
//   font.default    → --ratel-color-font-default

module.exports = {
  content: ["./src/**/*.rs", "./assets/**/*.html", "./index.html"],
  darkMode: ["class", "[data-theme='dark']"],

  theme: {
    fontFamily: {
      sans: ["var(--ratel-font-family)", "ui-sans-serif", "system-ui", "sans-serif"],
      mono: ["ui-monospace", "SFMono-Regular", "Menlo", "monospace"],
    },

    extend: {
      // ── Colors (all CSS-var backed) ───────────────────────────────────────
      colors: {
        ratel: {
          // Brand / generic
          primary:      "var(--ratel-color-generic-primary)",
          "primary-5":  "var(--ratel-color-generic-primary-opacity-5%)",
          "primary-10": "var(--ratel-color-generic-primary-opacity-10%)",
          "primary-15": "var(--ratel-color-generic-primary-opacity-15%)",
          "primary-25": "var(--ratel-color-generic-primary-opacity-25%)",
          "primary-50": "var(--ratel-color-generic-primary-opacity-50%)",
          "primary-75": "var(--ratel-color-generic-primary-opacity-75%)",

          // Status
          error:        "var(--ratel-color-generic-error)",
          "error-5":    "var(--ratel-color-generic-error-opacity-5%)",
          "error-10":   "var(--ratel-color-generic-error-opacity-10%)",
          info:         "var(--ratel-color-generic-info)",
          "info-5":     "var(--ratel-color-generic-info-opacity-5%)",
          "info-10":    "var(--ratel-color-generic-info-opacity-10%)",
          success:      "var(--ratel-color-generic-success)",
          "success-5":  "var(--ratel-color-generic-success-opacity-5%)",
          "success-10": "var(--ratel-color-generic-success-opacity-10%)",

          // Font / text
          text:         "var(--ratel-color-font-default)",
          "text-pri":   "var(--ratel-color-font-primary)",
          "text-head":  "var(--ratel-color-font-header)",
          "text-body":  "var(--ratel-color-font-body)",
          "text-dis":   "var(--ratel-color-font-disable)",
          "text-neut":  "var(--ratel-color-font-neutral-absolute)",
          "text-neut2": "var(--ratel-color-font-neutral-absolute2)",
          "text-black": "var(--ratel-color-font-black-absolute)",
          "text-white": "var(--ratel-color-font-white-absolute)",
          "inv-black":  "var(--ratel-color-font-invert-black)",
          "inv-white":  "var(--ratel-color-font-invert-white)",

          // Surface / background (from border.background tokens)
          // bg-white  → border.background.neutral-850 = #FFFFFF
          // bg-muted  → border.background.neutral-950 = #FAFAFA
          // bg-subtle → border.incard.background.default = #F5F5F5
          "bg-white":   "var(--ratel-color-border-background-neutral-850)",
          "bg-muted":   "var(--ratel-color-border-background-neutral-950)",
          "bg-subtle":  "var(--ratel-color-border-incard-background-default)",
          "bg-800":     "var(--ratel-color-border-background-neutral-800)",

          // Border / stroke / divider
          border:       "var(--ratel-color-border-stroke-neutral-800)",
          "border-pri": "var(--ratel-color-border-stroke-primary)",
          divider:      "var(--ratel-color-divider-netural-800)",
          "divider-dk": "var(--ratel-color-divider-netural-700)",
        },
      },

      // ── Border radius (from radius.json tokens) ───────────────────────────
      borderRadius: {
        "ratel-none": "var(--ratel-radius-none)",
        "ratel-xs":   "var(--ratel-radius-xs)",
        "ratel-sm":   "var(--ratel-radius-sm, 4px)",   // sm not emitted by filter — fallback
        "ratel-md":   "var(--ratel-radius-md)",
        "ratel-lg":   "var(--ratel-radius-lg, 8px)",
        "ratel-xl":   "var(--ratel-radius-xl)",
        "ratel-2xl":  "var(--ratel-radius-2xl)",
        "ratel-3xl":  "var(--ratel-radius-3xl)",
        "ratel-4xl":  "var(--ratel-radius-4xl)",
        "ratel-full": "var(--ratel-radius-full)",
      },

      // ── Border widths (from stroke.json) ──────────────────────────────────
      borderWidth: {
        "ratel-05":  "var(--ratel-stroke-0p5)",
        "ratel-075": "var(--ratel-stroke-0p75)",
        "ratel-1":   "var(--ratel-stroke-1)",
        "ratel-125": "var(--ratel-stroke-1p25)",
        "ratel-15":  "var(--ratel-stroke-1p5)",
        "ratel-2":   "var(--ratel-stroke-2)",
        "ratel-25":  "var(--ratel-stroke-2p5)",
        "ratel-3":   "var(--ratel-stroke-3)",
      },

      // ── Typography scale (token-named steps) ──────────────────────────────
      fontSize: {
        "title-1": ["var(--ratel-text-title-title-1-size)", { lineHeight: "var(--ratel-text-title-title-1-lh)", letterSpacing: "var(--ratel-text-title-title-1-ls)" }],
        "title-2": ["var(--ratel-text-title-title-2-size)", { lineHeight: "var(--ratel-text-title-title-2-lh)", letterSpacing: "var(--ratel-text-title-title-2-ls)" }],
        "title-3": ["var(--ratel-text-title-title-3-size)", { lineHeight: "var(--ratel-text-title-title-3-lh)", letterSpacing: "var(--ratel-text-title-title-3-ls)" }],
        "h1":      ["var(--ratel-text-heading-h1-size)",    { lineHeight: "var(--ratel-text-heading-h1-lh)",    letterSpacing: "var(--ratel-text-heading-h1-ls)" }],
        "h2":      ["var(--ratel-text-heading-h2-size)",    { lineHeight: "var(--ratel-text-heading-h2-lh)",    letterSpacing: "var(--ratel-text-heading-h2-ls)" }],
        "h3":      ["var(--ratel-text-heading-h3-size)",    { lineHeight: "var(--ratel-text-heading-h3-lh)",    letterSpacing: "var(--ratel-text-heading-h3-ls)" }],
        "h4":      ["var(--ratel-text-heading-h4-size)",    { lineHeight: "var(--ratel-text-heading-h4-lh)",    letterSpacing: "var(--ratel-text-heading-h4-ls)" }],
        "label-1": ["var(--ratel-text-label-label-1-size)", { lineHeight: "var(--ratel-text-label-label-1-lh)", letterSpacing: "var(--ratel-text-label-label-1-ls)" }],
        "label-2": ["var(--ratel-text-label-label-2-size)", { lineHeight: "var(--ratel-text-label-label-2-lh)", letterSpacing: "var(--ratel-text-label-label-2-ls)" }],
        "label-3": ["var(--ratel-text-label-label-3-size)", { lineHeight: "var(--ratel-text-label-label-3-lh)", letterSpacing: "var(--ratel-text-label-label-3-ls)" }],
        "label-4": ["var(--ratel-text-label-label-4-size)", { lineHeight: "var(--ratel-text-label-label-4-lh)", letterSpacing: "var(--ratel-text-label-label-4-ls)" }],
        "label-5": ["var(--ratel-text-label-label-5-size)", { lineHeight: "var(--ratel-text-label-label-5-lh)", letterSpacing: "var(--ratel-text-label-label-5-ls)" }],
        "body-1":  ["var(--ratel-text-body-body-1-size)",   { lineHeight: "var(--ratel-text-body-body-1-lh)",   letterSpacing: "0px" }],
        "body-2":  ["var(--ratel-text-body-body-2-size)",   { lineHeight: "var(--ratel-text-body-body-2-lh)",   letterSpacing: "0px" }],
      },

      // ── Spacing extras ────────────────────────────────────────────────────
      spacing: {
        "ratel-4":   "var(--ratel-space-4)",
        "ratel-6":   "var(--ratel-space-6)",
        "ratel-8":   "var(--ratel-space-8)",
        "ratel-10":  "var(--ratel-space-10)",
        "ratel-12":  "var(--ratel-space-12)",
        "ratel-16":  "var(--ratel-space-16)",
        "ratel-20":  "var(--ratel-space-20)",
        "ratel-24":  "var(--ratel-space-24)",
        "ratel-32":  "var(--ratel-space-32)",
        "ratel-40":  "var(--ratel-space-40)",
        "ratel-48":  "var(--ratel-space-48)",
        "ratel-56":  "var(--ratel-space-56)",
        "ratel-64":  "var(--ratel-space-64)",
        "ratel-80":  "var(--ratel-space-80)",
        "ratel-96":  "var(--ratel-space-96)",
        "ratel-128": "var(--ratel-space-128)",
        "ratel-160": "var(--ratel-space-160)",
      },
    },
  },

  plugins: [],
};
