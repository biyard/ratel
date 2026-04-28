//! Controller for the Analyze CREATE wizard.
//!
//! Bundles wizard-local state (mode, picker state, chip set, preview
//! name) plus every server-backed loader the picker needs: the four
//! source lists (poll / quiz / discussion / follow), the detail
//! loaders for the currently picked poll or quiz item, and the
//! preview / submit actions.
//!
//! Picker flow per source:
//! - Poll / Quiz: pick action tile → pick item from the radio list →
//!   check options inside cf-sunji → 확인 promotes each checked
//!   option into one chip.
//! - Discussion: pick action tile → pick item → type comma-separated
//!   keywords → 확인 promotes each unique keyword into one chip.
//! - Follow: pick action tile → cf-sunji opens directly with the
//!   target list as multi-select checkboxes (no item layer) → 확인
//!   promotes each checked target into one chip.
//!
//! 다음 calls `handle_compute_preview` which posts the current chip
//! list to the preview endpoint and stashes the count for the
//! preview card. 보고서 생성 calls `handle_submit` which persists the
//! report with `status = InProgress`, refreshes the LIST loader, and
//! navigates back to the LIST arena.

use std::collections::HashSet;

use crate::common::ListResponse;
use crate::features::spaces::pages::actions::actions::poll::PollResponse;
use crate::features::spaces::pages::actions::actions::poll::controllers::get_poll;
use crate::features::spaces::pages::actions::actions::quiz::QuizResponse;
use crate::features::spaces::pages::actions::actions::quiz::controllers::get_quiz;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

/// Sentinel item id used when picking the Follow source. Follow has
/// no item layer — cf-sunji opens directly with the target list.
const FOLLOW_ITEM_SENTINEL: &str = "__follow__";

#[derive(Clone, Copy)]
pub struct UseAnalyzeCreate {
    pub space_id: ReadSignal<SpacePartition>,

    // ── Wizard state ─────────────────────────────────────────────
    pub mode: Signal<CreateMode>,
    pub add_state: Signal<AddState>,
    pub picking_type: Signal<Option<AnalyzeFilterSource>>,
    pub picked_item_id: Signal<Option<String>>,
    pub picked_item_title: Signal<String>,
    pub picked_sunji: Signal<HashSet<String>>,
    pub keyword_input: Signal<String>,
    pub filters: Signal<Vec<AnalyzeReportFilter>>,
    pub preview_name: Signal<String>,

    // ── List loaders for the action picker tiles + radio list ────
    pub polls: Loader<ListResponse<AnalyzePollItem>>,
    pub quizzes: Loader<ListResponse<AnalyzeQuizItem>>,
    pub discussions: Loader<ListResponse<AnalyzeDiscussionItem>>,
    pub follows: Loader<ListResponse<AnalyzeFollowItem>>,

    // ── Selected item detail (poll / quiz only) ──────────────────
    pub selected_poll: Loader<Option<PollResponse>>,
    pub selected_quiz: Loader<Option<QuizResponse>>,

    // ── Preview + submit ─────────────────────────────────────────
    pub preview: Signal<Option<PreviewAnalyzeReportResponse>>,
    pub handle_compute_preview: Action<(), ()>,
    pub handle_submit: Action<(), ()>,
}

impl UseAnalyzeCreate {
    pub fn start_add(&mut self) {
        self.add_state.set(AddState::PickingAction);
        self.reset_picker();
    }

    pub fn cancel_add(&mut self) {
        self.add_state.set(AddState::Idle);
        self.reset_picker();
    }

    /// `picking-action → picking-item`. For Follow we skip the item
    /// layer and pre-populate `picked_item_id` with a sentinel so
    /// cf-sunji opens immediately with the target list.
    pub fn pick_action(&mut self, src: AnalyzeFilterSource) {
        self.picking_type.set(Some(src));
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());

        match src {
            AnalyzeFilterSource::Follow => {
                self.picked_item_id
                    .set(Some(FOLLOW_ITEM_SENTINEL.to_string()));
                self.picked_item_title.set(String::new());
            }
            _ => {
                self.picked_item_id.set(None);
                self.picked_item_title.set(String::new());
            }
        }

