use crate::*;

#[cfg(target_arch = "wasm32")]
fn toggle_lock_class(element: &web_sys::Element, lock: bool) {
    const LOCK_CLASSES: [&str; 3] = ["overflow-hidden", "overscroll-none", "touch-none"];

    let current = element.get_attribute("class").unwrap_or_default();
    let mut classes: Vec<String> = current.split_whitespace().map(|s| s.to_string()).collect();

    if lock {
        for cls in LOCK_CLASSES {
            if !classes.iter().any(|c| c == cls) {
                classes.push(cls.to_string());
            }
        }
    } else {
        classes.retain(|c| !LOCK_CLASSES.contains(&c.as_str()));
    }

    let next = classes.join(" ");
    if next.is_empty() {
        let _ = element.remove_attribute("class");
    } else {
        let _ = element.set_attribute("class", &next);
    }
}

pub fn use_scroll_lock(lock: bool) {
    use_effect(use_reactive((&lock,), move |(lock,)| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        toggle_lock_class(&html, lock);
                    }
                    if let Some(body) = document.body() {
                        let body: web_sys::Element = body.into();
                        toggle_lock_class(&body, lock);
                    }
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = lock;
        }
    }));
}
