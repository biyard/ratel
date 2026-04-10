use crate::features::spaces::pages::actions::actions::poll::components::*;
use crate::features::spaces::pages::actions::actions::poll::controllers::*;
use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use crate::features::spaces::space_common::types::{
    space_my_score_key, space_page_actions_poll_key, space_ranking_key,
};
use std::collections::HashMap;

translate! {
    ActionPollTranslate;

    progress: { en: "Progress", ko: "진행률" },
    question_prefix: { en: "Question", ko: "질문" },
    required: { en: "Required", ko: "필수" },
    poll_type: { en: "Poll", ko: "투표" },
    in_progress: { en: "In Progress", ko: "진행 중" },
    not_started: { en: "Not Started", ko: "시작 전" },
    finished: { en: "Finished", ko: "종료됨" },
    pts_suffix: { en: "pts", ko: "점" },
    btn_back: { en: "Back", ko: "뒤로" },
    btn_next: { en: "Next", ko: "다음" },
    btn_submit: { en: "Submit", ko: "제출" },
    btn_update: { en: "Update", ko: "수정" },
    submit_success: { en: "Response submitted successfully.", ko: "응답이 성공적으로 제출되었습니다." },
    submit_confirm_title: { en: "Submit response", ko: "응답 제출" },
    submit_confirm_desc: { en: "Once submitted, this response cannot be edited. Do you want to submit?", ko: "한번 제출한 응답은 수정이 불가능합니다. 제출하시겠습니까?" },
    submit_confirm_cancel: { en: "Cancel", ko: "취소" },
    submit_confirm_action: { en: "Submit", ko: "제출" },
    poll_ended: { en: "This poll has ended.", ko: "이 투표가 종료되었습니다." },
    poll_not_started: { en: "This poll has not started yet.", ko: "이 투표는 아직 시작되지 않았습니다." },
    no_permission: { en: "You do not have permission to participate.", ko: "이 투표에 참여할 권한이 없습니다." },
    already_responded: { en: "You have already participated.", ko: "이미 이 투표에 참여했습니다." },
    no_questions: { en: "No questions yet.", ko: "아직 질문이 없습니다." },
    subjective_placeholder: { en: "Share your thoughts here...", ko: "의견을 자유롭게 작성해 주세요..." },
}

