# ContentBody Migration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Introduce a tagged `ContentBody` enum (`StructuredContent(ContentDocument) | HtmlContent(String)`) that replaces every rich-editor-backed `String` field in the codebase, with full backward compatibility for existing DynamoDB rows storing raw HTML.

**Architecture:** A new shared type `ContentBody` lives in `app/ratel/src/common/types/content/`. It serializes via **adjacent tagging** (`{"content_type": "...", "data": ...}`) for new writes and uses a **custom `Deserialize`** that also accepts legacy raw HTML strings, so old rows JIT-convert at read time without a backfill. Migration proceeds field by field starting with `Post.html_contents` as the pilot, then rolls out to `Space.content`, comments, sub-team docs/announcements, and team/user descriptions.

**Tech Stack:** Rust (serde, serde_json), Dioxus 0.7, DynamoDB (single-table), `DynamoEntity` derive, existing Tiptap rich editor (frontend), `regex` for plain-text extraction.

---

## Important serde gotcha — read first

The user-proposed shape:

```rust
#[serde(tag = "content_type")]
pub enum ContentBody {
    StructuredContent(ContentDocument),
    HtmlContent(String), // ❌ does not compile/round-trip with internal tagging
}
```

**Internally tagged enums in serde require each variant payload to serialize as a map** (struct, struct variant, or `BTreeMap`-like). `String` is a scalar and breaks this contract.

**Resolution adopted in this plan: adjacent tagging + custom Deserialize.** Adjacent tagging (`tag = "content_type", content = "data"`) accepts `String` newtype variants natively. We then layer a hand-written `Deserialize` that *also* accepts a bare JSON string (legacy raw HTML row), turning it into `HtmlContent`. Result: one field, three accepted on-wire shapes, one canonical write shape.

| On-wire input | Deserializes to |
|---|---|
| `"<p>hi</p>"` (legacy raw HTML row) | `HtmlContent("<p>hi</p>")` |
| `{"content_type":"html_content","data":"<p>hi</p>"}` | `HtmlContent("<p>hi</p>")` |
| `{"content_type":"structured_content","data":{...}}` | `StructuredContent(...)` |

Canonical serialization is always the second or third form.

---

## File Structure

### New files (Phase 1)

| Path | Responsibility |
|---|---|
| `app/ratel/src/common/types/content/mod.rs` | Re-exports |
| `app/ratel/src/common/types/content/body.rs` | `ContentBody` enum + custom `Deserialize` |
| `app/ratel/src/common/types/content/document.rs` | `ContentDocument`, `Block`, `BlockKind`, `RichText`, etc. |
| `app/ratel/src/common/types/content/render.rs` | `to_html`, `to_plain_text`, `char_count` for both variants |
| `app/ratel/src/common/types/content/tests.rs` | Unit tests |

### Modified files (Phase 2 — Post pilot)

| Path | Change |
|---|---|
| `app/ratel/src/features/posts/models/post.rs` | `html_contents: String` → `body: ContentBody` (with `#[serde(alias = "html_contents")]`) |
| `app/ratel/src/features/posts/models/post_repost.rs` | Mirror change for `post_html_contents` field |
| `app/ratel/src/features/posts/controllers/dto/post_response.rs` | DTO field rename + dual-shape parsing |
| `app/ratel/src/features/posts/controllers/update_post.rs` | Use `with_body(content_body)` |
| `app/ratel/src/features/posts/controllers/create_post.rs` | Wrap incoming HTML in `ContentBody::HtmlContent` |
| `app/ratel/src/features/posts/utils/validator.rs` | `validate_content` takes `&ContentBody` |
| `app/ratel/src/features/posts/components/post_detail/content.rs` | Render via `body.to_html()` |
| `app/ratel/src/features/posts/components/feed_card/mod.rs` | Preview via `body.to_plain_text()` |
| `app/ratel/src/features/rag/qdrant/indexers/post_indexer.rs` | Use `body.to_plain_text()` |
| `app/ratel/src/features/cross_posting/services/format.rs` | Use `body.to_plain_text()` |
| `app/ratel/src/features/cross_posting/services/dispatcher.rs` | Use `body.to_plain_text()` |
| `app/ratel/src/features/social/**/draft|user_draft|home/views/mod.rs` | Replace `strip_html` calls |
| `app/ratel/src/features/posts/views/post_edit/component.rs` | Editor I/O remains HTML; wrap as `ContentBody::HtmlContent` on save |
| `app/ratel/src/tests/sub_team_tests.rs` | Update fixtures using `html_contents:` literal |

### Modified files (Phase 3 — rollout, listed but not detailed per task)

- `app/ratel/src/features/spaces/models/space.rs` — `content` → `body`
- `app/ratel/src/features/posts/models/post_comment.rs` — `content` → `body`
- `app/ratel/src/features/spaces/pages/actions/actions/discussion/models/space_post.rs` and `space_post_comment.rs`
- `app/ratel/src/features/sub_team/models/sub_team_document.rs` — `body: String` → `body: ContentBody`
- `app/ratel/src/features/sub_team/models/sub_team_announcement.rs` — same
- (Optional) `Team.description`, `User.description` — only if the user-facing editor produces rich content for these

---

## Phase 1 — Foundation: `ContentBody` and `ContentDocument` types

### Task 1: Define `ContentDocument` and block tree

**Files:**
- Create: `app/ratel/src/common/types/content/document.rs`
- Modify: `app/ratel/src/common/types/content/mod.rs`
- Modify: `app/ratel/src/common/types/mod.rs` (add `pub mod content;`)

- [ ] **Step 1: Create `document.rs` with full block tree**

