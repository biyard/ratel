use super::*;

use crate::features::spaces::pages::actions::actions::poll::components::*;
use crate::features::spaces::pages::actions::actions::poll::controllers::*;
use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::actions::components::FullActionLayover;
use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
use crate::features::spaces::space_common::providers::use_space_context;
use std::collections::HashMap;

#[component]
pub fn PollContent(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
    can_respond: bool,
) -> Element {
    let tr: participant::i18n::PollParticipantTranslate = use_translate();
    let mut space_ctx = use_space_context();
    let mut popup = use_popup();
    let mut toast = use_toast();
    let mut question_index = use_signal(|| 0usize);

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

                    let req = RespondPollRequest { answers: payload };

                    match respond_poll(space_id(), poll_id(), req).await {
                        Ok(_) => {
                            poll_loader.restart();
                            space_ctx.ranking.restart();
                            space_ctx.my_score.restart();
                            toast.info(tr.submit_success);
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
                    PollSubmitConfirm {
                        cancel_label: tr.submit_confirm_cancel,
                        confirm_label: tr.submit_confirm_action,
                        on_cancel,
                        on_confirm,
                    }
                })
                .with_title(tr.submit_confirm_title)
                .with_description(tr.submit_confirm_description);
        } else {
            build_submit_response()();
        }
    };

    rsx! {
        FullActionLayover {
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
                        {tr.btn_back}
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
                        {tr.btn_next}
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
                            {tr.btn_update}
                        } else {
                            {tr.btn_submit}
                        }
                    }
                }
            },
            div { class: "w-full",
                if poll.status == PollStatus::Finish {
                    div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                        {tr.poll_ended}
                    }
                }
                if poll.status == PollStatus::NotStarted {
                    div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                        {tr.poll_not_started}
                    }
                }

                if is_in_progress && !can_execute_action {
                    div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                        {tr.no_permission}
                    }
                }

                if is_in_progress && can_execute_action && has_response && !poll.response_editable
                    && can_respond
                {
                    div { class: "p-3 text-sm rounded-lg bg-banner-bg text-banner-text",
                        {tr.already_responded}
                    }
                }

                if total == 0 {
                    div { class: "flex justify-center items-center py-10 text-neutral-500",
                        {tr.no_questions}
                    }
                }

                if total > 0 {
                    {
                        let idx = question_index().min(total.saturating_sub(1));
                        let question = poll.questions[idx].clone();
                        let current_answer = answers.read().get(&idx).cloned();
                        let can_next = idx + 1 < total;
                        rsx! {
                            div { key: "poll-read-question-{idx}", class: "w-full",
                                div { class: "flex justify-end items-center mb-5 font-normal text-[16px] text-text-primary",
                                    "{tr.question_label}: {idx + 1}/{total}"
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

#[component]
fn PollSubmitConfirm(
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
