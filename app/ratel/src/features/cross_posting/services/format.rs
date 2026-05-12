//! Per-platform body formatting and truncation (FR-5.5).
//!
//! Two functions, one for each call site in Stage 2's body resolver:
//! - [`format_for_platform`] — used when `SyndicationJob.body_override` is
//!   `None` (Phase 1 always; v1.5 when the user did NOT author a per-network
//!   variant). Builds the syndicated body from the canonical `Post`'s
//!   `title` + HTML-stripped `html_contents` + backlink.
//! - [`truncate_override`] — used when `body_override` is `Some(_)` (v1.5+).
//!   Free-text override goes through length-only truncation, preserves the
//!   trailing backlink.

use crate::features::cross_posting::types::SocialPlatform;
use crate::features::posts::models::Post;

const ELLIPSIS: char = '…';

/// Build a syndicated post body for a platform, fitting within
/// `platform.char_limit()`. The backlink is appended verbatim and is never
/// truncated (FR-5 #36 — backlink integrity is non-negotiable).
///
/// Order when over-budget: `{title}\n\n{first_sentence}…\n{backlink}`.
/// When under-budget: `{title}\n\n{full_body}\n\n{backlink}`.
///
/// If `{title}\n{backlink}` alone exceeds the budget, the title is the only
/// thing truncated; body is omitted entirely.
pub fn format_for_platform(post: &Post, platform: SocialPlatform, backlink: &str) -> String {
    let limit = platform.char_limit();
    let title = post.title.trim();
    let body = post.body.to_plain_text();

    // Reserve trailing "\n{backlink}" — backlink is non-truncatable.
    let suffix = format!("\n{backlink}");
    let suffix_chars = char_count(&suffix);

    if suffix_chars >= limit {
        // Pathological — backlink alone is over budget. Send raw backlink only.
        return backlink.to_string();
    }

    // Budget for {title} + "\n\n" + {body}
    let budget = limit - suffix_chars;

    // Try the under-budget path first: full title + double newline + full body.
    let full_prefix = if body.is_empty() {
        title.to_string()
    } else {
        format!("{title}\n\n{body}")
    };
    if char_count(&full_prefix) <= budget {
        return format!("{full_prefix}{suffix}");
    }

    // Over budget — truncate to first sentence + ellipsis.
    let title_block = if title.is_empty() {
        String::new()
    } else {
        format!("{title}\n\n")
    };
    let title_block_chars = char_count(&title_block);

    if title_block_chars >= budget {
        // Title alone exceeds the body budget. Drop body entirely; truncate
        // title hard at (budget - 1) chars + ellipsis.
        let truncated_title = take_chars(title, budget.saturating_sub(1));
        return format!("{truncated_title}{ELLIPSIS}{suffix}");
    }

    let body_budget = budget - title_block_chars;
    let first_sentence = first_sentence_of(&body);
    // -1 for the trailing ellipsis we'll append.
    let body_chars_to_take = body_budget.saturating_sub(1);
    let truncated_body = take_chars(&first_sentence, body_chars_to_take);

    format!("{title_block}{truncated_body}{ELLIPSIS}{suffix}")
}

/// Truncate a user-authored override body to fit `limit`, preserving the
/// trailing backlink (v1.5 path).
///
/// Strategy:
///   1. Reserve `backlink.len() + "\n…\n".len()` chars at the tail.
///   2. If body fits within `(limit - reserved)`, append `"\n{backlink}"` verbatim.
///   3. Else truncate body at `(limit - reserved)`, append `"…\n{backlink}"`.
pub fn truncate_override(body: String, backlink: &str, limit: usize) -> String {
    let suffix_short = format!("\n{backlink}"); // when body fits
    let suffix_long = format!("{ELLIPSIS}\n{backlink}"); // when body needs truncation

    let body_chars = char_count(&body);
    let suffix_short_chars = char_count(&suffix_short);

    if body_chars + suffix_short_chars <= limit {
        return format!("{body}{suffix_short}");
    }

    let suffix_long_chars = char_count(&suffix_long);
    if suffix_long_chars >= limit {
        // Pathological — backlink + ellipsis alone overflows. Send raw backlink only.
        return backlink.to_string();
    }
    let body_budget = limit - suffix_long_chars;
    let truncated = take_chars(&body, body_budget);
    format!("{truncated}{suffix_long}")
}

/// Strip HTML tags from rich-text body. Lightweight tag stripper — does NOT
/// fully parse HTML; just removes anything between `<` and `>` and decodes a
/// small set of common entities. Sufficient because Ratel's rich-text editor
/// produces well-formed HTML. Re-used by the dispatcher to build the
/// fallback description on Bluesky's rich-link card.
pub fn strip_html(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match (c, in_tag) {
            ('<', _) => in_tag = true,
            ('>', _) => in_tag = false,
            (ch, false) => out.push(ch),
            _ => {}
        }
    }
    decode_entities(&out).trim().to_string()
}

fn decode_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
}

/// Return the substring up to (and including) the first sentence terminator
/// (`.`, `?`, `!`) followed by whitespace or end-of-string. If no terminator
/// exists, return the whole string.
fn first_sentence_of(text: &str) -> String {
    let bytes = text.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        if matches!(*b, b'.' | b'?' | b'!') {
            let next = bytes.get(i + 1);
            if next.map_or(true, |n| n.is_ascii_whitespace()) {
                return text[..=i].to_string();
            }
        }
    }
    text.to_string()
}

