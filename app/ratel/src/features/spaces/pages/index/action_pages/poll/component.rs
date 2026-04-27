use super::checkbox_choice::*;
use super::dropdown_choice::*;
use super::linear_scale::*;
use super::multi_choice::*;
use super::single_choice::*;
use super::subjective::*;
use crate::features::spaces::pages::actions::actions::poll::components::*;
use crate::features::spaces::pages::actions::actions::poll::controllers::*;
use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::index::action_pages::quiz::{
    ActiveActionOverlaySignal, CompletedActionCard,
};
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};

translate! {
    ActionPollTranslate;

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
    other_placeholder: { en: "Enter your answer...", ko: "기타 응답을 입력하세요..." },
    questions_label: { en: "Questions", ko: "질문" },
    credits_label: { en: "Credits", ko: "크레딧" },
    reward_label: { en: "Reward", ko: "보상" },
    begin_poll: { en: "Begin Poll", ko: "투표 시작" },
    encrypted_secret_title: {
        en: "Set up encrypted voting",
        ko: "암호화 투표 설정",
    },
    encrypted_secret_description: {
        en: "This poll's results are encrypted and stored on-chain. Set a password used to encrypt your vote. You can verify your ballot later with the same password.",
        ko: "이 투표의 결과는 암호화되어 블록체인에 저장됩니다. 투표 결과를 암호화 하는데 사용할 비밀번호를 설정하세요. 동일한 비밀번호로 투표를 검증할 수 있습니다.",
    },
    encrypted_secret_placeholder: { en: "Password", ko: "비밀번호" },
    encrypted_secret_continue: { en: "Continue", ko: "계속" },
    encrypted_secret_cancel: { en: "Cancel", ko: "취소" },
    encrypted_secret_required: {
        en: "Encrypted polls require a password before submitting.",
        ko: "암호화 투표는 제출 전에 비밀번호 설정이 필요합니다.",
    },
    encrypted_material_load_failed: {
        en: "Failed to load encryption material.",
        ko: "암호화 자료 로드에 실패했습니다.",
    },
    encrypted_material_loading: {
        en: "Preparing encryption keys… this can take a few seconds.",
        ko: "암호화 키를 준비 중입니다… 몇 초 걸릴 수 있어요.",
    },
    encrypted_secret_applying: {
        en: "Applying…",
        ko: "적용 중…",
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PollStep {
    Overview,
    Poll,
}

fn default_poll_answer(question: &Question) -> Answer {
    match question {
        Question::SingleChoice(_) => Answer::SingleChoice {
            answer: None,
            other: None,
        },
        Question::MultipleChoice(_) => Answer::MultipleChoice {
            answer: None,
            other: None,
        },
        Question::ShortAnswer(_) => Answer::ShortAnswer { answer: None },
        Question::Subjective(_) => Answer::Subjective { answer: None },
        Question::Checkbox(_) => Answer::Checkbox { answer: None },
        Question::Dropdown(_) => Answer::Dropdown { answer: None },
        Question::LinearScale(_) => Answer::LinearScale { answer: None },
    }
}

#[component]
pub fn ActionPollViewer(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
    can_respond: bool,
) -> Element {
    let tr: ActionPollTranslate = use_translate();
    let mut space_ctx = crate::features::spaces::space_common::providers::use_space_context();
    let mut toast = use_toast();
    let nav = navigator();
    let role = use_space_role()();
    let space = use_space()();

    let mut poll_loader = use_loader(move || get_poll(space_id(), poll_id()))?;
    let poll = poll_loader();
    let questions = poll.questions.clone();
    let total = questions.len();

    let can_execute_action = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        poll.space_action.prerequisite,
        space.status,
        poll.space_action.status.as_ref(),
        true,
        space.join_anytime,
    );

    let is_in_progress = poll.status == PollStatus::InProgress;
    let has_response = poll.my_response.is_some();
    let can_submit = can_respond && can_execute_action && is_in_progress && !has_response;
    let can_update = can_respond
        && can_execute_action
        && is_in_progress
        && has_response
        && poll.response_editable;
    let show_submit = can_respond && (can_submit || can_update);
    let disabled =
        !can_respond || !can_execute_action || !is_in_progress || (!can_submit && !can_update);
    let response_editable = poll.response_editable;

    let mut question_index = use_signal(|| 0usize);
    let mut step = use_signal(|| PollStep::Overview);
    let mut show_confirm = use_signal(|| false);

    let encryption_enabled = poll.encrypted_upload_enabled;
    let mut encryption_material = use_signal(|| None::<VoteEncryptionMaterialResponse>);
    let mut client_secret = use_signal(|| None::<String>);
    let mut secret_input = use_signal(String::new);
    let mut show_secret_modal = use_signal(|| false);
    let mut loading_material = use_signal(|| false);
    let mut applying_secret = use_signal(|| false);
    let needs_secret = encryption_enabled && can_submit;

    // Open the modal immediately and kick off the (slow) keygen request in the
    // background so the user sees a loading state instead of a blank page.
    use_effect(move || {
        if !needs_secret {
            return;
        }
        spawn(async move {
            if let Some(s) = load_session_vote_secret().await {
                if !s.is_empty() && client_secret().is_none() {
                    // Reuse session secret as soon as material arrives.
                    secret_input.set(s);
                }
            }
            if client_secret().is_none() {
                show_secret_modal.set(true);
            }
            if encryption_material().is_none() && !loading_material() {
                loading_material.set(true);
                match get_encryption_material(space_id(), poll_id()).await {
                    Ok(m) => encryption_material.set(Some(m)),
                    Err(_) => {
                        toast.warn(tr.encrypted_material_load_failed.to_string());
                    }
                }
                loading_material.set(false);
            }
        });
    });

    // When both material and a session secret are present, finalize without
    // requiring the user to press Continue again.
    use_effect(move || {
        if client_secret().is_some() {
            return;
        }
        let Some(material) = encryption_material() else {
            return;
        };
        let secret = secret_input();
        if secret.is_empty() {
            return;
        }
        // Only auto-finalize when this came from session storage (modal hidden).
        if show_secret_modal() {
            return;
        }
        match build_stored_voter_key_from_encryption_material(&material, &secret) {
            Ok(stored) => {
                let _ = save_stored_voter_key(&stored);
                client_secret.set(Some(secret));
            }
            Err(e) => {
                toast.warn(e);
            }
        }
    });

    // Pre-fill with defaults so `write()[idx] = ans` never panics even if
    // `my_response` has fewer entries than current questions.
    let initial_answers: Vec<Answer> = {
        let mut base: Vec<Answer> = questions.iter().map(default_poll_answer).collect();
        if let Some(resp) = poll.my_response.clone() {
            for (i, ans) in resp.into_iter().enumerate().take(base.len()) {
                base[i] = ans;
            }
        }
        base
    };
    let mut answers = use_signal(|| initial_answers);

    let questions_for_memo = questions.clone();
    let all_answered = use_memo(move || {
        if total == 0 {
            return false;
        }
        let ans = answers.read();
        questions_for_memo
            .iter()
            .enumerate()
            .all(|(i, q)| has_answer_for_question(q, ans.get(i)))
    });

    let current_idx = question_index().min(total.saturating_sub(1));
    let is_first = total == 0 || current_idx == 0;
    let is_last = total == 0 || current_idx + 1 >= total;

    // Hide sidebar while overlay is open.
    let layout_ui = crate::features::spaces::layout::use_space_layout_ui();
    let mut sidebar_visible = layout_ui.sidebar_visible;
    use_effect(move || sidebar_visible.set(false));
    use_drop(move || sidebar_visible.set(true));

    let overlay: Option<ActiveActionOverlaySignal> = try_consume_context();
    let completed: Option<CompletedActionCard> = try_consume_context();

    let do_submit = Callback::new(move |_: ()| {
        spawn(async move {
            let mut req = RespondPollRequest {
                answers: answers(),
                client_ciphertext_json: None,
                client_voter_tag: None,
            };

            if encryption_enabled {
                let material = encryption_material();
                let secret = client_secret();
                let now = crate::common::utils::time::get_now_timestamp_millis();
                match (material, secret) {
                    (Some(m), Some(_)) => match encrypt_answers_for_canister(&m, &answers(), now) {
                        Ok(enc) => {
                            req.client_ciphertext_json = Some(enc.ciphertext_json);
                            req.client_voter_tag = Some(enc.voter_tag);
                        }
                        Err(e) => {
                            toast.warn(e);
                            return;
                        }
                    },
                    _ => {
                        toast.warn(tr.encrypted_secret_required.to_string());
                        show_secret_modal.set(true);
                        return;
                    }
                }
            }

            match respond_poll(space_id(), poll_id(), req).await {
                Ok(_) => {
                    space_ctx.ranking.restart();
                    space_ctx.my_score.restart();
                    // Restart actions list so dashboard refreshes after animation
                    space_ctx.actions.restart();
                    toast.info(tr.submit_success);
                    if let Some(mut ov) = overlay {
                        // Signal the dashboard to animate this card into archive
                        if let Some(mut c) = completed {
                            c.0.set(Some(poll_id().to_string()));
                        }
                        ov.0.set(None);
                    } else {
                        poll_loader.restart();
                        nav.replace(crate::Route::SpaceIndexPage {
                            space_id: space_id(),
                        });
                    }
                }
                Err(err) => {
                    toast.error(err);
                }
            }
        });
    });

    let on_submit = move |_| {
        if can_submit && !response_editable {
            show_confirm.set(true);
        } else {
            do_submit.call(());
        }
    };

    let status_text = match poll.status {
        PollStatus::InProgress => tr.in_progress.to_string(),
        PollStatus::NotStarted => tr.not_started.to_string(),
        PollStatus::Finish => tr.finished.to_string(),
    };
    let status_class = match poll.status {
        PollStatus::InProgress => "poll-header__status",
        PollStatus::NotStarted => "poll-header__status poll-header__status--not-started",
        PollStatus::Finish => "poll-header__status poll-header__status--finished",
    };

    let current_question = questions.get(current_idx).cloned();
    let has_current_answer = current_question
        .as_ref()
        .map(|q| has_answer_for_question(q, answers.read().get(current_idx)))
        .unwrap_or(false);
    let next_disabled = can_submit && !has_current_answer;

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }

        div { class: "poll-arena",
            // ─── Header (HTML design poll-header) ───
            div { class: "poll-header",
                div { class: "poll-header__left",
                    button {
                        class: "poll-header__back",
                        onclick: move |_| {
                            if let Some(mut ov) = overlay {
                                ov.0.set(None);
                            } else {
                                nav.push(crate::Route::SpaceIndexPage {
                                    space_id: space_id(),
                                });
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
                        span { class: "poll-header__title", "{poll.title}" }
                    }
                }
                div { class: "poll-header__right",
                    if step() == PollStep::Poll && total > 0 {
                        span { class: "poll-header__counter", "{current_idx + 1}/{total}" }
                    }
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

            // ─── SCREEN 1: Overview ───
            if step() == PollStep::Overview {
                div {
                    class: "poll-overview",
                    "data-testid": "poll-arena-overview",

                    div { class: "poll-overview-ring",
                        svg {
                            class: "poll-overview-ring__svg",
                            view_box: "0 0 140 140",
                            circle {
                                class: "poll-overview-ring__bg",
                                cx: "70",
                                cy: "70",
                                r: "60",
                            }
                            circle {
                                class: "poll-overview-ring__fill",
                                cx: "70",
                                cy: "70",
                                r: "60",
                            }
                        }
                        div { class: "poll-overview-ring__center",
                            span { class: "poll-overview-ring__number", "{total}" }
                            span { class: "poll-overview-ring__label", {tr.questions_label} }
                        }
                    }

                    div { class: "poll-overview-card",
                        div { class: "poll-overview-card__title", "{poll.title}" }
                        if !poll.description.is_empty() {
                            div {
                                class: "poll-overview-card__desc",
                                dangerous_inner_html: "{poll.description}",
                            }
                        }

                        div { class: "poll-overview-stats",
                            div { class: "poll-overview-stat",
                                span { class: "poll-overview-stat__value", "{total}" }
                                span { class: "poll-overview-stat__label", {tr.questions_label} }
                            }
                            if poll.space_action.activity_score > 0 {
                                div { class: "poll-overview-stat",
                                    span { class: "poll-overview-stat__value",
                                        "+{poll.space_action.activity_score}"
                                    }
                                    span { class: "poll-overview-stat__label", {tr.reward_label} }
                                }
                            }
                            if poll.space_action.credits > 0 {
                                div { class: "poll-overview-stat",
                                    span { class: "poll-overview-stat__value",
                                        "{poll.space_action.credits}"
                                    }
                                    span { class: "poll-overview-stat__label", {tr.credits_label} }
                                }
                            }
                        }

                        button {
                            class: "poll-begin-btn",
                            "data-testid": "poll-arena-begin",
                            // Allow viewing questions even when the poll has ended
                            // or the user cannot submit — read-only mode.
                            disabled: total == 0,
                            onclick: move |_| {
                                question_index.set(0);
                                step.set(PollStep::Poll);
                            },
                            {tr.begin_poll}
                        }
                    }
                }
            }

            // ─── Question Card (only when step is Poll) ───
            if step() == PollStep::Poll && total == 0 {
                div { class: "question-stage",
                    div { class: "question-card",
                        span { class: "question-card__desc", {tr.no_questions} }
                    }
                }
            }
            if step() == PollStep::Poll && total > 0 {
                {
                    let idx = current_idx;
                    let question = questions[idx].clone();
                    let current_answer = answers.read().get(idx).cloned();
                    let can_next = idx + 1 < total;
                    rsx! {
                        div { key: "poll-q-{idx}", class: "question-stage",
                            div { class: "question-card",
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
                                h2 { class: "question-card__title", {question.title()} }
                                {
                                    let desc = match &question {
                                        Question::SingleChoice(q) => q.description.clone(),
                                        Question::MultipleChoice(q) => q.description.clone(),
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
                                                answers.write()[idx] = ans.clone();
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
                                                answers.write()[idx] = ans;
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
                                                answers.write()[idx] = ans;
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
                                                answers.write()[idx] = ans;
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
                                                        answers.write()[idx] = ans.clone();
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
                                                answers.write()[idx] = ans;
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
                                                answers.write()[idx] = ans;
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
            if step() == PollStep::Poll {
                div { class: "poll-footer",
                    // Right: nav buttons
                    div { class: "poll-footer__right",
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
                                "data-testid": "poll-submit",
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
            } // end if step == Poll (footer)

            // ─── Confirm Modal ───
            if show_confirm() {
                {
                    rsx! {
                        div { class: "poll-confirm-overlay", onclick: move |_| show_confirm.set(false),
                            div { class: "poll-confirm-modal", onclick: move |e| e.stop_propagation(),
                                div { class: "poll-confirm-modal__icon",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        "stroke-width": "2",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                                        line {
                                            x1: "12",
                                            y1: "9",
                                            x2: "12",
                                            y2: "13",
                                        }
                                        line {
                                            x1: "12",
                                            y1: "17",
                                            x2: "12.01",
                                            y2: "17",
                                        }
                                    }
                                }
                                h3 { class: "poll-confirm-modal__title", {tr.submit_confirm_title} }
                                p { class: "poll-confirm-modal__desc", {tr.submit_confirm_desc} }
                                div { class: "poll-confirm-modal__actions",
                                    button {
                                        class: "poll-btn poll-btn--back",
                                        onclick: move |_| show_confirm.set(false),
                                        {tr.submit_confirm_cancel}
                                    }
                                    button {
                                        class: "poll-btn poll-btn--submit",
                                        "data-testid": "poll-confirm-submit",
                                        onclick: move |_| {
                                            show_confirm.set(false);
                                            do_submit.call(());
                                        },
                                        {tr.submit_confirm_action}
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ─── Encrypted-vote Secret Setup Modal ───
            if show_secret_modal() {
                div { class: "poll-confirm-overlay",
                    div {
                        class: "poll-confirm-modal",
                        onclick: move |e| e.stop_propagation(),
                        h3 { class: "poll-confirm-modal__title", {tr.encrypted_secret_title} }
                        p { class: "poll-confirm-modal__desc", {tr.encrypted_secret_description} }
                        input {
                            "data-testid": "poll-encrypted-secret-input",
                            r#type: "password",
                            placeholder: "{tr.encrypted_secret_placeholder}",
                            value: "{secret_input()}",
                            disabled: applying_secret(),
                            style: "width: 100%; padding: 10px 12px; border-radius: 8px; border: 1px solid var(--color-card-border, #2a2a3a); background: var(--color-input-box-bg, #0c0c1a); color: var(--color-text-primary, #f0f0f5); font-size: 14px; margin: 12px 0;",
                            oninput: move |evt| secret_input.set(evt.value()),
                        }
                        if loading_material() && encryption_material().is_none() {
                            div {
                                style: "display: flex; align-items: center; gap: 8px; color: var(--color-foreground-muted, #8888a8); font-size: 13px; margin: 4px 0 12px;",
                                "data-testid": "poll-encrypted-material-loading",
                                span { style: "display: inline-block; width: 14px; height: 14px; border: 2px solid currentColor; border-top-color: transparent; border-radius: 50%; animation: spin 0.8s linear infinite;" }
                                {tr.encrypted_material_loading}
                            }
                            style { "@keyframes spin {{ to {{ transform: rotate(360deg); }} }}" }
                        }
                        div { class: "poll-confirm-modal__actions",
                            button {
                                class: "poll-btn poll-btn--back",
                                disabled: applying_secret(),
                                onclick: move |_| {
                                    secret_input.set(String::new());
                                    show_secret_modal.set(false);
                                    if let Some(mut ov) = overlay {
                                        ov.0.set(None);
                                    } else {
                                        nav.push(crate::Route::SpaceIndexPage {
                                            space_id: space_id(),
                                        });
                                    }
                                },
                                {tr.encrypted_secret_cancel}
                            }
                            button {
                                class: "poll-btn poll-btn--submit",
                                "data-testid": "poll-encrypted-secret-continue",
                                disabled: secret_input().is_empty() || loading_material() || encryption_material().is_none()
                                    || applying_secret(),
                                onclick: move |_| {
                                    let s = secret_input();
                                    if s.is_empty() {
                                        return;
                                    }
                                    let Some(material) = encryption_material() else {
                                        toast.warn(tr.encrypted_material_load_failed.to_string());
                                        return;
                                    };
                                    applying_secret.set(true);
                                    match build_stored_voter_key_from_encryption_material(&material, &s) {
                                        Ok(stored) => {
                                            let _ = save_stored_voter_key(&stored);
                                            save_session_vote_secret(&s);
                                            client_secret.set(Some(s));
                                            secret_input.set(String::new());
                                            show_secret_modal.set(false);
                                        }
                                        Err(e) => {
                                            toast.warn(e);
                                        }
                                    }
                                    applying_secret.set(false);
                                },
                                if applying_secret() {
                                    {tr.encrypted_secret_applying}
                                } else {
                                    {tr.encrypted_secret_continue}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

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
