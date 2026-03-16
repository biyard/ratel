use super::*;

use crate::features::spaces::layout::use_space_layout_ui;
use crate::features::spaces::pages::actions::actions::poll::components::*;
use crate::features::spaces::pages::actions::actions::poll::controllers::*;
use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::space_common::types::space_page_actions_poll_key;
use std::collections::HashMap;

#[component]
pub fn PollContent(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
    can_respond: bool,
) -> Element {
    let tr: participant::i18n::PollParticipantTranslate = use_translate();
    let mut query = use_query_store();
    let mut question_index = use_signal(|| 0usize);
    let mut hide_once = use_signal(|| false);
    let layout_ui = use_space_layout_ui();
    let sidebar_visible = layout_ui.sidebar_visible;
    let key = space_page_actions_poll_key(&space_id(), &poll_id());

    let poll_loader = use_query(&key, { move || get_poll(space_id(), poll_id()) })?;

    let poll = poll_loader.read();

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
            if poll.questions.len() == 0 {
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
    let can_submit = can_respond && is_in_progress && !has_response;
    let can_update = can_respond && is_in_progress && has_response && poll.response_editable;
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

    use_effect(move || {
        if hide_once() {
            return;
        }
        hide_once.set(true);
        let mut sidebar_visible = sidebar_visible;
        sidebar_visible.set(false);
    });

    use_drop(move || {
        let mut sidebar_visible = sidebar_visible;
        sidebar_visible.set(true);
    });

    let on_submit = {
        let questions = poll.questions.clone();
        move |_| {
            let questions = questions.clone();
            let mut query = query;

            spawn(async move {
                let answers_map = answers.read().clone();
                let payload: Vec<Answer> = (0..questions.len())
                    .map(|i| answers_map.get(&i).cloned().unwrap_or_default())
                    .collect();

                let req = RespondPollRequest { answers: payload };

                if respond_poll(space_id(), poll_id(), req).await.is_ok() {
                    let keys = space_page_actions_poll_key(&space_id(), &poll_id());
                    query.invalidate(&keys);
                }
            });
        }
    };

    rsx! {
        div { class: "flex min-h-0 w-full flex-1 flex-col gap-4",
            div { class: "mx-auto flex w-full max-w-desktop flex-1 flex-col gap-4 overflow-y-auto pb-6",
                if poll.status == PollStatus::Finish {
                    div { class: "rounded-lg bg-neutral-800 p-3 text-sm text-neutral-400",
                        {tr.poll_ended}
                    }
                }
                if poll.status == PollStatus::NotStarted {
                    div { class: "rounded-lg bg-neutral-800 p-3 text-sm text-neutral-400",
                        {tr.poll_not_started}
                    }
                }

                if total == 0 {
                    div { class: "flex items-center justify-center py-10 text-neutral-500",
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
                                div { class: "mb-5 flex items-center justify-end text-[16px] font-normal text-text-primary",
                                    "{tr.question_label}: {idx + 1}/{total}"
                                }
                                div { class: "w-full [&_[data-question-title-wrap]]:mb-5 [&_[data-question-title-wrap]>div]:justify-center [&_[data-question-title]]:text-center [&_[data-question-title]]:text-[21px] [&_[data-question-desc]]:text-center",
                                    QuestionViewer {
                                        index: idx,
                                        total,
                                        question: question.clone(),
                                        answer: current_answer,
                                        disabled: !can_respond || !is_in_progress || (!can_submit && !can_update),
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

            div { class: "-mx-5 -mb-5 max-tablet:-mx-3 max-tablet:-mb-3 max-mobile:-mx-2 max-mobile:-mb-2 flex items-center justify-between gap-3 border-t border-card-border bg-card-bg px-5 py-3",
                div {}
                div { class: "flex items-center gap-3",
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
                }
            }
        }
    }
}
