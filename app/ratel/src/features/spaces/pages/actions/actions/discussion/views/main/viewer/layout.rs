use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::{
    DiscussionContentBody, DiscussionToc,
};
use crate::features::spaces::pages::actions::actions::discussion::*;

fn heading_count(html: &str) -> usize {
    // Count case-insensitive occurrences of `<h1`, `<h2`, `<h3` followed by `>` or whitespace.
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
pub fn NotionLayout(html_contents: String) -> Element {
    let has_toc = heading_count(&html_contents) >= 3;

    if has_toc {
        rsx! {
            div { class: "grid grid-cols-1 gap-8 desktop:grid-cols-[minmax(0,720px)_200px] desktop:gap-16 desktop:justify-center",
                DiscussionContentBody { html_contents }
                DiscussionToc {}
            }
        }
    } else {
        rsx! {
            div { class: "mx-auto w-full max-w-[720px]",
                DiscussionContentBody { html_contents }
            }
        }
    }
}
