//! Mock data + helpers for the Phase-2 CREATE wizard.
//!
//! Mirrors the `ITEMS`, `ITEM_QUESTIONS`, `COMMENT_TEMPLATES`,
//! `DISC_ITEM_TITLES` constants and the `generateRecordsForFilter`
//! helper from `app/ratel/assets/design/analyze-create-arena.html`.
//! Phase 2 is a pure visual port — no controllers, no DynamoDB. Once
//! Phase 5+ wires this to the real action data, only the mock helper
//! functions need to swap; the consumer-side shapes (chip records,
//! preview rows) stay stable.

use super::report::AnalyzeFilterSource;

/// Top-level wizard mode — driven by `data-mode` on `.split` /
/// `.analyze-builder` in the HTML mockup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateMode {
    /// Cross-filter selection step (1 / 2). Shows the cross-filter
    /// card + cf-sunji picker.
    Create,
    /// Confirm step (2 / 2). Shows the name input, chip summary,
    /// stats, and per-source merged record tables.
    Preview,
}

/// State machine inside the cross-filter card. Mirrors `[data-add-state]`
/// in the HTML mockup — `idle | picking-action | picking-item`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddState {
    /// Just the chips strip + "+ 필터 추가" CTA.
    Idle,
    /// 4-tile action picker (Poll / Quiz / Discussion / Follow).
    PickingAction,
    /// Single-select radio list of items for the picked action type.
    /// Selecting a radio auto-opens the cf-sunji card below.
    PickingItem,
}

impl AddState {
    pub fn as_str(&self) -> &'static str {
        match self {
            AddState::Idle => "idle",
            AddState::PickingAction => "picking-action",
            AddState::PickingItem => "picking-item",
        }
    }
}

impl CreateMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            CreateMode::Create => "create",
            CreateMode::Preview => "preview",
        }
    }
}

/// One selectable item in the picking-item radio list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeActionItem {
    pub id: String,
    pub source: AnalyzeFilterSource,
    pub title: String,
    pub meta: String,
}

/// One selectable option inside an item's question. Becomes one chip
/// in the cross-filter when checked + 확인 pressed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeOption {
    pub id: String,
    pub label: String,
    pub correct: bool,
}

/// One question inside an item — title heading + nested option
/// checkboxes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeQuestion {
    pub id: String,
    pub title: String,
    pub options: Vec<AnalyzeOption>,
}

impl AnalyzeFilterSource {
    /// Display label used both in the action tile (`Poll`, `Quiz`, …)
    /// and in chip badges. Capitalised.
    pub fn type_label(&self) -> &'static str {
        match self {
            AnalyzeFilterSource::Poll => "Poll",
            AnalyzeFilterSource::Quiz => "Quiz",
            AnalyzeFilterSource::Discussion => "Discussion",
            AnalyzeFilterSource::Follow => "Follow",
        }
    }

    /// Uppercase chip badge ("POLL" / "QUIZ" / "DISCUSSION" / "FOLLOW").
    pub fn badge(&self) -> &'static str {
        match self {
            AnalyzeFilterSource::Poll => "POLL",
            AnalyzeFilterSource::Quiz => "QUIZ",
            AnalyzeFilterSource::Discussion => "DISCUSSION",
            AnalyzeFilterSource::Follow => "FOLLOW",
        }
    }
}

