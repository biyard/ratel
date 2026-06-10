use crate::common::components::editor::Editor as RichEditor;
use crate::common::components::use_is_mobile;
use crate::features::spaces::pages::actions::*;

/// Description editor used by the in-space action creators (discussion / quiz).
///
/// On desktop it renders the shared rich-text [`Editor`]. On mobile the rich
/// toolbar is cramped and fiddly, so it renders a plain `<textarea>` instead —
/// but the content is still stored as the **same HTML** the rich editor would
/// produce (each line wrapped in `<p>…</p>`), so what gets saved is identical
/// regardless of which control the author used. Existing HTML content is
/// converted back to plain text (block boundaries → newlines) for editing in
/// the textarea.
///
/// Snapshots its initial content once at mount, mirroring [`Editor`]; remount
/// via a parent `key` to push new content in (the discussion editor does this
/// on "import from overview").
#[component]
pub fn ActionDescriptionEditor(
    content: String,
    #[props(default = true)] editable: bool,
    #[props(default)] placeholder: String,
    #[props(default)] class: String,
    #[props(default)] on_content_change: Option<EventHandler<String>>,
) -> Element {
    let is_mobile = use_is_mobile();

    // Plain-text form of the initial HTML, snapshotted once (same contract as
    // the rich editor, which also snapshots `content` at mount).
    let initial_plain = use_hook(|| html_to_textarea(&content));
    let mut plain = use_signal(|| initial_plain.clone());

    if is_mobile() {
        rsx! {
            textarea {
                class: "action-desc-textarea {class}",
                "data-testid": "action-description-textarea",
                placeholder: "{placeholder}",
                readonly: !editable,
                value: "{plain()}",
                oninput: move |e| {
                    let text = e.value();
                    plain.set(text.clone());
                    if let Some(handler) = &on_content_change {
                        // Store the same tagged HTML the rich editor emits.
                        handler.call(textarea_to_html(&text));
                    }
                },
            }
        }
    } else {
        rsx! {
            RichEditor {
                class,
                content,
                editable,
                placeholder,
                on_content_change: move |html: String| {
                    if let Some(handler) = &on_content_change {
                        handler.call(html);
                    }
                },
            }
        }
    }
}

/// Convert editor HTML to plain text for the mobile textarea, preserving
/// paragraph/line breaks as newlines (unlike `strip_html_tags`, which collapses
/// all whitespace into single spaces and would flatten paragraphs).
fn html_to_textarea(html: &str) -> String {
    if html.trim().is_empty() {
        return String::new();
    }
    let mut s = html.to_string();
    // <br> variants → newline.
    s = regex::Regex::new(r"(?i)<br\s*/?>")
        .unwrap()
        .replace_all(&s, "\n")
        .into_owned();
    // Closing block tags → newline.
    s = regex::Regex::new(r"(?i)</(p|div|h[1-6]|li|blockquote)\s*>")
        .unwrap()
        .replace_all(&s, "\n")
        .into_owned();
    // Strip every remaining tag.
    s = regex::Regex::new(r"<[^>]+>")
        .unwrap()
        .replace_all(&s, "")
        .into_owned();
    // Decode the handful of entities the editor emits.
    s = s
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&");
    // Collapse 3+ blank lines, trim trailing/leading whitespace.
    s = regex::Regex::new(r"\n{3,}")
        .unwrap()
        .replace_all(&s, "\n\n")
        .into_owned();
    s.trim().to_string()
}

/// Convert plain textarea text back to the editor's HTML shape — each line
/// wrapped in `<p>…</p>` — so stored content matches what the rich editor
/// produces and renders identically in the viewer.
fn textarea_to_html(text: &str) -> String {
    if text.trim().is_empty() {
        return String::new();
    }
    text.split('\n')
        .map(|line| {
            let escaped = line
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            if escaped.trim().is_empty() {
                "<p></p>".to_string()
            } else {
                format!("<p>{escaped}</p>")
            }
        })
        .collect::<String>()
}
