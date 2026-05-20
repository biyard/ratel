use crate::features::spaces::pages::apps::apps::analyzes::{
    NetworkEdge, NetworkNode, TermScore, TopicRow,
};
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Identifies which action surface an aggregate came from. Drives the
/// per-source color tokens in CSS and the tab grouping in the picker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum ActionSource {
    Poll,
    Quiz,
    Discussion,
    Follow,
}

impl ActionSource {
    pub const VARIANTS: [ActionSource; 4] = [
        ActionSource::Poll,
        ActionSource::Quiz,
        ActionSource::Discussion,
        ActionSource::Follow,
    ];

    pub fn as_token(&self) -> &'static str {
        match self {
            ActionSource::Poll => "poll",
            ActionSource::Quiz => "quiz",
            ActionSource::Discussion => "discussion",
            ActionSource::Follow => "follow",
        }
    }
}

/// LDA / TF-IDF / text-network result payload for one discussion under
/// one analyze report. Sourced from `SpaceAnalyzeDiscussionResult` and
/// embedded in `AnalyzeItem` + `ReportBlock::Chart` so the picker hands
/// the canvas a ready-to-render shape (mirrors the eager-data pattern
/// poll/quiz/follow already use via `options`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct DiscussionData {
    #[serde(default)]
    pub topics: Vec<TopicRow>,
    #[serde(default)]
    pub tfidf_terms: Vec<TermScore>,
    #[serde(default)]
    pub network_nodes: Vec<NetworkNode>,
    #[serde(default)]
    pub network_edges: Vec<NetworkEdge>,
}

/// One pickable aggregate inside an analyze, grouped by source.
/// Carries the actual tally data so the picker can hand a ready-to-
/// render payload straight into the Chart block on insertion — the
/// detail editor doesn't need to round-trip back to the server when
/// the user picks an item.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct AnalyzeItem {
    pub id: String,
    pub title: String,
    /// Free-text meta line ("5 옵션 · 매칭 응답 1,248" etc.) shown
    /// under the title in the picker list.
    pub meta: String,
    /// Per-option tally for poll/quiz/follow items. Empty for
    /// discussion items (their data lives in `discussion_data`) AND
    /// for subjective (short-answer) poll/quiz items where the data
    /// lives in `text_answers` instead.
    #[serde(default)]
    pub options: Vec<ChartOption>,
    /// Denominator for percentage math. Stays 0 when there's no
    /// useful base count (e.g. follow items where the picker shows
    /// counts directly).
    #[serde(default)]
    pub respondent_count: u32,
    /// LDA / TF-IDF / network payload for discussion items. `None` for
    /// poll/quiz/follow.
    #[serde(default)]
    pub discussion_data: Option<DiscussionData>,
    /// Free-text answers for subjective poll/quiz questions
    /// (short / long answer style — no multi-option tally). When
    /// non-empty the chart is rendered as a numbered list of answers
    /// (`ChartType::TextList`), and the type-swap UI is hidden.
    #[serde(default)]
    pub text_answers: Vec<String>,
}

/// One bar / pie slice / table row inside a chart payload.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ChartOption {
    pub label: String,
    pub count: u32,
}

/// One element of the analyze's cross-filter set — rendered as a small
/// source-tinted chip under the analyze label in the picker dropdown.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CrossFilterChip {
    pub source: ActionSource,
    pub label: String,
}

/// A saved analyze that the report can pull aggregates from.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct Analyze {
    pub id: String,
    pub name: String,
    pub respondents: u32,
    /// Cross-filters that defined the analyze's audience (action +
    /// option / keyword intersections).
    pub filters: Vec<CrossFilterChip>,
    pub poll: Vec<AnalyzeItem>,
    pub quiz: Vec<AnalyzeItem>,
    pub discussion: Vec<AnalyzeItem>,
    pub follow: Vec<AnalyzeItem>,
}

impl Analyze {
    pub fn items_for(&self, src: ActionSource) -> &[AnalyzeItem] {
        match src {
            ActionSource::Poll => &self.poll,
            ActionSource::Quiz => &self.quiz,
            ActionSource::Discussion => &self.discussion,
            ActionSource::Follow => &self.follow,
        }
    }