/// Mock items per action type — must match the right-sidebar list in
/// the DETAIL view so users see the same labels everywhere. Numbers
/// (`5 문항 · 128명 응답`) are presentation-only mock counts.
pub fn mock_action_items(source: AnalyzeFilterSource) -> Vec<AnalyzeActionItem> {
    match source {
        AnalyzeFilterSource::Poll => vec![
            AnalyzeActionItem {
                id: "poll-1".to_string(),
                source,
                title: "귀하는 현재 대한민국 헌법을 개정하는 것이 필요하다고 생각하십니까?"
                    .to_string(),
                meta: "5 문항 · 128명 응답".to_string(),
            },
            AnalyzeActionItem {
                id: "poll-2".to_string(),
                source,
                title: "공직 선거 연령 하향에 찬성하십니까?".to_string(),
                meta: "3 문항 · 94명 응답".to_string(),
            },
            AnalyzeActionItem {
                id: "poll-3".to_string(),
                source,
                title: "사법부 독립성 강화 방안 우선순위".to_string(),
                meta: "7 문항 · 71명 응답".to_string(),
            },
        ],
        AnalyzeFilterSource::Quiz => vec![
            AnalyzeActionItem {
                id: "quiz-1".to_string(),
                source,
                title: "헌법 기본 상식 퀴즈".to_string(),
                meta: "10 문항 · 86명 응시".to_string(),
            },
            AnalyzeActionItem {
                id: "quiz-2".to_string(),
                source,
                title: "법률 용어 이해도 테스트".to_string(),
                meta: "8 문항 · 47명 응시".to_string(),
            },
        ],
        AnalyzeFilterSource::Discussion => vec![
            AnalyzeActionItem {
                id: "disc-1".to_string(),
                source,
                title: "비동의 강간죄 도입에 대해서 어떻게 생각하십니까?".to_string(),
                meta: "142 댓글 · 38명 참여".to_string(),
            },
            AnalyzeActionItem {
                id: "disc-2".to_string(),
                source,
                title: "무고죄 형량 강화에 대한 시민 의견".to_string(),
                meta: "87 댓글 · 24명 참여".to_string(),
            },
        ],
        AnalyzeFilterSource::Follow => vec![AnalyzeActionItem {
            id: "follow-1".to_string(),
            source,
            title: "법률 전문가 팔로우 캠페인".to_string(),
            meta: "12 타겟 · 42명 참여".to_string(),
        }],
    }
}

/// Mock count summary used in the action picker tiles ("3개", "1개", …).
/// Pulls from `mock_action_items` so the two stay consistent.
pub fn mock_action_count(source: AnalyzeFilterSource) -> usize {
    mock_action_items(source).len()
}

fn opt(id: &str, label: &str) -> AnalyzeOption {
    AnalyzeOption {
        id: id.to_string(),
        label: label.to_string(),
        correct: false,
    }
}

fn opt_correct(id: &str, label: &str) -> AnalyzeOption {
    AnalyzeOption {
        id: id.to_string(),
        label: label.to_string(),
        correct: true,
    }
}