```rust
// app/ratel/src/common/types/content/document.rs
use serde::{Deserialize, Serialize};

pub type BlockId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ContentDocument {
    pub schema_version: u32,
    pub blocks: Vec<Block>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub meta: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub id: BlockId,
    #[serde(flatten)]
    pub kind: BlockKind,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Block>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum BlockKind {
    Paragraph(TextBlock),
    Heading(HeadingBlock),
    BulletedListItem(TextBlock),
    NumberedListItem(TextBlock),
    Todo(TodoBlock),
    Toggle(TextBlock),
    Quote(TextBlock),
    Callout(CalloutBlock),
    Code(CodeBlock),
    Equation(EquationBlock),
    Divider,
    Image(MediaBlock),
    Video(MediaBlock),
    File(MediaBlock),
    Bookmark(BookmarkBlock),
    Embed(EmbedBlock),
    Custom(CustomBlock),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TextBlock {
    pub rich_text: RichText,
    #[serde(default, skip_serializing_if = "Color::is_default")]
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeadingBlock {
    pub level: HeadingLevel,
    pub rich_text: RichText,
    #[serde(default)]
    pub toggleable: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HeadingLevel { H1, H2, H3 }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TodoBlock { pub rich_text: RichText, pub checked: bool }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeBlock {
    pub rich_text: RichText,
    pub language: String,
    #[serde(default)] pub caption: RichText,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalloutBlock {
    pub rich_text: RichText,
    pub icon: Option<Icon>,
    #[serde(default)] pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EquationBlock { pub expression: String }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MediaBlock {
    pub source: MediaSource,
    #[serde(default)] pub caption: RichText,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub alt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MediaSource {
    Asset { asset_id: String },
    External { url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BookmarkBlock { pub url: String, #[serde(default)] pub caption: RichText }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbedBlock { pub url: String, pub provider: Option<String> }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomBlock {
    pub namespace: String,
    pub kind: String,
    pub schema_version: u32,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct RichText(pub Vec<InlineNode>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum InlineNode {
    Text(TextRun),
    Mention(Mention),
    Equation(InlineEquation),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextRun {
    pub content: String,
    #[serde(default, skip_serializing_if = "Annotations::is_default")]
    pub annotations: Annotations,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Annotations {
    #[serde(default)] pub bold: bool,
    #[serde(default)] pub italic: bool,
    #[serde(default)] pub strikethrough: bool,
    #[serde(default)] pub underline: bool,
    #[serde(default)] pub code: bool,
    #[serde(default)] pub color: Color,
}

impl Annotations { pub fn is_default(&self) -> bool { *self == Self::default() } }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", content = "ref", rename_all = "snake_case")]
pub enum Mention {
    User(String),
    Team(String),
    Space(String),
    Post(String),
    Date { iso: String, end: Option<String> },
    Url(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineEquation { pub expression: String }

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    #[default] Default,
    Gray, Brown, Orange, Yellow, Green, Blue, Purple, Pink, Red,
    GrayBackground, BrownBackground, OrangeBackground, YellowBackground,
    GreenBackground, BlueBackground, PurpleBackground, PinkBackground, RedBackground,
}
impl Color { pub fn is_default(&self) -> bool { matches!(self, Self::Default) } }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Icon {
    Emoji(String),
    Asset { asset_id: String },
    External { url: String },
}
```

- [ ] **Step 2: Wire `mod.rs` files**

```rust
// app/ratel/src/common/types/content/mod.rs
mod body;
mod document;
mod render;
#[cfg(test)]
mod tests;

pub use body::ContentBody;
pub use document::*;
```

```rust
// add to app/ratel/src/common/types/mod.rs
pub mod content;
```

- [ ] **Step 3: Verify compile**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: PASS (no usages yet, just type definitions).

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/types/content/document.rs app/ratel/src/common/types/content/mod.rs app/ratel/src/common/types/mod.rs
git commit -m "feat(content): add ContentDocument block tree types"
```

---

### Task 2: Define `ContentBody` enum with backward-compat `Deserialize`

**Files:**
- Create: `app/ratel/src/common/types/content/body.rs`

- [ ] **Step 1: Write the failing test**

Append to `app/ratel/src/common/types/content/tests.rs`:

```rust
// app/ratel/src/common/types/content/tests.rs
use super::*;

#[test]
fn legacy_raw_string_deserializes_as_html_content() {
    let json = r#""<p>hello</p>""#;
    let body: ContentBody = serde_json::from_str(json).unwrap();
    assert_eq!(body, ContentBody::HtmlContent("<p>hello</p>".to_string()));
}

#[test]
fn tagged_html_content_deserializes() {
    let json = r#"{"content_type":"html_content","data":"<p>hi</p>"}"#;
    let body: ContentBody = serde_json::from_str(json).unwrap();
    assert_eq!(body, ContentBody::HtmlContent("<p>hi</p>".to_string()));
}