fn char_count(s: &str) -> usize {
    s.chars().count()
}

fn take_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

// ─────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::ContentBody;
    use crate::features::posts::models::Post;

    fn make_post(title: &str, html: &str) -> Post {
        Post { title: title.to_string(), body: ContentBody::html(html), ..Default::default() }
    }

    // ── strip_html ──────────────────────────────────────────────────────
    #[test]
    fn strip_html_removes_tags() {
        assert_eq!(strip_html("<p>hello <b>world</b></p>"), "hello world");
    }

    #[test]
    fn strip_html_decodes_entities() {
        assert_eq!(strip_html("a &amp; b &lt;c&gt;"), "a & b <c>");
    }

    // ── first_sentence_of ───────────────────────────────────────────────
    #[test]
    fn first_sentence_finds_period() {
        assert_eq!(first_sentence_of("Hello world. Next sentence."), "Hello world.");
    }

    #[test]
    fn first_sentence_finds_question() {
        assert_eq!(first_sentence_of("What is this? Continue."), "What is this?");
    }

    #[test]
    fn first_sentence_returns_all_when_no_terminator() {
        assert_eq!(first_sentence_of("no terminator here"), "no terminator here");
    }

    // ── format_for_platform ─────────────────────────────────────────────
    #[test]
    fn format_under_budget_includes_full_body() {
        let post = make_post("Hi", "<p>Short body.</p>");
        let out =
            format_for_platform(&post, SocialPlatform::Bluesky, "https://r/p?utm_source=bluesky");
        assert!(out.contains("Hi"));
        assert!(out.contains("Short body."));
        assert!(out.ends_with("\nhttps://r/p?utm_source=bluesky"));
        assert!(!out.contains(ELLIPSIS));
    }

    #[test]
    fn format_over_budget_truncates_to_first_sentence_with_ellipsis() {
        let long_body = "x".repeat(500);
        let html = format!("<p>First sentence. {long_body}</p>");
        let post = make_post("Title", &html);
        let backlink = "https://r/p?utm_source=bluesky";
        let out = format_for_platform(&post, SocialPlatform::Bluesky, backlink);

        assert!(out.starts_with("Title\n\n"));
        assert!(out.contains(ELLIPSIS));
        assert!(out.ends_with(&format!("\n{backlink}")));
        assert!(char_count(&out) <= SocialPlatform::Bluesky.char_limit());
    }

    #[test]
    fn format_preserves_backlink_intact_under_pressure() {
        // Bluesky: 300 char limit. Build a body that pushes us deep into truncation.
        let post = make_post("T", &"a".repeat(2_000));
        let backlink = "https://example.com/very/long/canonical/path?utm_source=bluesky&extra=1";
        let out = format_for_platform(&post, SocialPlatform::Bluesky, backlink);
        assert!(out.ends_with(&format!("\n{backlink}")));
        assert!(char_count(&out) <= SocialPlatform::Bluesky.char_limit());
    }

    #[test]
    fn format_includes_utm_per_platform() {
        let post = make_post("T", "<p>body</p>");
        let bs = format_for_platform(&post, SocialPlatform::Bluesky, "https://r/p?utm_source=bluesky");
        let li = format_for_platform(&post, SocialPlatform::LinkedIn, "https://r/p?utm_source=linkedin");
        let th = format_for_platform(&post, SocialPlatform::Threads, "https://r/p?utm_source=threads");
        assert!(bs.contains("utm_source=bluesky"));
        assert!(li.contains("utm_source=linkedin"));
        assert!(th.contains("utm_source=threads"));
    }

    #[test]
    fn format_handles_unicode_correctly() {
        // Korean characters — byte length != char length. char-based budget
        // must apply, not byte-based.
        let post = make_post("제목", "<p>안녕하세요. 다음 문장입니다. 그리고 계속됩니다.</p>");
        let out =
            format_for_platform(&post, SocialPlatform::Bluesky, "https://r/p?utm_source=bluesky");
        assert!(out.contains("제목"));
        assert!(out.contains("안녕하세요."));
        assert!(char_count(&out) <= SocialPlatform::Bluesky.char_limit());
    }

    #[test]
    fn format_handles_title_only_when_body_empty() {
        let post = make_post("Just a title", "");
        let out =
            format_for_platform(&post, SocialPlatform::Bluesky, "https://r/p?utm_source=bluesky");
        assert!(out.starts_with("Just a title"));
        assert!(out.ends_with("\nhttps://r/p?utm_source=bluesky"));
    }

    // ── truncate_override ───────────────────────────────────────────────
    #[test]
    fn truncate_override_under_limit_keeps_body() {
        let out = truncate_override("short body".to_string(), "https://r/p", 100);
        assert_eq!(out, "short body\nhttps://r/p");
    }

    #[test]
    fn truncate_override_over_limit_appends_ellipsis_and_backlink() {
        let body = "a".repeat(500);
        let backlink = "https://r/p?utm_source=bluesky";
        let out = truncate_override(body, backlink, 300);
        assert!(out.contains(ELLIPSIS));
        assert!(out.ends_with(&format!("\n{backlink}")));
        assert!(char_count(&out) <= 300);
    }

    #[test]
    fn truncate_override_pathological_returns_backlink_only() {
        // Limit smaller than backlink + ellipsis → fall back to backlink only.
        let out = truncate_override("x".to_string(), "https://r/very/long/url", 5);
        assert_eq!(out, "https://r/very/long/url");
    }
}
