use std::collections::HashMap;

use crate::common::components::{Button, ButtonShape, ButtonStyle};
use crate::common::utils::time::time_ago;
use crate::features::spaces::pages::actions::actions::poll::components::{
    has_answer_for_question, should_auto_next, QuestionViewer,
};
use crate::features::spaces::pages::actions::actions::poll::controllers::*;
use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::actions::components::FullActionLayover;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use crate::features::spaces::space_common::providers::use_space_context;

#[derive(Clone, Copy, PartialEq, Eq)]
enum PollReadStep {
    Overview,
    Poll,
}

translate! {
    PollReadTranslate;

    btn_next: { en: "Next", ko: "다음" },
    btn_back: { en: "Back", ko: "뒤로" },
    btn_cancel: { en: "Cancel", ko: "취소" },
    btn_submit: { en: "Submit", ko: "제출" },
    btn_update: { en: "Update", ko: "수정" },
    poll_ended: {
        en: "This poll has ended.",
        ko: "이 투표가 종료되었습니다.",
    },
    poll_not_started: {
        en: "This poll has not started yet.",
        ko: "이 투표는 아직 시작되지 않았습니다.",
    },
    no_permission: {
        en: "You do not have permission to participate in this poll.",
        ko: "이 투표에 참여할 권한이 없습니다.",
    },
    already_responded: {
        en: "You have already participated in this poll.",
        ko: "이미 이 투표에 참여했습니다.",
    },
    question_label: { en: "Question", ko: "질문" },
    no_questions: {
        en: "No questions yet.",
        ko: "아직 질문이 없습니다.",
    },
    submit_success: {
        en: "Response submitted successfully.",
        ko: "응답이 성공적으로 제출되었습니다.",
    },
    submit_confirm_title: {
        en: "Submit response",
        ko: "응답 제출",
    },
    submit_confirm_description: {
        en: "Once submitted, this response cannot be edited. Do you want to submit?",
        ko: "한번 제출한 응답은 수정이 불가능합니다. 제출하시겠습니까?",
    },
    submit_confirm_cancel: { en: "Cancel", ko: "취소" },
    submit_confirm_action: { en: "Submit", ko: "제출" },
    encrypted_verify_title: {
        en: "Encrypted vote verification",
        ko: "암호화 투표 검증",
    },
    encrypted_verify_description: {
        en: "Enter a local secret to encrypt your poll-specific voter key in this browser and decrypt your on-chain ballot locally.",
        ko: "로컬 시크릿을 입력하면 이 브라우저에 투표별 복호화 키를 암호화해 저장하고 온체인 투표를 로컬에서 복호화합니다.",
    },
    encrypted_verify_session_description: {
        en: "A local secret is remembered for this browser session and will be reused for other encrypted polls.",
        ko: "이 브라우저 세션 동안 로컬 시크릿을 기억하고 다른 암호화 Poll에도 재사용합니다.",
    },
    encrypted_verify_secret_placeholder: {
        en: "Local secret",
        ko: "로컬 시크릿",
    },
    encrypted_verify_session_active: {
        en: "Using the local secret saved for this browser session.",
        ko: "이 브라우저 세션에 저장된 로컬 시크릿을 사용 중입니다.",
    },
    encrypted_verify_change_secret: {
        en: "Change secret",
        ko: "시크릿 변경",
    },
    encrypted_verify_button: {
        en: "Save key and verify",
        ko: "키 저장 및 검증",
    },
    encrypted_verify_success: {
        en: "Vote verified locally.",
        ko: "투표를 로컬에서 검증했습니다.",
    },
    encrypted_verify_error: {
        en: "Local vote verification failed.",
        ko: "로컬 투표 검증에 실패했습니다.",
    },
    encrypted_verify_decrypted: {
        en: "Decrypted answer",
        ko: "복호화된 답변",
    },
    encrypted_secret_modal_title: {
        en: "Set up encrypted voting",
        ko: "암호화 투표 설정",
    },
    encrypted_secret_modal_description: {
        en: "This poll uses end-to-end encryption. Set a local secret used to derive your voter key in this browser. The same secret will let you verify your ballot later. The secret never leaves your device.",
        ko: "이 투표는 종단 간 암호화를 사용합니다. 이 브라우저에서 투표자 키를 파생하는 데 쓸 로컬 시크릿을 설정하세요. 동일한 시크릿으로 나중에 투표를 검증할 수 있습니다. 시크릿은 기기를 떠나지 않습니다.",
    },
    encrypted_secret_modal_placeholder: {
        en: "Local secret",
        ko: "로컬 시크릿",
    },
    encrypted_secret_modal_continue: {
        en: "Continue",
        ko: "계속",
    },
    encrypted_secret_modal_cancel: {
        en: "Cancel",
        ko: "취소",
    },
    encrypted_secret_required: {
        en: "Encrypted polls require a local secret before submitting.",
        ko: "암호화 투표는 제출 전에 로컬 시크릿 설정이 필요합니다.",
    },
    encrypted_material_load_failed: {
        en: "Failed to load encryption material.",
        ko: "암호화 자료 로드에 실패했습니다.",
    },
}