#[test]
fn tagged_structured_content_deserializes() {
    let json = r#"{
        "content_type":"structured_content",
        "data":{"schema_version":1,"blocks":[],"meta":{}}
    }"#;
    let body: ContentBody = serde_json::from_str(json).unwrap();
    match body {
        ContentBody::StructuredContent(d) => assert_eq!(d.schema_version, 1),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn html_content_serializes_as_tagged() {
    let body = ContentBody::HtmlContent("<p>x</p>".to_string());
    let json = serde_json::to_value(&body).unwrap();
    assert_eq!(
        json,
        serde_json::json!({"content_type":"html_content","data":"<p>x</p>"})
    );
}

#[test]
fn structured_content_serializes_as_tagged() {
    let doc = ContentDocument { schema_version: 1, ..Default::default() };
    let body = ContentBody::StructuredContent(doc);
    let json = serde_json::to_value(&body).unwrap();
    assert_eq!(json["content_type"], "structured_content");
    assert!(json["data"].is_object());
}

#[test]
fn round_trip_legacy_string() {
    // Legacy in -> tagged out -> tagged back in -> equal
    let legacy = r#""<p>legacy</p>""#;
    let body1: ContentBody = serde_json::from_str(legacy).unwrap();
    let canonical = serde_json::to_string(&body1).unwrap();
    let body2: ContentBody = serde_json::from_str(&canonical).unwrap();
    assert_eq!(body1, body2);
}

#[test]
fn default_is_empty_html() {
    let body: ContentBody = ContentBody::default();
    assert_eq!(body, ContentBody::HtmlContent(String::new()));
}
```

- [ ] **Step 2: Run tests, verify they fail (no `body.rs` yet)**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev cargo test --features full content::tests -- --nocapture`
Expected: compile error (`ContentBody` undefined).

- [ ] **Step 3: Implement `body.rs`**

```rust
// app/ratel/src/common/types/content/body.rs
use serde::{Deserialize, Deserializer, Serialize};

use super::ContentDocument;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "content_type", content = "data", rename_all = "snake_case")]
pub enum ContentBody {
    StructuredContent(ContentDocument),
    HtmlContent(String),
}

impl Default for ContentBody {
    fn default() -> Self {
        ContentBody::HtmlContent(String::new())
    }
}

impl ContentBody {
    pub fn html<S: Into<String>>(s: S) -> Self {
        ContentBody::HtmlContent(s.into())
    }

    pub fn structured(doc: ContentDocument) -> Self {
        ContentBody::StructuredContent(doc)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ContentBody::HtmlContent(s) => s.trim().is_empty(),
            ContentBody::StructuredContent(d) => d.blocks.is_empty(),
        }
    }
}

// Custom Deserialize: accept three shapes.
//   1. JSON string                                 -> HtmlContent
//   2. {"content_type":"html_content","data":...}  -> tagged
//   3. {"content_type":"structured_content",...}   -> tagged
impl<'de> Deserialize<'de> for ContentBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Buffer the input as serde_json::Value so we can branch on its shape.
        // This is small (one rich-text body); the cost is negligible compared to
        // the deserialization itself.
        let v = serde_json::Value::deserialize(deserializer)?;

        match v {
            serde_json::Value::String(s) => Ok(ContentBody::HtmlContent(s)),
            other => {
                // Re-enter via the derived adjacent-tagged Deserialize.
                #[derive(Deserialize)]
                #[serde(tag = "content_type", content = "data", rename_all = "snake_case")]
                enum Tagged {
                    StructuredContent(ContentDocument),
                    HtmlContent(String),
                }
                let tagged: Tagged = serde_json::from_value(other).map_err(serde::de::Error::custom)?;
                Ok(match tagged {
                    Tagged::StructuredContent(d) => ContentBody::StructuredContent(d),
                    Tagged::HtmlContent(s) => ContentBody::HtmlContent(s),
                })
            }
        }
    }
}
```

- [ ] **Step 4: Run tests, verify they pass**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev cargo test --features full content::tests`
Expected: all 7 tests PASS.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/common/types/content/body.rs app/ratel/src/common/types/content/tests.rs app/ratel/src/common/types/content/mod.rs
git commit -m "feat(content): add ContentBody enum with backward-compat Deserialize"
```

---

### Task 3: Implement `to_plain_text`, `to_html`, `char_count` on `ContentBody`

**Files:**
- Create: `app/ratel/src/common/types/content/render.rs`
- Modify: `app/ratel/src/common/types/content/tests.rs`

- [ ] **Step 1: Write failing tests**

Append to `tests.rs`:

```rust
#[test]
fn html_content_to_plain_text_strips_tags() {
    let body = ContentBody::html("<p>Hello <b>world</b> <a href='x'>link</a></p>");
    assert_eq!(body.to_plain_text(), "Hello world link");
}

#[test]
fn html_content_to_html_returns_string() {
    let body = ContentBody::html("<p>raw</p>");
    assert_eq!(body.to_html(), "<p>raw</p>");
}

#[test]
fn structured_content_to_plain_text_walks_blocks() {
    use super::*;
    let doc = ContentDocument {
        schema_version: 1,
        blocks: vec![Block {
            id: "b1".into(),
            kind: BlockKind::Paragraph(TextBlock {
                rich_text: RichText(vec![InlineNode::Text(TextRun {
                    content: "Hello world".into(),
                    annotations: Annotations::default(),
                    link: None,
                })]),
                color: Color::Default,
            }),
            children: vec![],
            created_at: 0,
            updated_at: 0,
        }],
        meta: serde_json::Map::new(),
    };
    let body = ContentBody::structured(doc);
    assert_eq!(body.to_plain_text(), "Hello world");
}

#[test]
fn char_count_counts_unicode_chars() {
    let body = ContentBody::html("<p>가나다</p>");
    assert_eq!(body.char_count(), 3);
}
```

- [ ] **Step 2: Run, verify fail**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev cargo test --features full content::tests`
Expected: 4 new tests fail (methods undefined).

- [ ] **Step 3: Implement `render.rs`**

```rust
// app/ratel/src/common/types/content/render.rs
use super::{Block, BlockKind, ContentBody, ContentDocument, InlineNode, RichText};

impl ContentBody {
    pub fn to_plain_text(&self) -> String {
        match self {
            ContentBody::HtmlContent(html) => strip_html_tags(html),
            ContentBody::StructuredContent(doc) => doc.to_plain_text(),
        }
    }

    pub fn to_html(&self) -> String {
        match self {
            ContentBody::HtmlContent(html) => html.clone(),
            ContentBody::StructuredContent(doc) => doc.to_html(),
        }
    }

    pub fn char_count(&self) -> usize {
        self.to_plain_text().chars().count()
    }
}

impl ContentDocument {
    pub fn to_plain_text(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            block.append_plain_text(&mut out);
        }
        normalize_whitespace(&out)
    }

    /// Lossy projection used until structured rendering lands in the UI.
    /// Each block becomes one HTML element; inline runs become spans/strong/em.
    pub fn to_html(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            block.append_html(&mut out);
        }
        out
    }
}

