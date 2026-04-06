# Figma MCP Integration Rules

These rules define how to translate Figma designs into code for this project. Follow them for every Figma-driven change.

## Required Flow (do not skip)

1. Run `get_design_context` to fetch the structured representation of the target node(s)
2. If the response is too large or truncated, run `get_metadata` to get the node map, then re-fetch only the required nodes
3. Run `get_screenshot` for a visual reference of the exact variant being implemented
4. Only after you have both `get_design_context` and `get_screenshot`, start implementation
5. Translate the output (React + Tailwind reference) into Dioxus RSX using this project's components, tokens, and patterns
6. Validate visually against the Figma screenshot for 1:1 parity before marking complete

## Framework & Language

- **Language**: Rust (edition 2024)
- **UI Framework**: Dioxus 0.7 fullstack (RSX macro, not JSX)
- **Styling**: TailwindCSS v4 utility classes inside `class: "..."` string in RSX
- Figma MCP output is React + Tailwind — always translate to Dioxus RSX patterns

## Component Library

- **IMPORTANT**: Always check `app/ratel/src/common/components/` first before creating new elements
- Available primitives: `Accordion`, `AlertDialog`, `AspectRatio`, `Avatar`, `Badge`, `Button`, `Calendar`, `Card`, `Checkbox`, `Col`, `Collapsible`, `ContextMenu`, `DatePicker`, `Dialog`, `DragAndDropList`, `DropdownMenu`, `FileUploader`, `Form`, `HoverCard`, `Input`, `Label`, `Layover`, `LoadingIndicator`, `Menubar`, `Navbar`, `Pagination`, `Popover`, `Popup`, `Progress`, `RadioGroup`, `Row`, `ScrollArea`, `Select`, `SeoMeta`, `Separator`, `Sheet`, `Sidebar`, `Sidemenu`, `Skeleton`, `Slider`, `SpaceCard`, `SuspenseBoundary`, `Switch`, `Tabs`, `Textarea`, `ThemeSwitcher`, `Toggle`, `ToggleGroup`, `Toolbar`, `Tooltip`
- All components are available via `use crate::common::*;`

### Key Component Prop Patterns

**Button**:
```rust
Button {
    size: ButtonSize::Medium,    // Medium | Inline | Icon | Small
    style: ButtonStyle::Primary, // Primary | Secondary | Outline | Text
    shape: ButtonShape::Rounded, // Rounded | Square
    disabled: false,
    onclick: move |_| { ... },
    "Label"
}
```

**Card**:
```rust
Card {
    variant: CardVariant::Normal,          // Normal | Outlined | Filled
    direction: CardDirection::Col,         // Col | Row
    main_axis_align: MainAxisAlign::Start,
    cross_axis_align: CrossAxisAlign::Start,
    class: "extra-tailwind-classes",
    // children
}
```

**Row / Col** (flex layout primitives):
```rust
Row {
    main_axis_align: MainAxisAlign::Between,
    cross_axis_align: CrossAxisAlign::Center,
    class: "w-full gap-2",
    // children
}
Col {
    main_axis_align: MainAxisAlign::Start,
    cross_axis_align: CrossAxisAlign::Stretch,
    // children
}
```

**Badge**:
```rust
Badge {
    color: BadgeColor::Green, // Grey | Blue | Green | Orange | Pink | Purple | Red
    size: BadgeSize::Normal,
    variant: BadgeVariant::Default, // Default | Rounded
    "Label"
}
```

**Input**:
```rust
Input {
    variant: InputVariant::Default, // Default | Plain
    r#type: InputType::Text,
    placeholder: "...",
    oninput: move |e| { ... },
    onconfirm: move |_| { ... }, // Enter key
}
```

## Design Tokens

- **IMPORTANT**: Never hardcode hex colors — always use semantic token classes
- Token definitions: `app/ratel/tailwind.css` + `app/ratel/assets/tailwind-colors.css`
- Component theme: `app/ratel/assets/dx-components-theme.css`

### Semantic Color Classes