#[component]
pub fn PollReadPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
    can_respond: bool,
) -> Element {
    let i18n: PollReadTranslate = use_translate();
    let mut space_ctx = use_space_context();
    let mut popup = use_popup();
    let mut toast = use_toast();
    let mut step = use_signal(|| PollReadStep::Overview);
    let mut question_index = use_signal(|| 0usize);
    let nav = navigator();

    let mut poll_loader = use_loader(move || get_poll(space_id(), poll_id()))?;
    let poll = poll_loader.read().clone();
    let space = use_space().read().clone();
    let role = use_space_role()();
    let can_execute_action = crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        poll.space_action.prerequisite,
        space.status,
        poll.space_action.status.as_ref(),
        true,
        space.join_anytime,
    );

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
                .all(|(idx, question)| has_answer_for_question(question, answers_read.get(&idx)))
        }
    });

    let is_in_progress = poll.status == PollStatus::InProgress;
    let has_response = poll.my_response.is_some();
    let can_submit = can_respond && can_execute_action && is_in_progress && !has_response;
    let can_update = can_respond
        && can_execute_action
        && is_in_progress
        && has_response
        && poll.response_editable;
    let total = poll.questions.len();
    let current_idx = question_index().min(total.saturating_sub(1));
    let current_question = poll.questions.get(current_idx).cloned();
    let current_answer = answers.read().get(&current_idx).cloned();
    let has_current_answer = current_question
        .as_ref()
        .map(|q| has_answer_for_question(q, current_answer.as_ref()))
        .unwrap_or(false);
    let is_first_question = total == 0 || current_idx == 0;
    let is_last_question = total == 0 || current_idx + 1 >= total;
    let poll_next_disabled = can_submit && !has_current_answer;
    let show_submit_button = can_respond && (can_submit || can_update);

    let encryption_enabled = poll.encrypted_upload_enabled;
    let mut encryption_material = use_signal(|| None::<VoteEncryptionMaterialResponse>);
    let client_secret = use_signal(|| None::<String>);

    let build_submit_response = {
        let questions = poll.questions.clone();
        move || {
            let questions = questions.clone();
            move || {
                let questions = questions.clone();
                spawn(async move {
                    let answers_map = answers.read().clone();
                    let payload: Vec<Answer> = (0..questions.len())
                        .map(|i| answers_map.get(&i).cloned().unwrap_or_default())
                        .collect();

                    let mut req = RespondPollRequest {
                        answers: payload.clone(),
                        client_ciphertext_json: None,
                        client_voter_tag: None,
                    };

                    if encryption_enabled {
                        let material = encryption_material();
                        let secret = client_secret();
                        let now = crate::common::utils::time::get_now_timestamp_millis();
                        match (material, secret) {
                            (Some(m), Some(_)) => {
                                match encrypt_answers_for_canister(&m, &payload, now) {
                                    Ok(enc) => {
                                        req.client_ciphertext_json = Some(enc.ciphertext_json);
                                        req.client_voter_tag = Some(enc.voter_tag);
                                    }
                                    Err(e) => {
                                        toast.warn(e);
                                        return;
                                    }
                                }
                            }
                            _ => {
                                toast.warn(i18n.encrypted_secret_required.to_string());
                                return;
                            }
                        }
                    }

                    match respond_poll(space_id(), poll_id(), req).await {
                        Ok(_) => {
                            poll_loader.restart();
                            space_ctx.ranking.restart();
                            space_ctx.my_score.restart();
                            toast.info(i18n.submit_success);
                            nav.replace(crate::Route::SpaceActionsPage {
                                space_id: space_id(),
                            });
                        }
                        Err(err) => {
                            toast.error(err);
                        }
                    }
                });
            }
        }
    };

    let needs_secret = encryption_enabled && can_submit;

    use_effect({
        let i18n = i18n.clone();
        move || {
            if !needs_secret {
                return;
            }
            let i18n_eff = i18n.clone();
            spawn(async move {
                if encryption_material().is_none() {
                    match get_encryption_material(space_id(), poll_id()).await {
                        Ok(m) => encryption_material.set(Some(m)),
                        Err(_) => {
                            toast.warn(i18n_eff.encrypted_material_load_failed.to_string());
                            return;
                        }
                    }
                }

                if client_secret().is_none() {
                    if let Some(s) = load_session_vote_secret().await {
                        if !s.is_empty() {
                            finalize_secret_setup(
                                encryption_material,
                                client_secret,
                                s,
                                toast,
                            );
                            return;
                        }
                    }

                    open_secret_setup_modal(
                        popup,
                        i18n_eff,
                        encryption_material,
                        client_secret,
                        toast,
                    );
                }
            });
        }
    });

    let on_submit = move |_| {
        if can_submit && !poll.response_editable {
            let mut popup = popup;
            let confirm_submit_response = build_submit_response();
            let on_cancel = move |_| {
                popup.close();
            };
            let on_confirm = move |_| {
                popup.close();
                confirm_submit_response();
            };
            popup
                .open(rsx! {
                    PollReadSubmitConfirm {
                        cancel_label: i18n.submit_confirm_cancel,
                        confirm_label: i18n.submit_confirm_action,
                        on_cancel,
                        on_confirm,
                    }
                })
                .with_title(i18n.submit_confirm_title)
                .with_description(i18n.submit_confirm_description);
        } else {
            build_submit_response()();
        }
    };

    let on_cancel = move |_| {
        nav.push(crate::Route::SpaceActionsPage {
            space_id: space_id(),
        });
    };

    rsx! {
        if step() == PollReadStep::Overview {
            FullActionLayover {
                "data-testid": "poll-read-overview",
                content_class: "gap-6".to_string(),
                bottom_right: rsx! {
                    Button {
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "min-w-[120px]",
                        onclick: on_cancel,
                        {i18n.btn_cancel}
                    }
                    Button {
                        style: ButtonStyle::Primary,
                        shape: ButtonShape::Square,
                        class: "min-w-[120px]",
                        disabled: poll.questions.is_empty(),
                        "data-testid": "poll-read-next",
                        onclick: move |_| {
                            question_index.set(0);
                            step.set(PollReadStep::Poll);
                        },
                        {i18n.btn_next}
                    }
                },
                div { class: "w-full",
                    div { class: "font-bold text-[28px]/[34px] text-text-primary", "{poll.title}" }

                    div { class: "flex justify-end items-center py-4 border-y border-card-border",
                        div { class: "font-light shrink-0 text-[14px] text-text-primary",
                            "{time_ago(poll.created_at)}"
                        }
                    }

                    if !poll.description.is_empty() {
                        div {
                            class: "text-[15px]/[24px] tracking-[0.5px] text-foreground-muted",
                            dangerous_inner_html: poll.description.clone(),
                        }
                    }
                }
            }
        } else {
            FullActionLayover {
                "data-testid": "poll-read-poll",
                bottom_right: rsx! {
                    if !is_first_question && total > 0 {
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "min-w-[120px]",
                            onclick: move |_| {
                                let current = question_index();
                                if current > 0 {
                                    question_index.set(current - 1);
                                }
                            },
                            {i18n.btn_back}
                        }
                    }



                    if !is_last_question && total > 0 {
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "min-w-[120px]",
                            disabled: poll_next_disabled,
                            onclick: move |_| {
                                let current = question_index();
                                if current + 1 < total {
                                    question_index.set(current + 1);
                                }
                            },
                            {i18n.btn_next}
                        }
                    }

                    if is_last_question && show_submit_button && total > 0 {
                        Button {
                            style: ButtonStyle::Primary,
                            shape: ButtonShape::Square,
                            class: "min-w-[120px]",
                            disabled: !all_answered(),
                            onclick: on_submit,
                            if can_update {
                                {i18n.btn_update}
                            } else {
                                {i18n.btn_submit}
                            }
                        }
                    }
                },
                div { class: "w-full",
                    if poll.status == PollStatus::Finish {
                        div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                            {i18n.poll_ended}
                        }
                    }
                    if poll.status == PollStatus::NotStarted {
                        div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                            {i18n.poll_not_started}
                        }
                    }

                    if is_in_progress && !can_execute_action {
                        div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                            {i18n.no_permission}
                        }
                    }

                    if is_in_progress && can_execute_action && has_response && !poll.response_editable
                        && can_respond
                    {
                        div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                            {i18n.already_responded}
                        }
                    }

                    if poll.encrypted_upload_enabled && has_response && can_respond {
                        EncryptedVoteVerificationPanel { space_id, poll_id }
                    }

                    if total == 0 {
                        div { class: "flex justify-center items-center py-10 text-foreground-muted",
                            {i18n.no_questions}
                        }
                    } else {
                        {
                            let idx = question_index().min(total.saturating_sub(1));
                            let question = poll.questions[idx].clone();
                            let current_answer = answers.read().get(&idx).cloned();
                            let can_next = idx + 1 < total;
                            rsx! {
                                div { key: "poll-read-question-{idx}", class: "w-full",
                                    div { class: "flex justify-end items-center mb-5 font-normal text-[16px] text-text-primary",
                                        "{i18n.question_label}: {idx + 1}/{total}"
                                    }
                                    div { class: "w-full [&_[data-question-title-wrap]]:mb-5 [&_[data-question-title-wrap]>div]:justify-center [&_[data-question-title]]:text-center [&_[data-question-title]]:text-[21px] [&_[data-question-desc]]:text-center",
                                        QuestionViewer {
                                            index: idx,
                                            total,
                                            question: question.clone(),
                                            answer: current_answer,
                                            disabled: !can_respond || !can_execute_action || !is_in_progress
                                                || (!can_submit && !can_update),
                                            enable_other_option: true,
                                            on_change: move |ans: Answer| {
                                                answers.write().insert(idx, ans.clone());
                                                if can_submit && can_next && should_auto_next(&question, &ans) {
                                                    question_index.set(idx + 1);
                                                }
                                            },
                                        }
                                    }
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
fn EncryptedVoteVerificationPanel(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    let i18n: PollReadTranslate = use_translate();
    let mut toast = use_toast();
    let mut secret = use_signal(String::new);
    let mut secret_from_session = use_signal(|| false);
    let mut show_secret_input = use_signal(|| true);
    let mut verifying = use_signal(|| false);
    let mut decrypted = use_signal(|| None::<String>);
    let mut error = use_signal(|| None::<String>);

    let load_session_secret = move |_| {
        spawn(async move {
            if let Some(saved_secret) = load_session_vote_secret().await {
                if !saved_secret.is_empty() && secret().is_empty() {
                    secret.set(saved_secret);
                    secret_from_session.set(true);
                    show_secret_input.set(false);
                }
            }
        });
    };

    let on_verify = move |_| {
        let secret_value = secret();
        if secret_value.is_empty() {
            error.set(Some(i18n.encrypted_verify_secret_placeholder.to_string()));
            return;
        }

        verifying.set(true);
        error.set(None);
        decrypted.set(None);

        spawn(async move {
            let result = async {
                let material = get_vote_verification_material(space_id(), poll_id())
                    .await
                    .map_err(|e| e.to_string())?;
                let stored_key =
                    build_stored_voter_key(&material, &secret_value).map_err(|e| e.to_string())?;
                save_stored_voter_key(&stored_key).map_err(|e| e.to_string())?;
                let verified = verify_client_vote_material(&material, &stored_key, &secret_value)
                    .map_err(|e| e.to_string())?;
                save_session_vote_secret(&secret_value);
                Ok::<_, String>(verified)
            }
            .await;

            verifying.set(false);
            match result {
                Ok(verified) => {
                    secret_from_session.set(true);
                    show_secret_input.set(false);
                    decrypted.set(Some(verified.decrypted_choice));
                    toast.info(i18n.encrypted_verify_success);
                }
                Err(err) => {
                    let message = err.to_string();
                    error.set(Some(message.clone()));
                    toast.warn(message);
                }
            }
        });
    };

    rsx! {
        div {
            class: "flex flex-col gap-3 p-4 my-4 rounded-xl border border-card-border bg-card-bg",
            onmounted: load_session_secret,
            div { class: "flex flex-col gap-1",
                div { class: "font-semibold text-[15px] text-text-primary",
                    {i18n.encrypted_verify_title}
                }
                div { class: "text-[13px]/[20px] text-foreground-muted",
                    {i18n.encrypted_verify_description}
                }
                div { class: "text-[13px]/[20px] text-foreground-muted",
                    {i18n.encrypted_verify_session_description}
                }
            }
            if secret_from_session() && !show_secret_input() {
                div { class: "flex flex-col gap-2 sm:flex-row sm:items-center",
                    div { class: "flex-1 py-2 px-3 text-sm rounded-lg border border-card-border bg-background text-foreground-muted",
                        {i18n.encrypted_verify_session_active}
                    }
                    Button {
                        style: ButtonStyle::Secondary,
                        shape: ButtonShape::Square,
                        class: "min-w-[140px]",
                        disabled: verifying(),
                        onclick: move |_| {
                            secret.set(String::new());
                            secret_from_session.set(false);
                            show_secret_input.set(true);
                        },
                        {i18n.encrypted_verify_change_secret}
                    }
                }
            } else {
                input {
                    class: "py-2 px-3 text-sm rounded-lg border border-card-border bg-background text-text-primary",
                    r#type: "password",
                    value: "{secret()}",
                    placeholder: "{i18n.encrypted_verify_secret_placeholder}",
                    oninput: move |evt| {
                        secret.set(evt.value());
                        secret_from_session.set(false);
                    },
                }
            }
            div { class: "flex justify-end",
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Square,
                    class: "min-w-[160px]",
                    disabled: verifying(),
                    onclick: on_verify,
                    {i18n.encrypted_verify_button}
                }
            }
            if let Some(message) = error() {
                div { class: "text-red-500 text-[13px]/[20px]",
                    "{i18n.encrypted_verify_error}: {message}"
                }
            }
            if let Some(answer) = decrypted() {
                div { class: "flex flex-col gap-2",
                    div { class: "font-semibold text-[13px] text-text-primary",
                        {i18n.encrypted_verify_decrypted}
                    }
                    pre { class: "overflow-auto p-3 max-h-48 text-xs whitespace-pre-wrap rounded-lg bg-background text-foreground-muted",
                        "{answer}"
                    }
                }
            }
        }
    }
}

#[component]
fn PollReadSubmitConfirm(
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
                "data-testid": "poll-read-confirm-submit",
                style: ButtonStyle::Primary,
                shape: ButtonShape::Square,
                class: "min-w-[120px]",
                onclick: move |e| on_confirm.call(e),
                {confirm_label}
            }
        }
    }
}

