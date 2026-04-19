use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::{
    use_discussion_toc_context, DiscussionViewerTranslate,
};
use crate::features::spaces::pages::actions::actions::discussion::*;

#[component]
pub fn DiscussionToc() -> Element {
    let tr: DiscussionViewerTranslate = use_translate();
    let ctx = use_discussion_toc_context();
    let entries = ctx.headings.read().clone();

    #[cfg(feature = "web")]
    {
        let entries_dep = entries.clone();
        use_effect(move || {
            let _ = &entries_dep;
            setup_observer(ctx);
        });
    }

    if entries.len() < 3 {
        return rsx! {};
    }

    let active = ctx.active_id.read().clone();

    rsx! {
        nav {
            class: "sticky top-24 hidden max-h-[calc(100vh-8rem)] self-start overflow-y-auto desktop:block",
            "aria-label": "{tr.table_of_contents}",
            div { class: "flex flex-col gap-1.5 border-l border-border pl-3 text-xs text-foreground-muted",
                div { class: "mb-1 text-[10px] font-semibold uppercase tracking-wide text-foreground-muted",
                    "{tr.table_of_contents}"
                }
                for entry in entries.iter() {
                    {
                        let is_active = active.as_deref() == Some(entry.id.as_str());
                        let indent = match entry.level {
                            1 => "pl-0",
                            2 => "pl-3",
                            _ => "pl-6",
                        };
                        let class = format!(
                            "block truncate transition-colors hover:text-text-primary {indent} aria-current:font-medium aria-current:text-text-primary"
                        );
                        rsx! {
                            a {
                                key: "{entry.id}",
                                href: "#{entry.id}",
                                class: "{class}",
                                "aria-current": if is_active { "true" } else { "false" },
                                "{entry.text}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "web")]
fn setup_observer(
    mut ctx: crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionTocContext,
) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(node_list) = document.query_selector_all(
        ".discussion-content h1, .discussion-content h2, .discussion-content h3",
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