        self.add_state.set(AddState::PickingItem);
    }

    pub fn back_to_action(&mut self) {
        self.reset_picker();
        self.add_state.set(AddState::PickingAction);
    }

    pub fn pick_item(&mut self, item_id: String, item_title: String) {
        self.picked_item_id.set(Some(item_id));
        self.picked_item_title.set(item_title);
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
    }

    pub fn clear_item(&mut self) {
        self.picked_item_id.set(None);
        self.picked_item_title.set(String::new());
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
    }

    pub fn toggle_sunji(&mut self, token: String) {
        let mut current = self.picked_sunji.write();
        if current.contains(&token) {
            current.remove(&token);
        } else {
            current.insert(token);
        }
    }

    /// Drain `picked_sunji` (and discussion-only `keyword_input`)
    /// into `filters`, then reset the picker. Each chip stamps the
    /// real `item_id` / `question_id` (= question index) /
    /// `option_id` so the detail page (next stage) can re-derive the
    /// matching respondent set against live response data.
    pub fn confirm_sunji(&mut self) {
        let item_id = match self.picked_item_id.read().clone() {
            Some(id) => id,
            None => return,
        };
        let src = match *self.picking_type.read() {
            Some(s) => s,
            None => return,
        };
        let item_title = self.picked_item_title.read().clone();
        let mut new_filters: Vec<AnalyzeReportFilter> = Vec::new();

        match src {
            AnalyzeFilterSource::Discussion => {
                let raw = self.keyword_input.read().clone();
                let mut seen: HashSet<String> = HashSet::new();
                for kw in raw
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                {
                    if !seen.insert(kw.clone()) {
                        continue;
                    }
                    new_filters.push(AnalyzeReportFilter {
                        source: src,
                        source_label: src.type_label().to_string(),
                        label: kw.clone(),
                        item_id: item_id.clone(),
                        question_id: "keywords".to_string(),
                        option_id: format!("kw-{}", kw),
                        option_text: kw,
                        question_title: item_title.clone(),
                        correct: false,
                    });
                }
            }
            AnalyzeFilterSource::Follow => {
                let follows_resp = self.follows.read().clone();
                let by_pk: std::collections::HashMap<String, AnalyzeFollowItem> = follows_resp
                    .items
                    .iter()
                    .map(|t| (t.user_pk.to_string(), t.clone()))
                    .collect();
                let tokens: Vec<String> =
                    self.picked_sunji.read().iter().cloned().collect();
                for token in tokens {
                    let target = match by_pk.get(&token) {
                        Some(t) => t.clone(),
                        None => continue,
                    };
                    let label = if target.display_name.is_empty() {
                        target.username.clone()
                    } else {
                        target.display_name.clone()
                    };
                    let pk_str = target.user_pk.to_string();
                    new_filters.push(AnalyzeReportFilter {
                        source: src,
                        source_label: src.type_label().to_string(),
                        label: label.clone(),
                        item_id: pk_str.clone(),
                        question_id: String::new(),
                        option_id: pk_str,
                        option_text: label,
                        question_title: String::new(),
                        correct: false,
                    });
                }
            }
            AnalyzeFilterSource::Poll => {
                let detail = self.selected_poll.read().clone();
                let questions = detail
                    .as_ref()
                    .map(|p| p.questions.clone())
                    .unwrap_or_default();
                let title = detail
                    .as_ref()
                    .map(|p| p.title.clone())
                    .unwrap_or_else(|| item_title.clone());
                let tokens: Vec<String> =
                    self.picked_sunji.read().iter().cloned().collect();
                for token in tokens {
                    if let Some(filter) =
                        build_choice_filter(src, &item_id, &title, &questions, &token, false)
                    {
                        new_filters.push(filter);
                    }
                }
            }
            AnalyzeFilterSource::Quiz => {
                let detail = self.selected_quiz.read().clone();
                let questions = detail
                    .as_ref()
                    .map(|q| q.questions.clone())
                    .unwrap_or_default();
                let title = detail
                    .as_ref()
                    .map(|q| q.title.clone())
                    .unwrap_or_else(|| item_title.clone());
                let correct_lookup: std::collections::HashMap<usize, Vec<u32>> = detail
                    .as_ref()
                    .and_then(|q| q.correct_answers.as_ref())
                    .map(|answers| {
                        answers
                            .iter()
                            .enumerate()
                            .filter_map(|(i, a)| correct_indices_for(a).map(|v| (i, v)))
                            .collect()
                    })
                    .unwrap_or_default();
                let tokens: Vec<String> =
                    self.picked_sunji.read().iter().cloned().collect();
                for token in tokens {
                    let mut parts = token.splitn(2, ':');
                    let q_idx: usize = match parts.next().and_then(|s| s.parse().ok()) {
                        Some(v) => v,
                        None => continue,
                    };
                    let o_idx: u32 = match parts.next().and_then(|s| s.parse().ok()) {
                        Some(v) => v,
                        None => continue,
                    };
                    let correct = correct_lookup
                        .get(&q_idx)
                        .map(|v| v.contains(&o_idx))
                        .unwrap_or(false);
                    if let Some(filter) =
                        build_choice_filter(src, &item_id, &title, &questions, &token, correct)
                    {
                        new_filters.push(filter);
                    }
                }
            }
        }

        if !new_filters.is_empty() {
            let mut all = self.filters.write();
            all.extend(new_filters);
        }

        self.add_state.set(AddState::Idle);
        self.reset_picker();
    }

    pub fn remove_filter(&mut self, idx: usize) {
        let mut all = self.filters.write();
        if idx < all.len() {
            all.remove(idx);
        }
    }

    pub fn clear_filters(&mut self) {
        self.filters.set(Vec::new());
    }

    pub fn goto_preview(&mut self) {
        self.mode.set(CreateMode::Preview);
    }

    pub fn back_to_create(&mut self) {
        self.mode.set(CreateMode::Create);
    }

    fn reset_picker(&mut self) {
        self.picking_type.set(None);
        self.picked_item_id.set(None);
        self.picked_item_title.set(String::new());
        self.picked_sunji.set(HashSet::new());
        self.keyword_input.set(String::new());
    }
}

