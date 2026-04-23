//! Tiny HTML sanitization helpers shared between server-side content
//! processing (e.g. `essence::services::indexer`) and client-side display
//! (e.g. essence sources table). Comment content across the platform can
//! be rich-text HTML (`<p>...</p>`, `<strong>...`, etc.) and we want to
//! surface it as plain text in title-like fields without rendering raw
//! markup.
//!
//! The regex is compiled on every call. These helpers run on small
//! snippets (single comment bodies / titles, not full documents) so the
//! cost is negligible. If usage grows, wrap the pattern in `OnceLock`.

/// Strip every HTML tag (`<...>`) from the input, collapse runs of
/// whitespace into single spaces, and trim. Does NOT decode HTML
/// entities — only tags. Safe to call on strings that may or may not
/// contain markup.
///
/// Examples:
/// - `"<p>option 1</p>"` → `"option 1"`
/// - `"hello <strong>world</strong>"` → `"hello world"`
/// - `"plain text"` → `"plain text"`
pub fn strip_html_tags(input: &str) -> String {
    let re = regex::Regex::new(r"<[^>]+>").unwrap();
    let stripped = re.replace_all(input, " ");
    stripped
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convenience: strip HTML then truncate to `n` characters, appending
/// an ellipsis when truncated. Mirrors the per-feature `summarize`
/// helpers so callers can opt into the shared behavior.
pub fn summarize_plain(input: &str, n: usize) -> String {
    let plain = strip_html_tags(input);
    if plain.chars().count() <= n {
        return plain;
    }
    let mut out: String = plain.chars().take(n).collect();
    out.push('…');
    out
}
