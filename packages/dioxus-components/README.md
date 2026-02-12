# dioxus-components

`Web Component` wrappers for Dioxus.
Bundles TypeScript-based components (e.g. rich text editor) into JS assets
and exposes them as native Dioxus components.

All Components should be **Web Components**
https://developer.mozilla.org/en-US/docs/Web/API/Web_components

## Structure

- `ts-packages/components/src/` — TypeScript source (Custom Elements)
- `packages/dioxus-components/assets/` — Built JS bundles (IIFE)
- `packages/dioxus-components/src/` — Dioxus wrapper components

## Components

## Build 

```bash
cd packages/dioxus-components && make build_js

## Adding a New Component

1. Implement new components in `ts-packages/components/src/{name}/index.ts` with Custom Element implementation
2. Run `make build_js`. It will build the component and copy it to `assets/{name}.js`
3. Create `src/{name}.rs` with Dioxus wrapper
4. Register the module in `src/lib.rs`