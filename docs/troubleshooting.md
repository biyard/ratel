# Troubleshooting

## Dioxus Async Event Handlers Silently Cancelled When Component Unmounts

**Symptom:** An async event handler calls JavaScript via `wasm_bindgen` / `JsFuture`, but the code after `.await` never executes — even when the JS Promise resolves successfully. No error is logged.

**Root Cause:** In Dioxus, async event handlers (`onclick: move |_| async move { ... }`) are tied to the component's scope. If the component unmounts while the future is still pending, the runtime drops the future and it never resumes.

A common trigger is calling `popup.close()` (or any action that removes the component from the DOM) **before** the `.await` point:

```rust
// BAD — popup.close() unmounts this component, cancelling the future
onclick: move |_| async move {
    popup.close(); // component unmounts here
    let result = some_js_call().await; // never reached
    // ...
}
```

**Fix:** Move any action that unmounts the component to **after** the async work completes:

```rust
// GOOD — component stays mounted until the future resolves
onclick: move |_| async move {
    match some_js_call().await {
        Ok(value) => {
            popup.close(); // safe to unmount now
            on_complete(value);
        }
        Err(err) => {
            popup.close();
            toast.error(err.to_string());
        }
    }
}
```

**General Rule:** In a Dioxus async event handler, never call `popup.close()`, navigate away, or otherwise cause the owning component to unmount before all `.await` points have resolved.

**Affected file (example):** `app/socials/users/pages/credential/src/components/verification_method_modal.rs` — PortOne identity verification was silently dropped because the modal closed before `JsFuture::from(promise).await` could complete.