/// Mock questions per item id. Mirrors the `ITEM_QUESTIONS` map in the
/// HTML mockup. For DISCUSSION items the predefined `keywords` question
/// is dropped on render — the keyword input replaces it. LDA topics
/// (`disc-1`) still render as checkboxes.
pub fn mock_questions_for(item_id: &str) -> Vec<AnalyzeQuestion> {
    match item_id {
        "poll-1" => vec![
            AnalyzeQuestion {
                id: "q1".to_string(),
                title: "Q1. 가장 시급하게 추진해야 할 헌법 개정 분야는?".to_string(),
                options: vec![
                    opt("q1-o1", "기본권 강화"),
                    opt("q1-o2", "권력구조 개편"),
                    opt("q1-o3", "지방분권 강화"),
                    opt("q1-o4", "사법 독립"),
                ],
            },
            AnalyzeQuestion {
                id: "q2".to_string(),
                title: "Q2. 개헌 시 가장 중요하게 다뤄져야 할 가치는?".to_string(),
                options: vec![
                    opt("q2-o1", "자유"),
                    opt("q2-o2", "평등"),
                    opt("q2-o3", "연대"),
                ],
            },
        ],
        "poll-2" => vec![AnalyzeQuestion {
            id: "q1".to_string(),
            title: "Q1. 공직 선거 연령 하향에 찬성하십니까?".to_string(),
            options: vec![
                opt("q1-yes", "찬성"),
                opt("q1-no", "반대"),
                opt("q1-na", "모름"),
            ],
        }],
        "poll-3" => vec![AnalyzeQuestion {
            id: "q1".to_string(),
            title: "Q1. 사법부 독립성 강화 우선순위 1위는?".to_string(),
            options: vec![
                opt("q1-r1", "인사권 분리"),
                opt("q1-r2", "예산 독립"),
                opt("q1-r3", "평가 위원회"),
            ],
        }],
        "quiz-1" => vec![
            AnalyzeQuestion {
                id: "q1".to_string(),
                title: "Q1. 헌법 제1조의 내용은?".to_string(),
                options: vec![
                    opt_correct("q1-o1", "대한민국은 민주공화국이다"),
                    opt("q1-o2", "대한민국의 영토는 한반도와 부속도서이다"),
                    opt("q1-o3", "모든 국민은 인간으로서의 존엄과 가치를 가진다"),
                ],
            },
            AnalyzeQuestion {
                id: "q2".to_string(),
                title: "Q2. 대한민국의 입법권은 어디에 있는가?".to_string(),
                options: vec![
                    opt_correct("q2-o1", "국회"),
                    opt("q2-o2", "정부"),
                    opt("q2-o3", "대통령"),
                    opt("q2-o4", "법원"),
                ],
            },
            AnalyzeQuestion {
                id: "q3".to_string(),
                title: "Q3. 헌법재판소의 재판관 수는?".to_string(),
                options: vec![
                    opt("q3-o1", "7명"),
                    opt_correct("q3-o2", "9명"),
                    opt("q3-o3", "11명"),
                    opt("q3-o4", "13명"),
                ],
            },
        ],
        "quiz-2" => vec![AnalyzeQuestion {
            id: "q1".to_string(),
            title: "Q1. \"기소\"의 정의는?".to_string(),
            options: vec![
                opt_correct("q1-o1", "검사가 법원에 재판을 청구하는 것"),
                opt("q1-o2", "경찰이 피의자를 체포하는 것"),
                opt("q1-o3", "법원이 형벌을 선고하는 것"),
            ],
        }],
        "disc-1" => vec![
            AnalyzeQuestion {
                id: "keywords".to_string(),
                title: "댓글에 포함된 키워드 (해당 키워드를 포함한 댓글만 선별)".to_string(),
                options: vec![
                    opt("kw-증거", "증거"),
                    opt("kw-진술", "진술"),
                    opt("kw-피해자", "피해자"),
                    opt("kw-처벌", "처벌"),
                    opt("kw-동의", "동의"),
                    opt("kw-무고죄", "무고죄"),
                ],
            },
            AnalyzeQuestion {
                id: "topics".to_string(),
                title: "LDA 토픽".to_string(),
                options: vec![
                    opt("topic-1", "토픽_1: 형량 및 처벌"),
                    opt("topic-2", "토픽_2: 피해자 보호"),
                    opt("topic-3", "토픽_3: 수사 절차"),
                ],
            },
        ],
        "disc-2" => vec![AnalyzeQuestion {
            id: "keywords".to_string(),
            title: "댓글에 포함된 키워드".to_string(),
            options: vec![
                opt("kw-무고죄", "무고죄"),
                opt("kw-기소", "기소"),
                opt("kw-녹음", "녹음"),
                opt("kw-피해자", "피해자"),
            ],
        }],
        "follow-1" => vec![
            AnalyzeQuestion {
                id: "targets".to_string(),
                title: "팔로우 타겟 (해당 타겟을 팔로우한 응답자)".to_string(),
                options: vec![
                    opt("follow-kim", "김변호 변호사"),
                    opt("follow-park", "박판사 판사"),
                    opt("follow-lee", "이검사 검사"),
                    opt("follow-choi", "최교수 법학교수"),
                ],
            },
            AnalyzeQuestion {
                id: "completion".to_string(),
                title: "팔로우 완성도".to_string(),
                options: vec![
                    opt("all-done", "12 타겟 전부 팔로우한 응답자"),
                    opt("half-done", "6 타겟 이상 팔로우한 응답자"),
                    opt("none", "한 명도 팔로우 안 한 응답자"),
                ],
            },
        ],
        _ => Vec::new(),
    }
}

