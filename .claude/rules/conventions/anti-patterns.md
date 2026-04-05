---
globs: ["app/ratel/**/*.rs"]
---

# Anti-patterns

Patterns to avoid across the Ratel codebase. Each item shows the bad pattern and its correct replacement.

## Navigation

```rust
// BAD — String path bypasses compile-time route validation
nav.push(format!("/spaces/{}/dashboard", space_id));
nav.replace(format!("/posts/{post_id}"));
Link { to: "/spaces/{space_id}/dashboard", "Dashboard" }

// GOOD — Route enum ensures valid routes at compile time
nav.push(Route::SpaceDashboardPage { space_id });
nav.replace(Route::PostDetailPage { post_id });
Link { to: Route::SpaceDashboardPage { space_id }, "Dashboard" }
```

## Styling

- `style="color: #fcb300"` — use semantic token class instead
- `style="background: #1a1a1a"` — use `bg-background` or `bg-card-bg`
- `text-neutral-400`, `bg-slate-800`, `text-gray-500` — use `text-foreground-muted`, `bg-card-bg`, `text-text-primary`
- `z-101` (silently ignored) — use `z-[101]` for arbitrary values

## Components

- Raw `<div class="flex ...">` for layouts — use `Row` or `Col` components
- Raw `<button>` — use `Button` component
- Raw `<input>` — use `Input` component
- Raw `<select>` — use `Select` component

## Props Cloning in Closures

```rust
// BAD — cloning props to pass into closures
fn MyComponent(tag: Vec<String>, on_remove: EventHandler<Vec<String>>) {
    onclick: {
        let tag = tag.clone();
        move |_| {
            on_remove.call(tag.clone());
        }
    },
}

// GOOD — use ReadSignal, which is Copy and avoids cloning
fn MyComponent(tag: ReadSignal<Vec<String>>, on_remove: EventHandler<Vec<String>>) {
    onclick: move |_| {
        on_remove.call(tag());
    },
}
```

Dioxus auto-converts `T` to `ReadSignal<T>` when passed as a prop. `ReadSignal` implements `Copy`, so it can be moved into closures without cloning.

## Async Event Handlers

```rust
// BAD — unnecessary spawn wrapping
onfocusout: move |_| {
    spawn(async move {
        // async work
    });
},

// GOOD — use async move directly
onfocusout: move |_| async move {
    // async work
},
```

## Error Handling

```rust
// BAD — leaks internal details to user
Error::BadRequest(format!("DynamoDB error: {e}"))

// GOOD — log detail server-side, return generic unit error
crate::error!("failed to create entity: {e}");
MyFeatureError::CreateFailed
```

## API Types

```rust
// BAD — exposes raw Partition/EntityType with prefix in API
fn update_poll(space_pk: Partition, poll_sk: EntityType)

// GOOD — uses SubPartition with id naming, no prefix
fn update_poll(space_id: SpacePartition, poll_id: SpacePollEntityType)
```

## i18n

```rust
// BAD — hardcoded string in UI
"Submit"

// GOOD — translated string
"{t.submit}"

// BAD — Display trait for enum in UI
status.to_string()

// GOOD — Translate trait for enum in UI
status.translate(&lang())
```
