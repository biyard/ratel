---
globs: ["app/ratel/src/features/**/mod.rs"]
---

# Feature Module Structure

Each feature in `app/ratel/src/features/<module>/` follows:

```
mod.rs, route.rs, layout.rs
controllers/    - Server functions (#[get], #[post], etc.)
models/         - DynamoDB entities (feature: server)
components/     - Reusable UI
views/          - Page-level views
hooks/          - Dioxus hooks
i18n.rs         - Translations
types/          - Custom types + error enums
```

## File Placement

- **Shared primitives** → `app/ratel/src/common/components/<component_name>/mod.rs`
- **Feature UI** → `app/ratel/src/features/<feature>/components/<name>/`
- **Page-level views** → `app/ratel/src/features/<feature>/views/<page>/`

## HTML-First Pages with Sub-Components

For pages built with the HTML-first pattern (see `conventions/html-first-components.md`), each page owns its sub-components in named subdirectories:

```
pages/<page>/
├── mod.rs              # Page module
├── component.rs        # Page component
├── style.css           # Page styles
├── i18n.rs             # Shared translations
├── <sub_component>/    # Self-contained UI section
│   ├── mod.rs
│   ├── component.rs
│   └── style.css       # Own styles loaded via asset!()
└── ...
```

Extract into a sub-component when a section is self-contained (panel, modal, card) and > ~50 lines of RSX.
