# dioxus-icons

SVG icon components library for Dioxus. Automatically converts SVG files to type-safe Dioxus components at build time.

## Features

- **Build-time Generation**: SVG files are converted to components via `build.rs`
- **Type Safe**: Full Rust type checking
- **Tree-shakeable**: Unused icons don't appear in the final bundle
- **CSS Customizable**: Apply styles with the `class` prop
- **Auto-reload**: Components regenerate when SVG files change

## Usage

```rust
use dioxus::prelude::*;
use dioxus_icons::ratel::{Logo, LogoLetter};

fn app() -> Element {
    rsx! {
        Logo { }
        Logo { class: "w-8 h-8 text-blue-500" }
        LogoLetter { }
    }
}
```

## Adding Icons

1. Add SVG file to `resources/` directory
2. Run `cargo build`
3. Component is automatically generated and ready to use

Example: `resources/ratel/logo.svg` becomes `Logo` component.

```rust
use dioxus_icons::ratel::Logo;

rsx! {
    Logo { class: "w-6 h-6" }
}
```

## How It Works

The `build.rs` script:
1. Scans `resources/` directory for `.svg` files
2. Parses SVG structure using `roxmltree`
3. Converts SVG elements to Dioxus RSX
4. Generates `src/icons/mod.rs` with all components

Each icon component:
- Accepts optional `class` prop for CSS styling
- Preserves original SVG attributes (width, height, viewBox, fill, etc.)
- Renders as a Dioxus component with full type safety

## Available Icons

| Name | File | Size |
|------|------|------|
| `Logo` | `logo.svg` | 54×54 |
| `LogoLetter` | `logo_letter.svg` | 100×38 |
