#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

use serde::{Deserialize, Serialize};

pub type BlockId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ContentDocument {
    pub schema_version: u32,
    pub blocks: Vec<Block>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub meta: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TextBlock {
    pub rich_text: RichText,
    #[serde(default, skip_serializing_if = "Color::is_default")]
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct HeadingBlock {
    pub level: HeadingLevel,
    pub rich_text: RichText,
    #[serde(default)]
    pub toggleable: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TodoBlock {
    pub rich_text: RichText,
    pub checked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CodeBlock {
    pub rich_text: RichText,
    pub language: String,
    #[serde(default)]
    pub caption: RichText,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CalloutBlock {
    pub rich_text: RichText,
    pub icon: Option<Icon>,
    #[serde(default)]
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct EquationBlock {
    pub expression: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct MediaBlock {
    pub source: MediaSource,
    #[serde(default)]
    pub caption: RichText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MediaSource {
    Asset { asset_id: String },
    External { url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct BookmarkBlock {
    pub url: String,
    #[serde(default)]
    pub caption: RichText,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct EmbedBlock {
    pub url: String,
    pub provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CustomBlock {
    pub namespace: String,
    pub kind: String,
    pub schema_version: u32,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(transparent)]
pub struct RichText(pub Vec<InlineNode>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum InlineNode {
    Text(TextRun),
    Mention(Mention),
    Equation(InlineEquation),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TextRun {
    pub content: String,
    #[serde(default, skip_serializing_if = "Annotations::is_default")]
    pub annotations: Annotations,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct Annotations {
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub strikethrough: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub code: bool,
    #[serde(default)]
    pub color: Color,
}

impl Annotations {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct InlineEquation {
    pub expression: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum Color {
    #[default]
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    GreenBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
}
impl Color {
    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Icon {
    Emoji(String),
    Asset { asset_id: String },
    External { url: String },
}