    pub fn total_items(&self) -> usize {
        self.poll.len() + self.quiz.len() + self.discussion.len() + self.follow.len()
    }
}

/// One outline entry derived from the report body — H1/H2/H3 heading
/// or an inserted chart block. Drives the right-rail outline list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct OutlineEntry {
    pub id: String,
    pub kind: OutlineKind,
    pub label: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum OutlineKind {
    H1,
    H2,
    H3,
    Chart,
}

/// Visual rendering for a Chart block. The picker stamps each chart
/// with a default type when it inserts; the outline swap then lets the
/// author pick a different one (bar → pie etc.). Source (poll/quiz/...)
/// controls *which* data the chart pulls from — type controls how it's
/// drawn. The Bar/Pie/Table trio covers poll/quiz/follow aggregates;
/// the LDA/TfIdf/Network trio is reserved for discussion text analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum ChartType {
    /// Bar chart — response distribution for poll/quiz/follow.
    Bar,
    /// Pie chart — proportion breakdown for poll/quiz/follow.
    Pie,
    /// Raw aggregate table — option × count grid.
    Table,
    /// LDA topic block — discussion-only. Topic weights + top keywords.
    Lda,
    /// TF-IDF ranked term table — discussion-only.
    TfIdf,
    /// Text-network graph — discussion-only. Nodes = terms, edges =
    /// co-occurrence frequency.
    Network,
    /// Free-text answer list — short/long-answer poll/quiz questions.
    /// The only valid type when the item has no options; the figure
    /// renders as a simple numbered table of respondent answers and
    /// the chart-type swap affordance is hidden in this mode.
    TextList,
}

impl ChartType {
    pub const VARIANTS: [ChartType; 7] = [
        ChartType::Bar,
        ChartType::Pie,
        ChartType::Table,
        ChartType::Lda,
        ChartType::TfIdf,
        ChartType::Network,
        ChartType::TextList,
    ];

    pub fn as_token(&self) -> &'static str {
        match self {
            ChartType::Bar => "bar",
            ChartType::Pie => "pie",
            ChartType::Table => "table",
            ChartType::Lda => "lda",
            ChartType::TfIdf => "tfidf",
            ChartType::Network => "network",
            ChartType::TextList => "textlist",
        }
    }

    /// True when this chart type doesn't allow any alternate renderings
    /// — i.e. the chart-type swap UI should be suppressed for it.
    /// Currently only `TextList` qualifies; subjective answers don't
    /// have a "bar chart" equivalent to flip to.
    pub fn is_single_mode(&self) -> bool {
        matches!(self, ChartType::TextList)
    }

    /// Chart types available for a given data source. Mirrors the
    /// mockup's `chartTypeOptions` map in `assets/design/reports/
    /// reports-edit.html` — discussion data goes through LDA/TF-IDF/
    /// Network text-analysis renderings, while poll/quiz/follow share
    /// the Bar/Pie/Table trio.
    pub fn options_for(source: ActionSource) -> &'static [ChartType] {
        match source {
            ActionSource::Poll | ActionSource::Quiz | ActionSource::Follow => {
                &[ChartType::Bar, ChartType::Pie, ChartType::Table]
            }
            ActionSource::Discussion => &[ChartType::Lda, ChartType::TfIdf, ChartType::Network],
        }
    }

    /// Default chart type for a freshly inserted chart, picked by the
    /// data source. The first entry of `options_for(source)` doubles as
    /// the default so a swap into a fresh source always starts somewhere
    /// valid.
    pub fn default_for(source: ActionSource) -> ChartType {
        Self::options_for(source)[0]
    }

    /// Default + valid-options based on the actual item shape, not just
    /// the source surface. A poll item with empty `options` + non-empty
    /// `text_answers` is a subjective question and the only sensible
    /// rendering is `TextList`; for everything else we fall back to the
    /// source-based default.
    pub fn default_for_item(item: &AnalyzeItem, source: ActionSource) -> ChartType {
        if Self::is_text_list_item(item) {
            ChartType::TextList
        } else {
            Self::default_for(source)
        }
    }

    /// Valid chart types for THIS specific item — narrows
    /// `options_for(source)` down to `[TextList]` when the item is
    /// subjective. Returns owned `Vec` because the textlist case can't
    /// be expressed as a `'static` slice alongside the others without
    /// duplicating constants.
    pub fn options_for_item(item: &AnalyzeItem, source: ActionSource) -> Vec<ChartType> {
        if Self::is_text_list_item(item) {
            vec![ChartType::TextList]
        } else {
            Self::options_for(source).to_vec()
        }
    }

    fn is_text_list_item(item: &AnalyzeItem) -> bool {
        item.options.is_empty() && !item.text_answers.is_empty()
    }

    /// True when this chart type is valid for the given source. Used by
    /// the outline-swap UI to filter the option list.
    pub fn is_valid_for(&self, source: ActionSource) -> bool {
        Self::options_for(source).contains(self)
    }
}

