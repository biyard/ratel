use std::collections::HashMap;

mod i18n;
use crate::components::*;
use crate::controllers::*;
use crate::*;
use i18n::PollParticipantTranslate;
use space_common::types::space_page_actions_poll_key;

#[component]
pub fn PollParticipantPage(space_id: SpacePartition, poll_id: SpacePollEntityType) -> Element {
    let tr: PollParticipantTranslate = use_translate();
    let nav = navigator();

    let poll_loader = use_query(&["polls", &space_id.to_string(), &poll_id.to_string()], {
        let space_id = space_id.clone();
        let poll_id = poll_id.clone();
        move || get_poll(space_id.clone(), poll_id.clone())
    })?;

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

    let is_in_progress = poll.status == PollStatus::InProgress;
    let has_response = poll.my_response.is_some();
    let can_submit = is_in_progress && !has_response;
    let can_update = is_in_progress && has_response && poll.response_editable;

    let total = poll.questions.len();

    let submit_handler = {
        let questions = poll.questions.clone();
        let space_id = space_id.clone();
        let poll_id = poll_id.clone();
        move |_: MouseEvent| {
            let space_id = space_id.clone();
            let poll_id = poll_id.clone();
            let questions = questions.clone();

            async move {
                let answers_map = answers.read().clone();
                let payload: Vec<Answer> = (0..questions.len())
                    .map(|i| answers_map.get(&i).cloned().unwrap_or_default())
                    .collect();

                let req = RespondPollRequest { answers: payload };

                match respond_poll(space_id.clone(), poll_id.clone(), req).await {
                    Ok(_) => {
                        let keys = space_page_actions_poll_key(&space_id, &poll_id);
                        invalidate_query(&keys);
                    }
                    _ => {}
                }
            }
        }
    };

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            // Back button
            button {
                class: "flex items-center gap-1 text-sm text-neutral-400 hover:text-white transition-colors w-fit",
                onclick: move |_| {
                    nav.go_back();
                },
                "← {tr.btn_back}"
            }

            // Time range
            TimeRangeDisplay { started_at: poll.started_at, ended_at: poll.ended_at }

            // Status message
            if poll.status == PollStatus::Finish {
                div { class: "p-3 rounded-lg bg-neutral-800 text-neutral-400 text-sm",
                    {tr.poll_ended}
                }
            }
            if poll.status == PollStatus::NotStarted {
                div { class: "p-3 rounded-lg bg-neutral-800 text-neutral-400 text-sm",
                    {tr.poll_not_started}
                }
            }

            // Questions
            if total == 0 {
                div { class: "flex justify-center items-center py-10 text-neutral-500",
                    "No questions yet."
                }
            }

            for (idx , question) in poll.questions.iter().enumerate() {
                {
                    let question = question.clone();
                    let current_answer = answers.read().get(&idx).cloned();
                    let display_idx = idx + 1;
                    rsx! {
                        div { class: "p-4 rounded-lg border border-neutral-700 bg-neutral-900",
                            div { class: "text-xs text-neutral-500 mb-2", "{display_idx} / {total}" }
                            QuestionViewer {
                                index: idx,
                                question,
                                answer: current_answer,
                                disabled: !is_in_progress || (!can_submit && !can_update),
                                on_change: move |ans: Answer| {
                                    answers.write().insert(idx, ans);
                                },
                            }
                        }
                    }
                }
            }

            // Submit / Update button
            if can_submit || can_update {
                button {
                    class: "w-full py-3 rounded-lg bg-blue-600 text-white font-medium hover:bg-blue-500 transition-colors",
                    onclick: submit_handler,
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