fn finalize_secret_setup(
    encryption_material: Signal<Option<VoteEncryptionMaterialResponse>>,
    mut client_secret: Signal<Option<String>>,
    secret: String,
    mut toast: crate::common::providers::ToastService,
) {
    let Some(material) = encryption_material() else {
        return;
    };
    match build_stored_voter_key_from_encryption_material(&material, &secret) {
        Ok(stored) => {
            let _ = save_stored_voter_key(&stored);
            save_session_vote_secret(&secret);
            client_secret.set(Some(secret));
        }
        Err(e) => {
            toast.warn(e);
        }
    }
}

fn open_secret_setup_modal(
    mut popup: crate::common::components::PopupService,
    i18n: PollReadTranslate,
    encryption_material: Signal<Option<VoteEncryptionMaterialResponse>>,
    client_secret: Signal<Option<String>>,
    toast: crate::common::providers::ToastService,
) {
    let on_continue = move |secret: String| {
        if secret.is_empty() {
            return;
        }
        finalize_secret_setup(encryption_material, client_secret, secret, toast);
        popup.close();
    };
    let on_cancel = move |_| {
        popup.close();
    };
    popup
        .open(rsx! {
            EncryptedSecretSetupModal {
                placeholder: i18n.encrypted_secret_modal_placeholder.to_string(),
                continue_label: i18n.encrypted_secret_modal_continue.to_string(),
                cancel_label: i18n.encrypted_secret_modal_cancel.to_string(),
                on_continue,
                on_cancel,
            }
        })
        .with_title(i18n.encrypted_secret_modal_title)
        .with_description(i18n.encrypted_secret_modal_description);
}

#[component]
fn EncryptedSecretSetupModal(
    placeholder: String,
    continue_label: String,
    cancel_label: String,
    on_continue: EventHandler<String>,
    on_cancel: EventHandler<MouseEvent>,
) -> Element {
    let mut secret = use_signal(String::new);
    rsx! {
        div { class: "flex flex-col gap-3 w-full",
            input {
                class: "py-2 px-3 text-sm rounded-lg border border-card-border bg-background text-text-primary",
                r#type: "password",
                value: "{secret()}",
                placeholder: "{placeholder}",
                "data-testid": "poll-encrypted-secret-input",
                oninput: move |evt| secret.set(evt.value()),
            }
            div { class: "flex gap-3 justify-end w-full",
                Button {
                    style: ButtonStyle::Outline,
                    shape: ButtonShape::Square,
                    class: "min-w-[120px]",
                    onclick: move |e| on_cancel.call(e),
                    {cancel_label}
                }
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Square,
                    class: "min-w-[120px]",
                    "data-testid": "poll-encrypted-secret-continue",
                    disabled: secret().is_empty(),
                    onclick: move |_| on_continue.call(secret()),
                    {continue_label}
                }
            }
        }
    }
}