| Purpose | Tailwind Class |
|---------|----------------|
| Page background | `bg-background` |
| Card background | `bg-card-bg` |
| Popover/modal bg | `bg-popover` |
| Primary text | `text-text-primary` / `text-foreground` |
| Muted text | `text-foreground-muted` |
| Primary action | `bg-primary` / `text-primary` (golden #fcb300) |
| Accent | `bg-accent` / `text-accent` (teal #6eedd8) |
| Destructive | `bg-destructive` / `text-destructive` (#db2780) |
| Border | `border-border` / `border-separator` |
| Input background | `bg-input-box-bg` |
| Input border | `border-input-box-border` |
| Icon color | `[&>path]:stroke-icon-primary` |
| Focus ring | `focus-visible:border-ring focus-visible:ring-ring/50` |
| Button primary | `bg-btn-primary-bg text-btn-primary-text` |

### Theme System

- Dark/light via `data-theme` attribute on `<html>` — no conditional rendering needed in Rust
- Custom TailwindCSS v4 variant: `light:` prefix for light-mode-only styles
- All semantic token classes automatically respond to theme switching

### Breakpoints

```
max-mobile:   ≤ 500px   (use max-mobile: prefix)
md:           ≥ 768px   (standard Tailwind)
desktop:      ≥ 1177px
```

## Component Structure Pattern

New components follow this structure:

```rust
use crate::common::*;

#[component]
pub fn MyComponent(
    #[props(default)] class: String,
    #[props(default)] variant: MyVariant,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "base-classes {variant} {class}",
            ..attributes,
            {children}
        }
    }
}

#[derive(Default, strum::Display, strum::EnumString)]
pub enum MyVariant {
    #[default]
    #[strum(serialize = "tailwind-classes-for-default")]
    Default,
    #[strum(serialize = "tailwind-classes-for-other")]
    Other,
}
```

Key rules:
- `#[props(default)]` for optional props
- `#[props(extends = GlobalAttributes)]` for HTML attribute passthrough + spread with `..attributes`
- Enum variants serialize directly to Tailwind class strings via `strum::Display`
- Use `{variant}` string interpolation in the class attribute

## Icon System

- Custom icons: `crate::common::icons::<category>::<IconName> { class: "..." }`
- Lucide icons: `lucide_dioxus::<IconName> { class: "..." }`
- **IMPORTANT**: Do NOT install new icon packages — use existing icon libraries
- Color icons via SVG path targeting: `[&>path]:stroke-icon-primary`
- Available categories: `validations`, `arrows`, `user`, `settings`, `edit`, `files`, `graph`, `notifications`, `wallet`, `calendar`, `ratel` (custom), and 40+ more in `packages/icons/src/`

## Asset Handling

- **IMPORTANT**: If Figma MCP returns a localhost image/SVG source, use it directly — do not create placeholders
- Declare static assets: `pub const MY_ASSET: Asset = asset!("/assets/filename.ext");`
- Asset files go in `app/ratel/assets/`
- Reference in RSX: `img { src: MY_ASSET }` or `document::Link { href: MY_ASSET }`

## i18n (Translations)

- All user-facing strings MUST use the `translate!` macro — no hardcoded English strings
- Define translations in `i18n.rs` within the component/feature directory:

```rust
translate! {
    MyComponentTranslate;
    title: {
        en: "Title",
        ko: "제목",
    },
}
```

- Use in component: `let t = MyComponentTranslate::new(use_locale());` then `"{t.title}"`

## Feature Module Placement

New UI from Figma goes in:
- **Shared primitives** → `app/ratel/src/common/components/<component_name>/mod.rs`
- **Feature UI** → `app/ratel/src/features/<feature>/components/<name>/`
- **Page-level views** → `app/ratel/src/features/<feature>/views/<page>/`

## Styling Anti-patterns (avoid)

- `style="color: #fcb300"` — use semantic token class instead
- `style="background: #1a1a1a"` — use `bg-background` or `bg-card-bg`
- Raw `<div class="flex ...">` for layouts — use `Row` or `Col` components
- Raw `<button>` — use `Button` component
- Raw `<input>` — use `Input` component
- `class: "gap-4 p-5 rounded-[10px]"` — Tailwind spacing/sizing is fine to use directly