/// Resolve a `${question_id}:${option_id}` token back to question +
/// option records. Used when 확인 promotes checked options into chips.
pub fn resolve_sunji(item_id: &str, sunji_token: &str) -> Option<(AnalyzeQuestion, AnalyzeOption)> {
    let mut parts = sunji_token.splitn(2, ':');
    let qid = parts.next()?;
    let oid = parts.next()?;
    let questions = mock_questions_for(item_id);
    let q = questions.into_iter().find(|q| q.id == qid)?;
    let o = q.options.iter().find(|o| o.id == oid).cloned()?;
    Some((q, o))
}

/// Lookup an action item by id within its source group.
pub fn find_action_item(source: AnalyzeFilterSource, item_id: &str) -> Option<AnalyzeActionItem> {
    mock_action_items(source)
        .into_iter()
        .find(|i| i.id == item_id)
}

// ── Preview-records mock generators ────────────────────────────────

/// One row in a preview-records table. Cells are pre-rendered text +
/// a flag for the "정답" tag (quiz only). The tag flag is broken out
/// so the RSX can render the styled `<span class="prv-tag-correct">`
/// instead of injecting raw HTML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewRow {
    /// `prv-cell--type` — uppercase action label, e.g. "POLL".
    pub type_label: String,
    /// `prv-cell--id` — anonymised display id.
    pub display_id: String,
    /// Third column. For poll/quiz this is the question; for discussion
    /// the discussion title; for follow the target.
    pub third_col: String,
    /// Fourth column ("답변" / "코멘트" / "상태"). The `prv-tag-correct`
    /// chip is rendered separately when `correct_tag` is true.
    pub fourth_col: String,
    /// Quiz-only — append "정답" tag after the answer text.
    pub correct_tag: bool,
}

/// Header columns + paginated body rows for one merged source table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewTable {
    pub source: AnalyzeFilterSource,
    pub source_badge: String,
    /// Comma-joined chip labels for the table header.
    pub header_label: String,
    /// "{n}개 필터 · {m}건"
    pub count_label: String,
    /// Localised column headers in display order.
    pub columns: [&'static str; 4],
    pub rows: Vec<PreviewRow>,
}

const COMMENT_TEMPLATES: &[&str] = &[
    "충분한 KW 없이 처벌만 강화하면 무고한 사람이 피해를 볼 수 있습니다.",
    "KW 능력이 의심되는 진술만으로는 기소를 신중히 해야 합니다.",
    "KW 보호와 가해자 인권 사이의 균형이 필요하다고 봅니다.",
    "KW를 인정하기 위한 절차가 더 명확해야 합니다.",
    "시민의 KW 참여 절차가 우선되어야 한다고 봅니다.",
    "제도적 KW 이 마련되지 않으면 부작용이 더 클 것입니다.",
    "KW 관련 교육이 학교부터 도입되어야 한다고 생각합니다.",
    "KW 가해자에 대한 사회적 낙인을 줄이는 정책이 필요합니다.",
];

const ANON_POOL: &[&str] = &[
    "나비",
    "사슴",
    "별자리",
    "숲",
    "등대",
    "파도",
    "달빛",
    "바람",
    "조약돌",
    "새벽",
    "빗방울",
    "단풍",
    "연꽃",
    "노을",
    "안개",
];

fn build_anon_name(i: usize) -> String {
    let seed = ANON_POOL[i % ANON_POOL.len()];
    let suffix = 100 + (i * 17) % 900;
    format!("익명_{}_{}", seed, suffix)
}

