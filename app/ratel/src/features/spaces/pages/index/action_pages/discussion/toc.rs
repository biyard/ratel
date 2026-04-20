use crate::common::*;
use crate::features::spaces::pages::index::action_pages::discussion::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TocEntry {
    pub id: String,
    pub text: String,
    pub level: u8,
}

#[derive(Clone, Copy)]
pub struct DiscussionTocContext {
    pub headings: Signal<Vec<TocEntry>>,
    pub active_id: Signal<Option<String>>,
}

pub fn use_discussion_toc_context() -> DiscussionTocContext {
    use_context()
}

impl DiscussionTocContext {
    pub fn init() -> Self {
        let ctx = Self {
            headings: Signal::new(Vec::new()),
            active_id: Signal::new(None),
        };
        use_context_provider(|| ctx);
        ctx
    }
}

// Cheap raw-HTML scan for `<h1|h2|h3>` openings so we can decide whether to
// reserve the TOC column before the DOM-walking collector has run.
pub fn heading_count(html: &str) -> usize {
    let bytes = html.as_bytes();
    let mut count = 0usize;
    let mut i = 0usize;
    while i + 3 < bytes.len() {
        if bytes[i] == b'<'
            && (bytes[i + 1] == b'h' || bytes[i + 1] == b'H')
            && matches!(bytes[i + 2], b'1' | b'2' | b'3')
            && (bytes[i + 3] == b'>' || bytes[i + 3].is_ascii_whitespace())
        {
            count += 1;
            i += 4;
        } else {
            i += 1;
        }
    }
    count
}

#[component]
pub fn DiscussionToc() -> Element {
    let tr: DiscussionArenaTranslate = use_translate();
    let ctx = use_discussion_toc_context();
    let entries = ctx.headings.read().clone();

    #[cfg(feature = "web")]
    {
        use_effect(move || {
            // Reactive read: re-run the IntersectionObserver wiring whenever
            // the heading list is rebuilt (e.g. after content edit).
            let _ = ctx.headings.read();
            setup_observer(ctx);
        });
    }

    if entries.len() < 3 {
        return rsx! {};
    }

    let active = ctx.active_id.read().clone();

    rsx! {
        nav { class: "discussion-toc", "aria-label": "{tr.table_of_contents}",
            div { class: "discussion-toc__title", "{tr.table_of_contents}" }
            for entry in entries.iter() {
                {
                    let is_active = active.as_deref() == Some(entry.id.as_str());
                    let level_class = match entry.level {
                        1 => "discussion-toc__link--l1",
                        2 => "discussion-toc__link--l2",
                        _ => "discussion-toc__link--l3",
                    };
                    rsx! {
                        a {
                            key: "{entry.id}",
                            href: "#{entry.id}",
                            class: "discussion-toc__link {level_class}",
                            "aria-current": if is_active { "true" } else { "false" },
                            "{entry.text}"
                        }
                    }
                }
            }
        }
    }
}

// Walks `.disc-body__content h1|h2|h3`, assigns stable ids, and publishes the
// list to the context. Runs after Dioxus applies `dangerous_inner_html`, so
// the headings are guaranteed to be in the DOM by the time this fires.
#[cfg(feature = "web")]
pub fn collect_headings(mut toc: DiscussionTocContext) {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(node_list) = document.query_selector_all(
        ".disc-body__content h1, .disc-body__content h2, .disc-body__content h3",
    ) else {
        return;
    };
    let mut entries: Vec<TocEntry> = Vec::new();
    for i in 0..node_list.length() {
        let Some(node) = node_list.item(i) else {
            continue;
        };
        let Ok(el) = node.dyn_into::<web_sys::Element>() else {
            continue;
        };
        let id = format!("toc-heading-{i}");
        let _ = el.set_attribute("id", &id);
        let tag = el.tag_name().to_ascii_lowercase();
        let level: u8 = match tag.as_str() {
            "h1" => 1,
            "h2" => 2,
            _ => 3,
        };
        let text = el.text_content().unwrap_or_default().trim().to_string();
        if text.is_empty() {
            continue;
        }
        entries.push(TocEntry { id, text, level });
    }
    toc.headings.set(entries);
}

#[cfg(feature = "web")]
fn setup_observer(mut ctx: DiscussionTocContext) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(node_list) = document.query_selector_all(
        ".disc-body__content h1, .disc-body__content h2, .disc-body__content h3",
    ) else {
        return;
    };
    if node_list.length() == 0 {
        return;
    }

    let callback =
        Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
            for i in 0..entries.length() {
                let Ok(entry) = entries
                    .get(i)
                    .dyn_into::<web_sys::IntersectionObserverEntry>()
                else {
                    continue;
                };
                if entry.is_intersecting() {
                    let target = entry.target();
                    let id = target.id();
                    if !id.is_empty() {
                        ctx.active_id.set(Some(id));
                        break;
                    }
                }
            }
        });

    let options = web_sys::IntersectionObserverInit::new();
    options.set_root_margin("-20% 0px -70% 0px");
    let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    ) else {
        return;
    };

    for i in 0..node_list.length() {
        if let Some(node) = node_list.item(i) {
            if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                observer.observe(&el);
            }
        }
    }

    // Leak the closure so the observer stays alive for the component's lifetime.
    callback.forget();
}