#[track_caller]
pub fn use_analyze_create(
    space_id: ReadSignal<SpacePartition>,
) -> std::result::Result<UseAnalyzeCreate, RenderError> {
    if let Some(ctx) = try_use_context::<UseAnalyzeCreate>() {
        return Ok(ctx);
    }

    let mut toast = use_toast();
    let nav = use_navigator();

    let mode = use_signal(|| CreateMode::Create);
    let add_state = use_signal(|| AddState::Idle);
    let picking_type = use_signal::<Option<AnalyzeFilterSource>>(|| None);
    let picked_item_id = use_signal::<Option<String>>(|| None);
    let picked_item_title = use_signal(String::new);
    let picked_sunji = use_signal(HashSet::<String>::new);
    let keyword_input = use_signal(String::new);
    let filters = use_signal(Vec::<AnalyzeReportFilter>::new);
    let preview_name = use_signal(String::new);
    let mut preview = use_signal::<Option<PreviewAnalyzeReportResponse>>(|| None);

    let polls = use_loader(move || async move { list_analyze_polls(space_id(), None).await })?;
    let quizzes =
        use_loader(move || async move { list_analyze_quizzes(space_id(), None).await })?;
    let discussions =
        use_loader(move || async move { list_analyze_discussions(space_id(), None).await })?;
    let follows = use_loader(move || async move { list_analyze_follows(space_id(), None).await })?;

    // NOTE: signal reads MUST stay outside the inner `async move`. After
    // SSR, `use_loader` short-circuits on the first hydration tick by
    // restoring the cached server result without ever polling the
    // user future, so any `signal.read()` placed inside the async
    // block never establishes a subscription. Subsequent changes to
    // `picked_item_id` / `picking_type` would silently fail to retrigger
    // the loader. Reading the signals in the outer closure runs
    // synchronously every time `use_resource` invokes the closure,
    // which is exactly when reactive subscriptions are recorded.
    let selected_poll = use_loader(move || {
        let id = picked_item_id.read().clone();
        let typ = *picking_type.read();
        let space_id = space_id();
        async move {
            match (typ, id) {
                (Some(AnalyzeFilterSource::Poll), Some(id)) if !id.is_empty() => {
                    let poll_id: SpacePollEntityType = id.into();
                    get_poll(space_id, poll_id).await.map(Some)
                }
                _ => Ok(None),
            }
        }
    })?;
    let selected_quiz = use_loader(move || {
        let id = picked_item_id.read().clone();
        let typ = *picking_type.read();
        let space_id = space_id();
        async move {
            match (typ, id) {
                (Some(AnalyzeFilterSource::Quiz), Some(id)) if !id.is_empty() => {
                    let quiz_id: SpaceQuizEntityType = id.into();
                    get_quiz(space_id, quiz_id).await.map(Some)
                }
                _ => Ok(None),
            }
        }
    })?;

    let mut filters_signal = filters;
    let mut preview_signal = preview;

    let handle_compute_preview = use_action(move || async move {
        let req = PreviewAnalyzeReportRequest {
            filters: filters_signal.read().clone(),
        };
        match preview_analyze_report(space_id(), req).await {
            Ok(resp) => preview_signal.set(Some(resp)),
            Err(err) => {
                crate::error!("preview_analyze_report failed: {err}");
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    let mut reports_ctrl = use_analyze_reports(space_id)?;
    let mut filters_signal2 = filters;
    let mut preview_name_signal = preview_name;
    let mut mode_signal = mode;
    let mut preview_signal2 = preview;

    let handle_submit = use_action(move || async move {
        let name = preview_name_signal.read().trim().to_string();
        if name.is_empty() {
            toast.error(crate::common::Error::InvalidFormat);
            return Ok::<(), crate::common::Error>(());
        }
        let req = CreateAnalyzeReportRequest {
            name,
            filters: filters_signal2.read().clone(),
        };
        match create_analyze_report(space_id(), req).await {
            Ok(_) => {
                filters_signal2.set(Vec::new());
                preview_name_signal.set(String::new());
                preview_signal2.set(None);
                mode_signal.set(CreateMode::Create);
                reports_ctrl.reports.restart();
                nav.push(Route::SpaceAnalyzesAppPage {
                    space_id: space_id(),
                });
            }
            Err(err) => {
                crate::error!("create_analyze_report failed: {err}");
                toast.error(err);
            }
        }
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseAnalyzeCreate {
        space_id,
        mode,
        add_state,
        picking_type,
        picked_item_id,
        picked_item_title,
        picked_sunji,
        keyword_input,
        filters,
        preview_name,
        polls,
        quizzes,
        discussions,
        follows,
        selected_poll,
        selected_quiz,
        preview,
        handle_compute_preview,
        handle_submit,
    }))
}

// ── Helpers ──────────────────────────────────────────────────────

fn build_choice_filter(
    src: AnalyzeFilterSource,
    item_id: &str,
    item_title: &str,
    questions: &[crate::features::spaces::pages::actions::actions::poll::Question],
    token: &str,
    correct: bool,
) -> Option<AnalyzeReportFilter> {
    let mut parts = token.splitn(2, ':');
    let q_idx: usize = parts.next()?.parse().ok()?;
    let o_idx: usize = parts.next()?.parse().ok()?;

    let question = questions.get(q_idx)?;
    let question_title = question.title().to_string();
    let option_text = option_label_at(question, o_idx)?;

    let label = if questions.len() > 1 {
        format!("{} · {}", question_title, option_text)
    } else if !item_title.is_empty() {
        format!("{} · {}", item_title, option_text)
    } else {
        option_text.clone()
    };

    Some(AnalyzeReportFilter {
        source: src,
        source_label: src.type_label().to_string(),
        label,
        item_id: item_id.to_string(),
        question_id: q_idx.to_string(),
        option_id: o_idx.to_string(),
        option_text,
        question_title,
        correct,
    })
}

fn option_label_at(
    question: &crate::features::spaces::pages::actions::actions::poll::Question,
    idx: usize,
) -> Option<String> {
    use crate::features::spaces::pages::actions::actions::poll::Question;
    let opts: &[String] = match question {
        Question::SingleChoice(q) | Question::MultipleChoice(q) => &q.options,
        Question::Checkbox(q) => &q.options,
        Question::Dropdown(q) => &q.options,
        _ => return None,
    };
    opts.get(idx).cloned()
}

fn correct_indices_for(
    answer: &crate::features::spaces::pages::actions::actions::quiz::QuizCorrectAnswer,
) -> Option<Vec<u32>> {
    use crate::features::spaces::pages::actions::actions::quiz::QuizCorrectAnswer;
    match answer {
        QuizCorrectAnswer::Single { answer } => answer.map(|v| vec![v as u32]),
        QuizCorrectAnswer::Multiple { answers } => {
            Some(answers.iter().map(|v| *v as u32).collect())
        }
    }
}

/// Re-export the sentinel so views can detect "follow has no item layer".
pub fn is_follow_sentinel(item_id: &str) -> bool {
    item_id == FOLLOW_ITEM_SENTINEL
}
