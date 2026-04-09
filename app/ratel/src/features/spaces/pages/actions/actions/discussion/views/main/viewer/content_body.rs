use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::{
    use_discussion_toc_context, TocEntry,
};
use crate::features::spaces::pages::actions::actions::discussion::*;

#[component]
pub fn DiscussionContentBody(html_contents: String) -> Element {
    if html_contents.is_empty() {
        return rsx! {};
    }

    #[cfg(not(feature = "server"))]
    {
        let toc = use_discussion_toc_context();
        let html_dep = html_contents.clone();
        use_effect(move || {
            let _ = &html_dep;
            collect_headings(toc);
        });
    }

    rsx! {
        div {
            class: "discussion-content prose prose-invert light:prose max-w-none text-text-primary [&>*]:rounded-md [&>*]:transition-colors [&>*]:duration-150 [&>*:hover]:-ml-2 [&>*:hover]:bg-hover [&>*:hover]:pl-2",
            dangerous_inner_html: "{html_contents}",
        }
    }
}

#[cfg(not(feature = "server"))]
fn collect_headings(
    mut toc: crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionTocContext,
) {
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