fn discussion_title(item_id: &str) -> &'static str {
    match item_id {
        "disc-1" => "비동의 강간죄 도입에 대해서 어떻게 생각하십니까?",
        "disc-2" => "무고죄 형량 강화에 대한 시민 의견",
        _ => "디스커션",
    }
}

/// Build the per-source merged preview table.
///
/// `filters_for_source` already groups every chip with the same `source`.
/// Columns and row generation come from the HTML mockup's
/// `generateRecordsForFilter` + `buildMergedRecordsTable` helpers.
pub fn build_preview_table(
    source: AnalyzeFilterSource,
    filters_for_source: &[super::report::AnalyzeReportFilter],
) -> PreviewTable {
    let columns: [&'static str; 4] = match source {
        AnalyzeFilterSource::Poll | AnalyzeFilterSource::Quiz => {
            ["ACTION TYPE", "DISPLAY ID", "문항", "답변"]
        }
        AnalyzeFilterSource::Discussion => ["ACTION TYPE", "DISPLAY ID", "제목", "코멘트"],
        AnalyzeFilterSource::Follow => ["ACTION TYPE", "DISPLAY ID", "타겟", "상태"],
    };

    let badge = source.badge().to_string();

    let mut rows: Vec<PreviewRow> = Vec::new();
    for f in filters_for_source.iter() {
        match source {
            AnalyzeFilterSource::Poll | AnalyzeFilterSource::Quiz => {
                let total = 30 + f.option_id.chars().count() * 3;
                for i in 0..total {
                    rows.push(PreviewRow {
                        type_label: badge.clone(),
                        display_id: build_anon_name(i),
                        third_col: f.question_title.clone(),
                        fourth_col: f.option_text.clone(),
                        correct_tag: matches!(source, AnalyzeFilterSource::Quiz) && f.correct,
                    });
                }
            }
            AnalyzeFilterSource::Discussion => {
                let keyword = f.option_text.as_str();
                let total = 12 + keyword.chars().count() * 2;
                let title = discussion_title(&f.item_id);
                for j in 0..total {
                    let template = COMMENT_TEMPLATES[j % COMMENT_TEMPLATES.len()];
                    let comment = template.replace("KW", keyword);
                    rows.push(PreviewRow {
                        type_label: badge.clone(),
                        display_id: build_anon_name(j),
                        third_col: title.to_string(),
                        fourth_col: comment,
                        correct_tag: false,
                    });
                }
            }
            AnalyzeFilterSource::Follow => {
                let target = f.option_text.as_str();
                let status_text = if target.contains("팔로우 안") {
                    "✗ 미수행"
                } else {
                    "✓ 팔로우 완료"
                };
                let total = 22;
                for m in 0..total {
                    rows.push(PreviewRow {
                        type_label: badge.clone(),
                        display_id: build_anon_name(m + 3),
                        third_col: target.to_string(),
                        fourth_col: status_text.to_string(),
                        correct_tag: false,
                    });
                }
            }
        }
    }

    let header_label = filters_for_source
        .iter()
        .map(|f| f.label.clone())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(", ");

    let count_label = format!("{}개 필터 · {}건", filters_for_source.len(), rows.len());

    PreviewTable {
        source,
        source_badge: badge,
        header_label,
        count_label,
        columns,
        rows,
    }
}

/// Page size used by every preview-records pager.
pub const PREVIEW_PAGE_SIZE: usize = 10;

/// Mock "해당 응답자" stat — same formula as the HTML mockup's
/// `pseudoCount`.
pub fn pseudo_respondent_count(filter_count: usize) -> i64 {
    if filter_count == 0 {
        return 128;
    }
    let mut x: f64 = 128.0;
    for _ in 0..filter_count {
        x = (x * 0.65).round();
    }
    x.max(1.0) as i64
}
