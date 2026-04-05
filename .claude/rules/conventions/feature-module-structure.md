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
