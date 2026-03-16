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
    let nav = navigator();
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

    let on_cancel = move |_| {
        let mut sidebar_visible = sidebar_visible;
        sidebar_visible.set(true);
        nav.push(format!("/spaces/{}/actions", space_id()));
    };

    rsx! {
        div { class: "flex min-h-0 w-full flex-1 flex-col gap-4",
            div { class: "flex flex-1 flex-col gap-4 overflow-y-auto pb-6",
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
                        "No questions yet."
                    }
                }

                if total > 0 {
                    {
                        let idx = question_index().min(total.saturating_sub(1));
                        let question = poll.questions[idx].clone();
                        let current_answer = answers.read().get(&idx).cloned();
                        let can_next = idx + 1 < total;
                        let has_current_answer =
                            has_answer_for_question(
                            &question,
                            current_answer.as_ref(),
                        );
                        let next_disabled = idx + 1 >= total || (can_respond && !has_current_answer);
                        rsx! {
                            div { class: "rounded-lg border border-neutral-700 bg-neutral-900 p-4",
                                div { class: "mb-2 text-xs text-neutral-500", "{idx + 1} / {total}" }
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
                                    on_prev: move |_| {
                                        if idx > 0 {
                                            question_index.set(idx - 1);
                                        }
                                    },
                                    on_next: move |_| {
                                        if idx + 1 < total && (!can_respond || has_current_answer) {
                                            question_index.set(idx + 1);
                                        }
                                    },
                                    next_disabled,
                                }
                            }
                        }
                    }
                }
            }

            div { class: "-mx-5 max-tablet:-mx-3 max-mobile:-mx-2 flex items-center justify-between gap-3 border-t border-neutral-700/80 bg-[#171a20] px-5 py-3 light:border-input-box-border light:bg-background",
                div {}
                div { class: "flex items-center gap-3",
                    Button {
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "min-w-[120px]",
                        onclick: on_cancel,
                        {tr.btn_cancel}
                    }
                    if show_submit_button {
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