/// One block in the report body. Mirrors the mockup's Notion-style
/// block editor: each block renders independently with its own handle
/// and contenteditable affordance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum ReportBlock {
    H1 {
        id: String,
        text: String,
    },
    H2 {
        id: String,
        text: String,
    },
    H3 {
        id: String,
        text: String,
    },
    /// Paragraph — `html` may contain inline `<strong>`/`<em>` etc.
    Text {
        id: String,
        html: String,
    },
    /// Inserted chart block. `source` is the analyze source that
    /// supplied the data (drives badge color); `chart_type` controls
    /// the visual rendering (bar/pie/topics/table). `options` /
    /// `respondent_count` carry the actual tally so Bar/Pie/Table
    /// render real data instead of mock numbers. Empty `options`
    /// (e.g. discussion items, or an inserted-but-stale block) makes
    /// the canvas render an empty-state message.
    Chart {
        id: String,
        source: ActionSource,
        chart_type: ChartType,
        analyze_name: String,
        item_title: String,
        meta: String,
        #[serde(default)]
        options: Vec<ChartOption>,
        #[serde(default)]
        respondent_count: u32,
        /// LDA / TF-IDF / network payload — populated only for
        /// `source = Discussion`. `None` for poll/quiz/follow charts
        /// (their data lives in `options`).
        #[serde(default)]
        discussion_data: Option<DiscussionData>,
    },
}

impl ReportBlock {
    pub fn id(&self) -> &str {
        match self {
            ReportBlock::H1 { id, .. }
            | ReportBlock::H2 { id, .. }
            | ReportBlock::H3 { id, .. }
            | ReportBlock::Text { id, .. }
            | ReportBlock::Chart { id, .. } => id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportDetail {
    pub id: String,
    pub eyebrow: String,
    pub title: String,
    pub subtitle: String,
    /// Body HTML — produced by the shared `Editor` component's
    /// `on_content_change` callback and persisted via
    /// `update_report.html_contents`. Replaces the old `blocks` model.
    /// Inline chart figures live inside this string as
    /// `<figure contenteditable="false" data-chart-id="…">…</figure>`.
    #[serde(default)]
    pub html_contents: String,
    /// Legacy block list — kept on the in-memory model only because the
    /// `GetReportResponse` DTO still returns it. The detail page ignores
    /// it; outline + chart manipulation read from `html_contents`. Will
    /// be dropped once the DTO is cleaned up.
    pub blocks: Vec<ReportBlock>,
    pub author: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub analyzes: Vec<Analyze>,
}

impl ReportDetail {
    /// Auto-derive the outline from the current block list.
    pub fn outline_from_blocks(&self) -> Vec<OutlineEntry> {
        self.blocks
            .iter()
            .filter_map(|b| match b {
                ReportBlock::H1 { id, text } => Some(OutlineEntry {
                    id: id.clone(),
                    kind: OutlineKind::H1,
                    label: text.clone(),
                }),
                ReportBlock::H2 { id, text } => Some(OutlineEntry {
                    id: id.clone(),
                    kind: OutlineKind::H2,
                    label: text.clone(),
                }),
                ReportBlock::H3 { id, text } => Some(OutlineEntry {
                    id: id.clone(),
                    kind: OutlineKind::H3,
                    label: text.clone(),
                }),
                ReportBlock::Chart { id, item_title, .. } => Some(OutlineEntry {
                    id: id.clone(),
                    kind: OutlineKind::Chart,
                    label: item_title.clone(),
                }),
                ReportBlock::Text { .. } => None,
            })
            .collect()
    }
}
