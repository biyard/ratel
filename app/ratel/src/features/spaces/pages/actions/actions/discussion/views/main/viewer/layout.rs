use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionContentBody;
use crate::features::spaces::pages::actions::actions::discussion::*;

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
pub fn NotionLayout(html_contents: String) -> Element {
    rsx! {
        DiscussionContentBody { html_contents }
    }
}
