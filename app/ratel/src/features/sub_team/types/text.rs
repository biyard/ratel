//! Tiny text helpers shared across sub-team pages.
//!
//! Sub-team documents are authored via a rich-text editor (Tiptap), so
//! `SubTeamDocument.body` is HTML (`<b>제 1조</b><div>...`). When we
//! want a short *preview* of that body — list cards, excerpts — we
//! can't just take the first N chars of the HTML (we'd surface raw
//! tags or, worse, an unclosed `<div>` if we tried `dangerous_inner_html`
//! on a truncated string). Strip tags first, then truncate.

/// Strip HTML tags + decode a small allowlist of common entities to
/// produce a plain-text preview from a rich-text doc body.
///
/// Not a sanitizer — the assumption is that the input was already
/// server-sanitized at write time. This only exists so list-card
/// previews don't render literal `<b>` and `<div>` text.
pub fn strip_html_to_plain(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut last_was_space = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                // Treat a closed tag as a soft whitespace boundary so
                // `<div>A</div><div>B</div>` collapses to `A B`, not `AB`.
                if !last_was_space && !out.is_empty() {
                    out.push(' ');
                    last_was_space = true;
                }
            }
            _ if in_tag => {}
            _ => {
                if c.is_whitespace() {
                    if !last_was_space && !out.is_empty() {
                        out.push(' ');
                        last_was_space = true;
                    }
                } else {
                    out.push(c);
                    last_was_space = false;
                }
            }
        }
    }
    out.trim_end()
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_bold_and_div_into_spaced_plain_text() {
        let html = "<b>제 1조 (목적)</b><div>본 학칙은 SNU CS Dept.</div>";
        assert_eq!(strip_html_to_plain(html), "제 1조 (목적) 본 학칙은 SNU CS Dept.");
    }

    #[test]
    fn collapses_consecutive_whitespace() {
        assert_eq!(strip_html_to_plain("<p>  a  \n b </p>"), "a b");
    }

    #[test]
    fn decodes_basic_entities() {
        assert_eq!(strip_html_to_plain("a &amp; b &lt;c&gt;"), "a & b <c>");
    }
}