#[component]
pub fn ActionPollViewer(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
    can_respond: bool,
) -> Element {
    let tr: ActionPollTranslate = use_translate();
    let mut query = use_query_store();
    let mut popup = use_popup();
    let mut toast = use_toast();
    let mut question_index = use_signal(|| 0usize);
    let key = space_page_actions_poll_key(&space_id(), &poll_id());

    let poll_loader = use_query(&key, { move || get_poll(space_id(), poll_id()) })?;
    let poll = poll_loader.read().clone();
    let space = use_space().read().clone();
    let role = use_space_role()();
    let can_execute_action = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        poll.space_action.prerequisite,
        space.status,
        space.join_anytime,
    );
    let nav = navigator();

    let mut answers: Signal<HashMap<usize, Answer>> = use_signal(|| {
        let mut map = HashMap::new();
        if let Some(ref my_resp) = poll.my_response {
            for (i, ans) in my_resp.iter().enumerate() {
                map.insert(i, ans.clone());
            }
        }
        map
    });
    let all_answered = use_memo({
        let poll = poll.clone();
        move || {
            if poll.questions.is_empty() {
                return false;
            }
            let answers_read = answers.read();
            poll.questions
                .iter()
                .enumerate()
                .all(|(idx, q)| has_answer_for_question(q, answers_read.get(&idx)))
        }
    });

    let is_in_progress = poll.status == PollStatus::InProgress;
    let has_response = poll.my_response.is_some();
    let can_submit = can_respond && can_execute_action && is_in_progress && !has_response;
    let can_update =
        can_respond && can_execute_action && is_in_progress && has_response && poll.response_editable;
    let total = poll.questions.len();
    let current_idx = question_index().min(total.saturating_sub(1));
    let current_answer = answers.read().get(&current_idx).cloned();
    let current_question = poll.questions.get(current_idx).cloned();
    let has_current_answer = current_question
        .as_ref()
        .map(|q| has_answer_for_question(q, current_answer.as_ref()))
        .unwrap_or(false);
    let is_first = total == 0 || current_idx == 0;
    let is_last = total == 0 || current_idx + 1 >= total;
    let next_disabled = can_submit && !has_current_answer;
    let show_submit = can_respond && (can_submit || can_update);
    let disabled =
        !can_respond || !can_execute_action || !is_in_progress || (!can_submit && !can_update);
    let progress_pct = if total > 0 {
        ((current_idx + 1) as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };

    // Hide sidebar
    let layout_ui = crate::features::spaces::layout::use_space_layout_ui();
    let sidebar_visible = layout_ui.sidebar_visible;
    use_effect(move || {
        let mut sv = sidebar_visible;
        sv.set(false);
    });
    use_drop(move || {
        let mut sv = sidebar_visible;
        sv.set(true);
    });

    // Submit handler
    let build_submit = {
        let questions = poll.questions.clone();
        move || {
            let questions = questions.clone();
            move || {
                let questions = questions.clone();
                spawn(async move {
                    let m = answers.read().clone();
                    let payload: Vec<Answer> = (0..questions.len())
                        .map(|i| m.get(&i).cloned().unwrap_or_default())
                        .collect();
                    match respond_poll(space_id(), poll_id(), RespondPollRequest { answers: payload })
                        .await
                    {
                        Ok(_) => {
                            query.invalidate(&space_page_actions_poll_key(&space_id(), &poll_id()));
                            query.invalidate(&space_ranking_key(&space_id()));
                            query.invalidate(&space_my_score_key(&space_id()));
                            toast.info(tr.submit_success);
                            nav.replace(crate::Route::SpaceActionsPage { space_id: space_id() });
                        }
                        Err(err) => { toast.error(err); },
                    }
                });
            }
        }
    };
    let on_submit = move |_| {
        if can_submit && !poll.response_editable {
            let mut popup = popup;
            let confirm = build_submit();
            popup
                .open(rsx! {
                    SubmitConfirmDialog {
                        cancel_label: tr.submit_confirm_cancel,
                        confirm_label: tr.submit_confirm_action,
                        on_cancel: move |_| popup.close(),
                        on_confirm: move |_| {
                            popup.close();
                            confirm();
                        },
                    }
                })
                .with_title(tr.submit_confirm_title)
                .with_description(tr.submit_confirm_desc);
        } else {
            build_submit()();
        }
    };

    // Status badge class
    let status_class = match poll.status {
        PollStatus::InProgress => "poll-header__status",
        PollStatus::NotStarted => "poll-header__status poll-header__status--not-started",
        PollStatus::Finish => "poll-header__status poll-header__status--finished",
    };
    let status_text = match poll.status {
        PollStatus::InProgress => tr.in_progress.to_string(),
        PollStatus::NotStarted => tr.not_started.to_string(),
        PollStatus::Finish => tr.finished.to_string(),
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "poll-arena",
            // ─── Header ───
            div { class: "poll-header",
                div { class: "poll-header__left",
                    button {
                        class: "poll-header__back",
                        onclick: move |_| {
                            nav.push(crate::Route::SpaceActionsPage {
                                space_id: space_id(),
                            });
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    div { class: "poll-header__info",
                        span { class: "poll-header__type",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                path { d: "M3 3v18h18" }
                                path { d: "M7 16h4v-6H7z" }
                                path { d: "M13 16h4V8h-4z" }
                            }
                            {tr.poll_type}
                        }
                        span { class: "poll-header__title", {poll.title.clone()} }
                    }
                }
                div { class: "poll-header__right",
                    span { class: "{status_class}", {status_text} }
                    if poll.space_action.activity_score > 0 {
                        span { class: "poll-header__reward",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                            }
                            "+{poll.space_action.activity_score} {tr.pts_suffix}"
                        }
                    }
                }
            }

            // ─── Banners ───
            if poll.status == PollStatus::Finish {
                div { class: "poll-banner poll-banner--info", {tr.poll_ended} }
            }
            if poll.status == PollStatus::NotStarted {
                div { class: "poll-banner poll-banner--info", {tr.poll_not_started} }
            }
            if is_in_progress && !can_execute_action {
                div { class: "poll-banner poll-banner--warning", {tr.no_permission} }
            }
            if is_in_progress && can_execute_action && has_response && !poll.response_editable
                && can_respond
            {
                div { class: "poll-banner poll-banner--info", {tr.already_responded} }
            }

            // ─── Progress ───
            if total > 0 {
                div { class: "poll-progress",
                    div { class: "poll-progress__top",
                        span { class: "poll-progress__label", {tr.progress} }
                        span { class: "poll-progress__fraction", "{current_idx + 1} / {total}" }
                    }
                    div { class: "poll-progress__bar-wrap",
                        div {
                            class: "poll-progress__bar",
                            style: "width: {progress_pct}%",
                        }
                    }
                    div { class: "poll-progress__dots",
                        for dot_idx in 0..total {
                            {
                                let is_active = dot_idx == current_idx;
                                let is_answered = answers.read().contains_key(&dot_idx);
                                rsx! {
                                    button {
                                        key: "dot-{dot_idx}",
                                        class: "poll-progress__dot",
                                        "data-active": is_active,
                                        "data-answered": !is_active && is_answered,
                                        onclick: move |_| question_index.set(dot_idx),
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ─── Question Card ───
            if total == 0 {
                div { class: "question-stage",
                    div { class: "question-card",
                        span { class: "question-card__desc", {tr.no_questions} }
                    }
                }
            }
            if total > 0 {
                {
                    let idx = current_idx;
                    let question = poll.questions[idx].clone();
                    let can_next = idx + 1 < total;
                    rsx! {
                        div { key: "poll-q-{idx}", class: "question-stage",
                            div { class: "question-card",
                                // Number + required badge
                                div { class: "question-card__number",
                                    "{tr.question_prefix} {idx + 1:02}"
                                    {
                                        let is_req = match &question {
                                            Question::SingleChoice(q) => q.is_required.unwrap_or(false),
                                            Question::MultipleChoice(q) => q.is_required.unwrap_or(false),
                                            Question::ShortAnswer(q) => q.is_required.unwrap_or(false),
                                            Question::Subjective(q) => q.is_required.unwrap_or(false),
                                            Question::Checkbox(q) => q.is_required.unwrap_or(false),
                                            Question::Dropdown(q) => q.is_required.unwrap_or(false),
                                            Question::LinearScale(q) => q.is_required.unwrap_or(false),
                                        };
                                        rsx! {
                                            if is_req {
                                                span { class: "question-card__required", {tr.required} }
                                            }
                                        }
                                    }
                                }
                                // Title
                                h2 { class: "question-card__title", {question.title()} }
                                // Description
                                {
                                    let desc = match &question {
                                        Question::SingleChoice(q) => q.description.clone(),
                                        Question::MultipleChoice(q) => q.description.clone(),
                                        // Image
                                        // Answer input
                                        Question::ShortAnswer(q) => {
                                            Some(q.description.clone()).filter(|d| !d.is_empty())
                                        }
                                        Question::Subjective(q) => {
                                            Some(q.description.clone()).filter(|d| !d.is_empty())
                                        }
                                        Question::Checkbox(q) => q.description.clone(),
                                        Question::Dropdown(q) => q.description.clone(),
                                        Question::LinearScale(q) => q.description.clone(),
                                    };
                                    rsx! {
                                        if let Some(d) = desc {
                                            if !d.is_empty() {
                                                p { class: "question-card__desc", {d} }
                                            }
                                        }
                                    }
                                }
                                {
                                    let img_url = match &question {
                                        Question::SingleChoice(q) => q.image_url.clone(),
                                        Question::MultipleChoice(q) => q.image_url.clone(),
                                        Question::Checkbox(q) => q.image_url.clone(),
                                        Question::Dropdown(q) => q.image_url.clone(),
                                        Question::LinearScale(q) => q.image_url.clone(),
                                        _ => None,
                                    };
                                    rsx! {
                                        if let Some(url) = img_url {
                                            if !url.is_empty() {
                                                img {
                                                    class: "question-card__image",
                                                    src: "{url}",
                                                    alt: "Question image",
                                                }
                                            }
                                        }
                                    }
                                }
                                match question.clone() {
                                    Question::SingleChoice(q) => rsx! {
                                        PollSingleChoice {
                                            idx,
                                            question: q,
                                            answer: current_answer.clone(),
                                            disabled,
                                            on_change: move |ans: Answer| {
                                                answers.write().insert(idx, ans.clone());
                                                if can_submit && can_next && should_auto_next(&question, &ans) {
                                                    question_index.set(idx + 1);
                                                }
                                            },
                                        }
                                    },
                                    Question::MultipleChoice(q) => rsx! {
                                        PollMultipleChoice {
                                            idx,
                                            question: q,
                                            answer: current_answer.clone(),
                                            disabled,
                                            on_change: move |ans: Answer| {
                                                answers.write().insert(idx, ans);
                                            },
                                        }
                                    },
                                    Question::Subjective(q) => rsx! {
                                        PollSubjective {
                                            idx,
                                            question: q,
                                            answer: current_answer.clone(),
                                            disabled,
                                            is_short: false,
                                            on_change: move |ans: Answer| {
                                                answers.write().insert(idx, ans);
                                            },
                                        }
                                    },
                                    Question::ShortAnswer(q) => rsx! {
                                        PollSubjective {
                                            idx,
                                            question: q,
                                            answer: current_answer.clone(),
                                            disabled,
                                            is_short: true,
                                            on_change: move |ans: Answer| {
                                                answers.write().insert(idx, ans);
                                            },
                                        }
                                    },
                                    Question::LinearScale(q) => rsx! {
                                        {
                                            let q_auto = q.clone();
                                            rsx! {
                                                PollLinearScale {
                                                    idx,
                                                    question: q,
                                                    answer: current_answer.clone(),
                                                    disabled,
                                                    on_change: move |ans: Answer| {
                                                        answers.write().insert(idx, ans.clone());
                                                        if can_submit && can_next
                                                            && should_auto_next(&Question::LinearScale(q_auto.clone()), &ans)
                                                        {
                                                            question_index.set(idx + 1);
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    },
                                    Question::Checkbox(q) => rsx! {
                                        PollCheckbox {
                                            idx,
                                            question: q,
                                            answer: current_answer.clone(),
                                            disabled,
                                            on_change: move |ans: Answer| {
                                                answers.write().insert(idx, ans);
                                            },
                                        }
                                    },
                                    Question::Dropdown(q) => rsx! {
                                        PollDropdown {
                                            idx,
                                            question: q,
                                            answer: current_answer.clone(),
                                            disabled,
                                            on_change: move |ans: Answer| {
                                                answers.write().insert(idx, ans);
                                            },
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            }

            // ─── Footer ───
            div { class: "poll-footer",
                div { class: "poll-footer__nav",
                    if !is_first && total > 0 {
                        button {
                            class: "poll-btn poll-btn--back",
                            onclick: move |_| {
                                if current_idx > 0 {
                                    question_index.set(current_idx - 1);
                                }
                            },
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                polyline { points: "15 18 9 12 15 6" }
                            }
                            {tr.btn_back}
                        }
                    }
                }
                div { class: "poll-footer__nav",
                    if !is_last && total > 0 {
                        button {
                            class: "poll-btn poll-btn--next",
                            disabled: next_disabled,
                            onclick: move |_| {
                                if current_idx + 1 < total {
                                    question_index.set(current_idx + 1);
                                }
                            },
                            {tr.btn_next}
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                polyline { points: "9 18 15 12 9 6" }
                            }
                        }
                    }
                    if is_last && show_submit && total > 0 {
                        button {
                            class: "poll-btn poll-btn--submit",
                            disabled: !all_answered(),
                            onclick: on_submit,
                            if can_update {
                                {tr.btn_update}
                            } else {
                                {tr.btn_submit}
                            }
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                "stroke-width": "2",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── Submit Confirm ────────────────────────────────────────────────────────

#[component]
fn SubmitConfirmDialog(
    cancel_label: String,
    confirm_label: String,
    on_cancel: EventHandler<MouseEvent>,
    on_confirm: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex gap-3 justify-end w-full",
            Button {
                style: ButtonStyle::Outline,
                shape: ButtonShape::Square,
                class: "min-w-[120px]",
                onclick: move |e| on_cancel.call(e),
                {cancel_label}
            }
            Button {
                "data-testid": "poll-confirm-submit",
                style: ButtonStyle::Primary,
                shape: ButtonShape::Square,
                class: "min-w-[120px]",
                onclick: move |e| on_confirm.call(e),
                {confirm_label}
            }
        }
    }
}

// ─── Single Choice ─────────────────────────────────────────────────────────

#[component]
fn PollSingleChoice(
    idx: usize,
    question: ChoiceQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected = match &answer {
        Some(Answer::SingleChoice { answer, .. }) => *answer,
        _ => None,
    };
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];

    rsx! {
        div { class: "options-single",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_sel = selected == Some(opt_idx as i32);
                    let oi = opt_idx as i32;
                    let letter = letters.get(opt_idx).copied().unwrap_or('?');
                    let on_change = on_change.clone();
                    rsx! {
                        div {
                            key: "sc-{idx}-{oi}",
                            class: "option-single",
                            "data-selected": is_sel,
                            "data-disabled": disabled,
                            onclick: move |_| {
                                if !disabled {
                                    on_change
                                        .call(Answer::SingleChoice {
                                            answer: if is_sel { None } else { Some(oi) },
                                            other: None,
                                        });
                                }
                            },
                            span { class: "option-single__letter", "{letter}" }
                            div { class: "option-single__radio",
                                div { class: "option-single__radio-dot" }
                            }
                            span { class: "option-single__label", "{option}" }
                        }
                    }
                }
            }
        }
    }
}

// ─── Multiple Choice ───────────────────────────────────────────────────────

#[component]
fn PollMultipleChoice(
    idx: usize,
    question: ChoiceQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected: Vec<i32> = match &answer {
        Some(Answer::MultipleChoice { answer, .. }) => answer.clone().unwrap_or_default(),
        _ => vec![],
    };

    rsx! {
        div { class: "options-multi",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_sel = selected.contains(&(opt_idx as i32));
                    let oi = opt_idx as i32;
                    let selected = selected.clone();
                    let on_change = on_change.clone();
                    rsx! {
                        div {
                            key: "mc-{idx}-{oi}",
                            class: "option-multi",
                            "data-selected": is_sel,
                            "data-disabled": disabled,
                            onclick: move |_| {
                                if !disabled {
                                    let mut next = selected.clone();
                                    if next.contains(&oi) {
                                        next.retain(|&x| x != oi);
                                    } else {
                                        next.push(oi);
                                    }
                                    on_change
                                        .call(Answer::MultipleChoice {
                                            answer: Some(next),
                                            other: None,
                                        });
                                }
                            },
                            div { class: "option-multi__checkbox",
                                span { class: "option-multi__check",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        "stroke-width": "3",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        polyline { points: "20 6 9 17 4 12" }
                                    }
                                }
                            }
                            span { class: "option-multi__label", "{option}" }
                        }
                    }
                }
            }
        }
    }
}

// ─── Subjective ────────────────────────────────────────────────────────────

#[component]
fn PollSubjective(
    idx: usize,
    question: SubjectiveQuestion,
    answer: Option<Answer>,
    disabled: bool,
    is_short: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let tr: ActionPollTranslate = use_translate();
    let current_value = match &answer {
        Some(Answer::ShortAnswer { answer }) => answer.clone().unwrap_or_default(),
        Some(Answer::Subjective { answer }) => answer.clone().unwrap_or_default(),
        _ => String::new(),
    };
    let mut draft = use_signal(|| current_value.clone());
    let mut synced = use_signal(|| current_value.clone());
    use_effect(use_reactive((&current_value,), move |(cv,)| {
        if synced() != cv {
            synced.set(cv.clone());
            draft.set(cv);
        }
    }));
    let char_count = draft().len();

    rsx! {
        div { class: "subjective-wrap",
            if is_short {
                input {
                    class: "subjective-input",
                    r#type: "text",
                    placeholder: tr.subjective_placeholder,
                    disabled,
                    value: "{draft()}",
                    oninput: move |evt: Event<FormData>| {
                        let v = evt.value().to_string();
                        draft.set(v.clone());
                        on_change
                            .call(Answer::ShortAnswer {
                                answer: Some(v),
                            });
                    },
                }
            } else {
                textarea {
                    class: "subjective-textarea",
                    placeholder: tr.subjective_placeholder,
                    disabled,
                    value: "{draft()}",
                    maxlength: 2000,
                    oninput: move |evt: Event<FormData>| {
                        let v = evt.value().to_string();
                        draft.set(v.clone());
                        on_change
                            .call(Answer::Subjective {
                                answer: Some(v),
                            });
                    },
                }
                span { class: "subjective-counter", "{char_count} / 2000" }
            }
        }
    }
}

// ─── Linear Scale ──────────────────────────────────────────────────────────

#[component]
fn PollLinearScale(
    idx: usize,
    question: LinearScaleQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected = match &answer {
        Some(Answer::LinearScale { answer }) => *answer,
        _ => None,
    };

    rsx! {
        div { class: "scale-wrap",
            div { class: "scale-labels",
                span { class: "scale-label scale-label--min", {question.min_label} }
                span { class: "scale-label scale-label--max", {question.max_label} }
            }
            div { class: "scale-track",
                for val in question.min_value..=question.max_value {
                    {
                        let is_sel = selected == Some(val as i32);
                        let in_range = selected
                            .map_or(
                                false,
                                |s| (val as i32) < s && (val as i32) >= (question.min_value as i32),
                            );
                        let on_change = on_change.clone();
                        rsx! {
                            button {
                                key: "sc-{idx}-{val}",
                                class: "scale-point",
                                "data-selected": is_sel,
                                "data-in-range": in_range,
                                "data-disabled": disabled,
                                disabled,
                                onclick: move |_| {
                                    on_change
                                        .call(Answer::LinearScale {
                                            answer: Some(val as i32),
                                        });
                                },
                                "{val}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── Checkbox ──────────────────────────────────────────────────────────────

#[component]
fn PollCheckbox(
    idx: usize,
    question: CheckboxQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected: Vec<i32> = match &answer {
        Some(Answer::Checkbox { answer }) => answer.clone().unwrap_or_default(),
        _ => vec![],
    };

    rsx! {
        div { class: "options-multi",
            for (opt_idx , option) in question.options.iter().enumerate() {
                {
                    let is_sel = selected.contains(&(opt_idx as i32));
                    let oi = opt_idx as i32;
                    let selected = selected.clone();
                    let is_multi = question.is_multi;
                    let on_change = on_change.clone();
                    rsx! {
                        div {
                            key: "cb-{idx}-{oi}",
                            class: "option-multi",
                            "data-selected": is_sel,
                            "data-disabled": disabled,
                            onclick: move |_| {
                                if !disabled {
                                    let mut next = selected.clone();
                                    if is_multi {
                                        if next.contains(&oi) {
                                            next.retain(|&x| x != oi);
                                        } else {
                                            next.push(oi);
                                        }
                                    } else if is_sel {
                                        next.clear();
                                    } else {
                                        next = vec![oi];
                                    }
                                    on_change
                                        .call(Answer::Checkbox {
                                            answer: Some(next),
                                        });
                                }
                            },
                            div { class: "option-multi__checkbox",
                                span { class: "option-multi__check",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        "stroke-width": "3",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        polyline { points: "20 6 9 17 4 12" }
                                    }
                                }
                            }
                            span { class: "option-multi__label", "{option}" }
                        }
                    }
                }
            }
        }
    }
}

// ─── Dropdown ──────────────────────────────────────────────────────────────

#[component]
fn PollDropdown(
    idx: usize,
    question: DropdownQuestion,
    answer: Option<Answer>,
    disabled: bool,
    on_change: EventHandler<Answer>,
) -> Element {
    let selected = match &answer {
        Some(Answer::Dropdown { answer }) => *answer,
        _ => None,
    };

    rsx! {
        select {
            class: "subjective-input",
            disabled,
            onchange: move |evt| {
                let idx: Option<i32> = evt.value().to_string().parse().ok();
                on_change.call(Answer::Dropdown { answer: idx });
            },
            option { value: "", selected: selected.is_none(), "Select..." }
            for (oi , opt) in question.options.iter().enumerate() {
                {
                    let v = format!("{oi}");
                    let is_sel = selected == Some(oi as i32);
                    rsx! {
                        option { value: "{v}", selected: is_sel, "{opt}" }
                    }
                }
            }
        }
    }
}