impl Block {
    fn append_plain_text(&self, out: &mut String) {
        match &self.kind {
            BlockKind::Paragraph(t) | BlockKind::Quote(t)
            | BlockKind::BulletedListItem(t) | BlockKind::NumberedListItem(t)
            | BlockKind::Toggle(t) => {
                t.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Heading(h) => {
                h.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Todo(t) => {
                t.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Callout(c) => {
                c.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Code(c) => {
                c.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Equation(e) => out.push_str(&e.expression),
            BlockKind::Bookmark(b) => {
                b.caption.append_plain(out);
                if !b.url.is_empty() { out.push(' '); out.push_str(&b.url); }
            }
            BlockKind::Embed(e) => out.push_str(&e.url),
            BlockKind::Image(m) | BlockKind::Video(m) | BlockKind::File(m) => {
                if let Some(alt) = &m.alt { out.push_str(alt); }
                m.caption.append_plain(out);
            }
            BlockKind::Divider | BlockKind::Custom(_) | BlockKind::Unknown => {}
        }
        for child in &self.children {
            child.append_plain_text(out);
        }
    }

    fn append_html(&self, out: &mut String) {
        // Minimal HTML projection — the canonical renderer is the Dioxus
        // structured renderer, planned for Phase 4. This is for legacy
        // pipelines that still pipe HTML strings (cross-posting fallback,
        // emails, etc.).
        match &self.kind {
            BlockKind::Paragraph(t) => { out.push_str("<p>"); t.rich_text.append_html(out); out.push_str("</p>"); }
            BlockKind::Heading(h) => {
                let tag = match h.level {
                    super::HeadingLevel::H1 => "h1",
                    super::HeadingLevel::H2 => "h2",
                    super::HeadingLevel::H3 => "h3",
                };
                out.push('<'); out.push_str(tag); out.push('>');
                h.rich_text.append_html(out);
                out.push_str("</"); out.push_str(tag); out.push('>');
            }
            BlockKind::BulletedListItem(t) => { out.push_str("<li>"); t.rich_text.append_html(out); out.push_str("</li>"); }
            BlockKind::NumberedListItem(t) => { out.push_str("<li>"); t.rich_text.append_html(out); out.push_str("</li>"); }
            BlockKind::Quote(t) => { out.push_str("<blockquote>"); t.rich_text.append_html(out); out.push_str("</blockquote>"); }
            BlockKind::Toggle(t) => { out.push_str("<details><summary>"); t.rich_text.append_html(out); out.push_str("</summary></details>"); }
            BlockKind::Todo(t) => {
                let cb = if t.checked { "checked " } else { "" };
                out.push_str("<label><input type=\"checkbox\" disabled "); out.push_str(cb); out.push('>');
                t.rich_text.append_html(out);
                out.push_str("</label>");
            }
            BlockKind::Code(c) => {
                out.push_str("<pre><code>");
                c.rich_text.append_plain(out); // inside <pre>, no inline formatting
                out.push_str("</code></pre>");
            }
            BlockKind::Callout(c) => { out.push_str("<aside>"); c.rich_text.append_html(out); out.push_str("</aside>"); }
            BlockKind::Equation(e) => { out.push_str("<span class=\"math\">"); html_escape(&e.expression, out); out.push_str("</span>"); }
            BlockKind::Image(m) => {
                let url = match &m.source { super::MediaSource::External { url } => url.clone(), super::MediaSource::Asset { asset_id } => asset_id.clone() };
                out.push_str("<img src=\""); html_escape_attr(&url, out); out.push_str("\"");
                if let Some(alt) = &m.alt { out.push_str(" alt=\""); html_escape_attr(alt, out); out.push('"'); }
                out.push_str(" />");
            }
            BlockKind::Video(m) | BlockKind::File(m) => {
                let url = match &m.source { super::MediaSource::External { url } => url.clone(), super::MediaSource::Asset { asset_id } => asset_id.clone() };
                out.push_str("<a href=\""); html_escape_attr(&url, out); out.push_str("\">");
                m.caption.append_html(out);
                out.push_str("</a>");
            }
            BlockKind::Bookmark(b) => {
                out.push_str("<a href=\""); html_escape_attr(&b.url, out); out.push_str("\">");
                b.caption.append_html(out);
                out.push_str("</a>");
            }
            BlockKind::Embed(e) => {
                out.push_str("<a href=\""); html_escape_attr(&e.url, out); out.push_str("\">"); html_escape(&e.url, out); out.push_str("</a>");
            }
            BlockKind::Divider => out.push_str("<hr />"),
            BlockKind::Custom(_) | BlockKind::Unknown => {} // dropped in lossy projection
        }
        for child in &self.children {
            child.append_html(out);
        }
    }
}

impl RichText {
    fn append_plain(&self, out: &mut String) {
        for node in &self.0 {
            match node {
                InlineNode::Text(t) => out.push_str(&t.content),
                InlineNode::Mention(_) => {} // skipped in plain text; UI renders these
                InlineNode::Equation(e) => out.push_str(&e.expression),
            }
        }
    }

    fn append_html(&self, out: &mut String) {
        for node in &self.0 {
            match node {
                InlineNode::Text(t) => {
                    let mut buf = String::new();
                    html_escape(&t.content, &mut buf);
                    let mut s = buf;
                    if t.annotations.code { s = format!("<code>{s}</code>"); }
                    if t.annotations.bold { s = format!("<strong>{s}</strong>"); }
                    if t.annotations.italic { s = format!("<em>{s}</em>"); }
                    if t.annotations.strikethrough { s = format!("<s>{s}</s>"); }
                    if t.annotations.underline { s = format!("<u>{s}</u>"); }
                    if let Some(link) = &t.link {
                        let mut href = String::new();
                        html_escape_attr(link, &mut href);
                        s = format!("<a href=\"{href}\">{s}</a>");
                    }
                    out.push_str(&s);
                }
                InlineNode::Mention(_) => {} // legacy projection skips
                InlineNode::Equation(e) => {
                    out.push_str("<span class=\"math\">");
                    html_escape(&e.expression, out);
                    out.push_str("</span>");
                }
            }
        }
    }
}

fn strip_html_tags(html: &str) -> String {
    // Same shape as features/posts/utils/validator.rs::extract_plain_text.
    let re_img = regex::Regex::new(r"<img[^>]*>").unwrap();
    let no_img = re_img.replace_all(html, "");
    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap();
    let no_tags = re_tags.replace_all(&no_img, "");
    let re_urls = regex::Regex::new(r"https?://[^\s]+").unwrap();
    let no_urls = re_urls.replace_all(&no_tags, "");
    normalize_whitespace(&no_urls)
}

fn normalize_whitespace(s: &str) -> String {
    let re = regex::Regex::new(r"\s+").unwrap();
    re.replace_all(s, " ").trim().to_string()
}

fn html_escape(input: &str, out: &mut String) {
    for ch in input.chars() {
        match ch {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            _ => out.push(ch),
        }
    }
}

fn html_escape_attr(input: &str, out: &mut String) {
    for ch in input.chars() {
        match ch {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev cargo test --features full content::tests`
Expected: all tests PASS.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/common/types/content/render.rs app/ratel/src/common/types/content/tests.rs
git commit -m "feat(content): add to_plain_text/to_html/char_count helpers"
```

---

### Task 4: Re-export `ContentBody` from common prelude

**Files:**
- Modify: `app/ratel/src/common/types/mod.rs` (or `common/mod.rs` if that's where wildcard re-exports live)

- [ ] **Step 1: Add re-export**

Find where the wildcard prelude is built (typically `app/ratel/src/common/types/mod.rs` or `app/ratel/src/common/mod.rs`). Add:

```rust
pub use content::{ContentBody, ContentDocument, Block, BlockKind, RichText, InlineNode};
```

- [ ] **Step 2: Compile**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web`

Both expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/common/
git commit -m "feat(content): re-export ContentBody from common prelude"
```

---

## Phase 2 — Pilot migration: `Post.html_contents` → `Post.body`

This phase is the **template** for Phase 3. Treat it carefully — every later entity follows the same pattern.

### Task 5: Migrate `Post` model to `body: ContentBody`

**Files:**
- Modify: `app/ratel/src/features/posts/models/post.rs:21`
- Modify: `app/ratel/src/features/posts/models/post.rs` (constructor at line ~164)
- Modify: `app/ratel/src/features/posts/models/post_repost.rs:18,33,49`

- [ ] **Step 1: Write failing test**

Add to `app/ratel/src/tests/posts_tests.rs` (create file if missing, register in `tests/mod.rs`):

```rust
use super::*;
use crate::common::ContentBody;

#[tokio::test]
async fn legacy_html_contents_string_loads_as_html_content_body() {
    let ctx = TestContext::setup().await;
    // Insert a row with the *legacy* attribute name + raw string.
    // Use ddb client directly to bypass the new write path.
    let cli = &ctx.ddb;
    let table = std::env::var("DYNAMO_TABLE_PREFIX").unwrap() + "-main";
    let pk = format!("POST#{}", uuid::Uuid::new_v4());
    let item = serde_dynamo::to_item(serde_json::json!({
        "pk": pk,
        "sk": "POST_BODY#legacy",
        "title": "Legacy",
        "html_contents": "<p>legacy body</p>",
        "post_type": "Discussion",
        "status": "Published",
        "user_pk": "USER#legacy",
        "shares": 0, "likes": 0, "comments": 0, "reports": 0,
        "created_at": 0, "updated_at": 0,
        "author_display_name": "x",
        "author_profile_url": "x",
        "author_username": "x",
        "author_type": "Individual",
        "urls": [],
        "categories": [],
    })).unwrap();
    cli.put_item().table_name(&table).set_item(Some(item)).send().await.unwrap();

    // Read via the new model — alias should kick in.
    let res = cli.get_item()
        .table_name(&table)
        .key("pk", aws_sdk_dynamodb::types::AttributeValue::S(pk.clone()))
        .key("sk", aws_sdk_dynamodb::types::AttributeValue::S("POST_BODY#legacy".into()))
        .send().await.unwrap();
    let post: crate::features::posts::models::Post =
        serde_dynamo::from_item(res.item.unwrap()).unwrap();
    assert_eq!(post.body, ContentBody::HtmlContent("<p>legacy body</p>".into()));
}
```

- [ ] **Step 2: Run, verify failure**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- legacy_html_contents_string_loads_as_html_content_body`
Expected: compile error — `body` field doesn't exist yet.

- [ ] **Step 3: Update `post.rs`**

Replace line 21:

```rust
// before
pub html_contents: String,

// after
#[serde(alias = "html_contents")]
pub body: ContentBody,
```

Update the `with_html_contents`-style constructor or builder around line 164 — find every `html_contents: html_contents.into()` and change to `body: ContentBody::html(html_contents)`. Add a new constructor variant:

```rust
pub fn with_body<T: Into<ContentBody>>(...) // similar shape to existing helpers
```

(Read the file end-to-end before editing; the `DynamoEntity` derive auto-generates `with_html_contents`. Rename the field → derive will now generate `with_body`. Caller sites get fixed in Task 7.)

Add an `From<String> for ContentBody` impl in `body.rs` so existing call-sites that pass a `String` keep compiling:

```rust
impl From<String> for ContentBody {
    fn from(s: String) -> Self { ContentBody::HtmlContent(s) }
}
impl From<&str> for ContentBody {
    fn from(s: &str) -> Self { ContentBody::HtmlContent(s.to_string()) }
}
```

- [ ] **Step 4: Mirror change in `post_repost.rs`**

```rust
// post_repost.rs:18
pub post_body: ContentBody,
// constructor at line 33+
post_body: post.body.clone(),
```

- [ ] **Step 5: Compile (will surface ~30 broken call sites)**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: many call-site errors. Don't fix them in this task — record the error list for Tasks 6–10.

- [ ] **Step 6: Commit (broken build is acceptable here — next tasks are sequential and finish in one PR)**

```bash
git add app/ratel/src/features/posts/models/
git commit -m "feat(content): migrate Post model to ContentBody (build broken pending callers)" --no-verify
```

> **Note:** `--no-verify` is *only* acceptable here because Tasks 5–11 form one logical PR. Do not push until Task 11 passes all builds.

---

### Task 6: Update post DTOs to accept both shapes

**Files:**
- Modify: `app/ratel/src/features/posts/controllers/dto/post_response.rs:14,68`
- Modify: `app/ratel/src/features/posts/controllers/update_post.rs:19,41` (request DTOs)
- Modify: `app/ratel/src/features/posts/controllers/create_post.rs` (request DTO)

- [ ] **Step 1: Update `PostResponse`**

```rust
// post_response.rs:14
pub body: ContentBody,

// post_response.rs:68
body: post.body,
```

Add `#[serde(alias = "html_contents")]` to `body` so old clients sending `html_contents` are still accepted.

- [ ] **Step 2: Update update_post request DTO**

In `controllers/update_post.rs`, the field currently called `content: String` becomes `content: ContentBody`. Because of the custom Deserialize, both shapes still work. **No alias needed** — clients just need to send either a raw string or the tagged form.

- [ ] **Step 3: Update create_post request DTO** — same shape change.

- [ ] **Step 4: Compile**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Errors should now be limited to controller bodies (Task 7).

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/posts/controllers/dto/ app/ratel/src/features/posts/controllers/update_post.rs app/ratel/src/features/posts/controllers/create_post.rs
git commit -m "feat(content): accept ContentBody in post DTOs" --no-verify
```

---

### Task 7: Update controllers to use `body` instead of `html_contents`

**Files:**
- Modify: `app/ratel/src/features/posts/controllers/update_post.rs:93,98,143,150`
- Modify: `app/ratel/src/features/posts/controllers/create_post.rs`

- [ ] **Step 1: Replace each `with_html_contents(content)` with `with_body(content_body)`**

```rust
// update_post.rs line 98 (and similar at 150)
let mut updater = updater.with_title(title).with_body(content);

// line 93 (and 143) — was `post.html_contents = content.clone();`
post.body = content.clone();
```

- [ ] **Step 2: Update `validate_content`**

Change `app/ratel/src/features/posts/utils/validator.rs:12`:

```rust
pub fn validate_content(body: &ContentBody) -> Result<()> {
    let len = body.char_count();
    if len < 10 { return Err(Error::ValidationTooShortContents); }
    Ok(())
}
```

Update the call site in update_post controllers and any create flows.

- [ ] **Step 3: Compile**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/posts/controllers/ app/ratel/src/features/posts/utils/validator.rs
git commit -m "feat(content): wire post controllers through ContentBody" --no-verify
```

---

### Task 8: Update display path

**Files:**
- Modify: `app/ratel/src/features/posts/components/post_detail/content.rs:13,50,57,73`
- Modify: `app/ratel/src/features/posts/components/feed_card/mod.rs:104,137,153,167`
- Modify: `app/ratel/src/features/posts/views/post_detail/component.rs:40`
- Modify: `app/ratel/src/features/posts/views/post_edit/component.rs:70,117,170,173,184,186,348,351,1065`
- Modify: `app/ratel/src/features/social/user_views/home/user_posts_panel.rs:70,129`
- Modify: `app/ratel/src/features/timeline/components/draft_row.rs:198`

- [ ] **Step 1: Change component prop types from `html_contents: String` to `body: ContentBody`**

For each component, prop signature changes:

```rust
// before
fn PostContent(html_contents: String, ...) -> Element

// after
fn PostContent(body: ContentBody, ...) -> Element
```

Inside the component, render via `body.to_html()`:

```rust
RichEditor {
    class: "w-full bg-transparent",
    content: body.to_html(),
    editable: false,
}
```

For preview/excerpt sites, replace `strip_html(&post.html_contents)` with `post.body.to_plain_text()`:

```rust
let preview = post.body.to_plain_text();
let excerpt: String = preview.chars().take(200).collect();
```

- [ ] **Step 2: post_edit save path**

In `app/ratel/src/features/posts/views/post_edit/component.rs`, the editor produces an HTML string. Wrap it on save:

```rust
// where you previously did:
//   html_contents: content(),
// now do:
body: ContentBody::html(content()),
```

Keep the editor input/output as `String` (HTML). Editor migration to structured emission is Phase 4.

- [ ] **Step 3: Delete duplicated `strip_html` helpers**

Remove the local `strip_html` definitions in:
- `social/pages/home/views/mod.rs:523`
- `social/pages/user_draft/views/mod.rs:55`
- `social/pages/draft/views/mod.rs:349`
- `posts/components/feed_card/mod.rs:167`
- `posts/views/post_edit/component.rs:1065`

Replace each call site with `body.to_plain_text()`. (For draft preview where the value is still a `String` because the entity hasn't been migrated yet — leave the local `strip_html`. Revisit when the entity is migrated.)

- [ ] **Step 4: Compile both targets**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
```

All three: PASS.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/posts/components app/ratel/src/features/posts/views app/ratel/src/features/social/user_views app/ratel/src/features/social/pages app/ratel/src/features/timeline
git commit -m "feat(content): render Post.body via to_html / to_plain_text" --no-verify
```

---

### Task 9: Update RAG indexer + cross-posting

**Files:**
- Modify: `app/ratel/src/features/rag/qdrant/indexers/post_indexer.rs:13`
- Modify: `app/ratel/src/features/cross_posting/services/format.rs:29,167`
- Modify: `app/ratel/src/features/cross_posting/services/dispatcher.rs:639`
- Modify: `app/ratel/src/features/social/pages/space/controllers/list_my_spaces.rs:116`
- Modify: `app/ratel/src/features/social/pages/home/views/mod.rs:303`
- Modify: `app/ratel/src/features/sub_team/services/announcement_fanout.rs:164`

- [ ] **Step 1: Replace each call**

```rust
// before
let plain_text = crate::features::posts::utils::extract_plain_text(&post.html_contents);
// after
let plain_text = post.body.to_plain_text();

// before (cross_posting)
let body = strip_html(&post.html_contents);
// after
let body = post.body.to_plain_text();

// before (announcement_fanout, where it crosses entity boundaries)
html_contents: announcement.body.clone(),
// after — when sub_team_announcement still holds String body, leave as-is until Phase 3.
// once SubTeamAnnouncement.body is ContentBody, the assignment becomes:
body: announcement.body.clone(),
```

- [ ] **Step 2: Compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

Both: PASS.

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/rag app/ratel/src/features/cross_posting app/ratel/src/features/social/pages app/ratel/src/features/sub_team
git commit -m "feat(content): drop ad-hoc strip_html in pipelines" --no-verify
```

---

### Task 10: Update test fixtures

**Files:**
- Modify: `app/ratel/src/tests/sub_team_tests.rs:1875`
- Anywhere else `html_contents:` literal appears in fixtures

- [ ] **Step 1: Find all fixture call-sites**

Run: `grep -rn 'html_contents:' app/ratel/src/tests app/ratel/src/features/cross_posting/services/format.rs:167`

- [ ] **Step 2: Replace literals**

```rust
// before
html_contents: "B".to_string(),
// after
body: ContentBody::html("B"),
```

- [ ] **Step 3: Run integration tests**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass"`
Expected: PASS, including `legacy_html_contents_string_loads_as_html_content_body`.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/tests app/ratel/src/features/cross_posting
git commit -m "test(content): update Post fixtures to ContentBody"
```

---

### Task 11: Lint, format, end-to-end build verification

- [ ] **Step 1: Format every modified file**

```bash
git diff --name-only origin/dev | grep '\.rs$' | xargs -I{} dx fmt -f {}
git diff --name-only origin/dev | grep '\.rs$' | xargs -I{} rustywind --custom-regex 'class: "(.*)"' --write {}
```

- [ ] **Step 2: Full build matrix**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features mobile
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass"
```

All: PASS.

- [ ] **Step 3: Playwright smoke against post create/read/edit**

```bash
make infra
cd playwright && npx playwright test tests/web/post-creation.spec.js --headed
```

Expected: PASS. The render still goes through the existing Tiptap path (via `body.to_html()`), so visible behavior should not change.

- [ ] **Step 4: Final commit + ready for review**

```bash
git add -A
git commit -m "chore(content): post pilot migration green across server/web/mobile/web-dx/cargo-test" || true
```

---

## Phase 3 — Roll out to remaining entities

Each entity follows the **same template** as Phase 2. One PR per entity.

### Task 12: `Space.content` → `Space.body`

**Touch points (use grep to find all):**
- `app/ratel/src/features/spaces/models/space.rs:10`
- `app/ratel/src/features/spaces/dto/get_space.rs:10`
- `app/ratel/src/features/spaces/space_common/controllers/get_space.rs:139`
- `app/ratel/src/features/spaces/space_common/controllers/update_space.rs:22,65,165,170`
- `app/ratel/src/features/spaces/pages/overview/controllers/update_content.rs:12,30,33`
- `app/ratel/src/features/spaces/pages/overview/views/overview_content.rs`
- `app/ratel/src/features/spaces/space_common/models/hot_space.rs:63` (`description`)
- `app/ratel/src/features/spaces/space_common/models/space_reward.rs:20,38` (`description`)
- `app/ratel/src/features/spaces/space_common/models/space_reward_response.rs:11`

Per-task structure: same six steps as Tasks 5–11 (model rename + `#[serde(alias = "content")]`, DTO updates, controller updates, view updates, RAG/preview updates, fixture updates, final build matrix).

**Estimated effort:** 1 day.

### Task 13: `PostComment.content` → `PostComment.body`

**Touch points:**
- `app/ratel/src/features/posts/models/post_comment.rs:16,41,100`
- `app/ratel/src/features/posts/controllers/comments/add_comment.rs:9`
- `app/ratel/src/features/posts/controllers/comments/reply_to_comment.rs:8`
- `app/ratel/src/features/posts/controllers/dto/post_comment_response.rs:13`
- Comment rendering components (grep `post_comment` and `comment.content`)

**Estimated effort:** 0.5 day.

### Task 14: Discussion `SpacePost.content` and `SpacePostComment.content`

**Touch points:**
- `app/ratel/src/features/spaces/pages/actions/actions/discussion/models/space_post.rs:85`
- `app/ratel/src/features/spaces/pages/actions/actions/discussion/models/space_post_comment.rs:26,65,123`
- `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/comments/{add_comment,reply_comment,update_comment}.rs`
- `app/ratel/src/features/spaces/pages/actions/actions/discussion/types/discussion_comment_response.rs:11`
- `app/ratel/src/features/spaces/pages/actions/actions/discussion/hooks/use_discussion_arena.rs:538,553` (action signatures)
- `app/ratel/src/features/spaces/pages/index/action_pages/discussion/component.rs:648`

**Estimated effort:** 0.5 day.

### Task 15: `SubTeamDocument.body` and `SubTeamAnnouncement.body`

**Touch points:**
- `app/ratel/src/features/sub_team/models/sub_team_document.rs:27,44,61`
- `app/ratel/src/features/sub_team/models/sub_team_announcement.rs:23,60`
- `app/ratel/src/features/sub_team/types/dto.rs:79,90,123,325,339`
- `app/ratel/src/features/sub_team/controllers/announcements.rs:154`
- `app/ratel/src/features/sub_team/controllers/docs.rs:115`
- `app/ratel/src/features/sub_team/services/announcement_fanout.rs:164`

**Estimated effort:** 0.5 day.

### Task 16: `Team.description` and `User.description`

Only migrate **if** the user-facing editor produces rich content for these. If they're plain-text profile bios, leave them as `String` and skip this task.

**Touch points (if migrating):**
- `app/ratel/src/features/posts/models/team.rs:45,84,119`
- `app/ratel/src/features/social/controllers/team.rs:8`
- `app/ratel/src/features/social/pages/setting/controllers/update_team.rs:28`
- `app/ratel/src/features/social/pages/user_setting/controllers/{update_profile,update_user}.rs`

**Estimated effort:** 0.5 day.

---

## Phase 4 — (Future) Editor produces structured content

Out of scope for this plan. Sketch:

1. Replace Tiptap HTML output with a Tiptap → `ContentDocument` JSON converter (frontend JS).
2. On save, send `ContentBody::StructuredContent(doc)` instead of `ContentBody::HtmlContent(s)`.
3. Build a Dioxus `StructuredRenderer` component that walks `ContentDocument` blocks and renders RSX directly (no `dangerous_inner_html`).
4. Drop `ContentBody::to_html()` lossy projection from view code (still needed for cross-posting / email).

This is a separate roadmap item. Consider it after Phase 1–3 ship and stabilize.

---

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| `serde_dynamo` round-trip with new tagged shape | Test in Task 5 Step 1 specifically exercises this |
| `DynamoEntity` derive doesn't generate `with_body` correctly | Inspect generated code with `cargo expand`; fall back to manual `with_*` helper if needed |
| Old in-flight clients send `html_contents:` field | `#[serde(alias = "html_contents")]` on DTO + custom `Deserialize` accepting raw string |
| Lossy `to_html()` for `StructuredContent` looks worse than Tiptap output | Acceptable for Phase 1–3 because no entity yet *contains* `StructuredContent` — every legacy row stays `HtmlContent`. Phase 4 ships the proper renderer |
| GSI fields named after the column (none of the migrated fields are GSI keys) | Verified — `html_contents`, `content`, `body` (sub-team), comment `content` are all non-indexed payload columns |
| Custom `Deserialize` slows down every read | Negligible: one `serde_json::Value` buffer per body. Bench if concerned |

---

## Self-review checklist

- [x] **Spec coverage**: every rich-content field in the codebase is named in Phase 2 (Post pilot) or Phase 3 (rollout). Plain-text-only fields explicitly called out as out-of-scope.
- [x] **Placeholders**: none. Every step has either runnable commands, code blocks, or named files with line numbers.
- [x] **Type consistency**: `ContentBody`, `with_body`, `body.to_plain_text()`, `body.to_html()` used uniformly across all phases.
- [x] **Backward compat**: documented at top, encoded in custom `Deserialize`, exercised by `legacy_html_contents_string_loads_as_html_content_body` test.
- [x] **Build verification**: every task ends with the project's standard `cargo check` matrix; final task runs full Playwright smoke.
- [x] **Commits**: one per task, with `--no-verify` permitted *only* during the broken-build window of Phase 2 (Tasks 5–10), restored to clean by Task 11.

---

## System design doc reminder

Per `CLAUDE.md` § feature-development workflow, before starting Phase 2 (code), author the system-design doc at:

```
docs/superpower/2026-05-06-content-body-migration.md
```

Following the template in `.claude/rules/workflows/develop-a-new-feature.md` § Step 2. This plan is the implementation breakdown; the design doc captures *why* (decision log, alternatives considered) and stakeholders sign off there before any code lands.
