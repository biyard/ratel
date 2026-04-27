//! Mock report types and seed data for the analyzes arena LIST view.
//!
//! Mirrors the `analyses` array in
//! `app/ratel/assets/design/analyze-list-arena.html`. Once Phase 2/3
//! lands, the mock module will be swapped for real server data without
//! changing the consumer-side shape (the list view destructures the
//! same fields).

use std::fmt;

/// Status badge shown on the top-left of a saved report card.
///
/// `Done` renders the green check + "분석 완료"; `Running` renders the
/// pulsing dot + "분석 중". Maps directly onto the HTML mockup's
/// `data-status` attribute, so the existing CSS picks up the badge
/// styling for free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalyzeReportStatus {
    Running,
    Done,
}

impl AnalyzeReportStatus {
    /// Lowercase string used in the `data-status` attribute (matches the
    /// HTML mockup verbatim).
    pub fn as_str(&self) -> &'static str {
        match self {
            AnalyzeReportStatus::Running => "running",
            AnalyzeReportStatus::Done => "done",
        }
    }
}

/// Source of a filter chip — i.e. which action type produced it.
///
/// Determines the chip palette via `data-source` (cyan for poll, purple
/// for quiz, blue for discussion, orange for follow). The lowercase
/// string from `Display` is what the CSS selector matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalyzeFilterSource {
    Poll,
    Quiz,
    Discussion,
    Follow,
}

impl AnalyzeFilterSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnalyzeFilterSource::Poll => "poll",
            AnalyzeFilterSource::Quiz => "quiz",
            AnalyzeFilterSource::Discussion => "discussion",
            AnalyzeFilterSource::Follow => "follow",
        }
    }
}

impl fmt::Display for AnalyzeFilterSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One filter chip on a saved report card.
///
/// The mockup distinguishes a coloured `source` capsule (POLL / QUIZ /
/// DISCUSSION / FOLLOW) from the longer `label` body that names the
/// specific question or option. `item_id` / `question_id` / `option_id`
/// are kept on the struct so that Phase-2 (CREATE) and Phase-3 (DETAIL)
/// can round-trip the same values without redefining the type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeReportFilter {
    pub source: AnalyzeFilterSource,
    pub source_label: String,
    pub label: String,
    pub item_id: String,
    pub question_id: String,
    pub option_id: String,
    pub option_text: String,
    pub question_title: String,
    pub correct: bool,
}

/// One saved analysis as displayed in the list carousel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeReport {
    pub id: String,
    pub name: String,
    pub status: AnalyzeReportStatus,
    pub created_at: String,
    pub created_at_time: String,
    pub filters: Vec<AnalyzeReportFilter>,
}

/// Pre-seeded reports matching the demo `analyses` array in the HTML
/// mockup. Real data starts empty; this is purely so the visual
/// styling (status badge, chip palette) is exercised in dev.
pub fn mock_reports() -> Vec<AnalyzeReport> {
    vec![
        AnalyzeReport {
            id: "1".to_string(),
            name: "헌법 개정 의견 × 무고죄 인식 교차분석".to_string(),
            status: AnalyzeReportStatus::Done,
            created_at: "2026.04.20".to_string(),
            created_at_time: "14:32".to_string(),
            filters: vec![
                AnalyzeReportFilter {
                    source: AnalyzeFilterSource::Poll,
                    source_label: "Poll".to_string(),
                    label: "헌법 개정 · 기본권 강화".to_string(),
                    item_id: "poll-001".to_string(),
                    question_id: "q1".to_string(),
                    option_id: "opt1".to_string(),
                    option_text: "기본권 강화".to_string(),
                    question_title: "헌법 개정의 핵심 가치는?".to_string(),
                    correct: false,
                },
                AnalyzeReportFilter {
                    source: AnalyzeFilterSource::Discussion,
                    source_label: "Discussion".to_string(),
                    label: "비동의 강간죄 · 신고".to_string(),
                    item_id: "disc-001".to_string(),
                    question_id: String::new(),
                    option_id: String::new(),
                    option_text: "신고".to_string(),
                    question_title: "비동의 강간죄 토론".to_string(),
                    correct: false,
                },
            ],
        },
        AnalyzeReport {
            id: "2".to_string(),
            name: "헌법 상식 정답률 × 응답 패턴".to_string(),
            status: AnalyzeReportStatus::Running,
            created_at: "2026.04.27".to_string(),
            created_at_time: "11:08".to_string(),
            filters: vec![
                AnalyzeReportFilter {
                    source: AnalyzeFilterSource::Quiz,
                    source_label: "Quiz".to_string(),
                    label: "헌법 기본 상식 퀴즈".to_string(),
                    item_id: "quiz-001".to_string(),
                    question_id: "qq1".to_string(),
                    option_id: "qopt1".to_string(),
                    option_text: "정답".to_string(),
                    question_title: "헌법 기본 상식 퀴즈".to_string(),
                    correct: true,
                },
                AnalyzeReportFilter {
                    source: AnalyzeFilterSource::Poll,
                    source_label: "Poll".to_string(),
                    label: "개헌 가치 · 자유".to_string(),
                    item_id: "poll-002".to_string(),
                    question_id: "q2".to_string(),
                    option_id: "opt2".to_string(),
                    option_text: "자유".to_string(),
                    question_title: "개헌의 핵심 가치는?".to_string(),
                    correct: false,
                },
            ],
        },
    ]
}
