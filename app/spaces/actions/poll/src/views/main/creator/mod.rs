use crate::components::*;
use crate::controllers::*;
use crate::*;
mod i18n;
use i18n::PollEditorTranslate;
use space_common::types::space_page_actions_poll_key;

#[component]
pub fn PollCreatorPage(space_id: SpacePartition, poll_id: SpacePollEntityType) -> Element {
    let tr: PollEditorTranslate = use_translate();
    let nav = navigator();
    let key = space_page_actions_poll_key(&space_id, &poll_id);
    let poll_loader = use_query(&key, {
        let space_id = space_id.clone();
        let poll_id = poll_id.clone();
        move || get_poll(space_id.clone(), poll_id.clone())
    })?;

    let poll = poll_loader.read().clone();

    let mut editing = use_signal(|| false);
    let mut questions = use_signal(|| poll.questions.clone());

    let can_edit = poll.user_response_count == 0;

    let on_back = move |_| {
        nav.go_back();
    };

    let on_time_change = {
        let space_id = space_id.clone();
        let poll_id = poll_id.clone();
        move |(start, end): (i64, i64)| {
            let space_id = space_id.clone();
            let poll_id = poll_id.clone();
            spawn(async move {
                let req = UpdatePollRequest::Time {
                    started_at: start,
                    ended_at: end,
                };
                if let Err(e) = update_poll(space_id.clone(), poll_id.clone(), req).await {
                    error!("Failed to update time range: {:?}", e);
                } else {
                    let keys = space_page_actions_poll_key(&space_id, &poll_id);
                    invalidate_query(&keys);
                }
            });
        }
    };

    let on_response_editable_change = {
        let space_id = space_id.clone();
        let poll_id = poll_id.clone();
        let current = poll.response_editable;
        move |_| {
            let space_id = space_id.clone();
            let poll_id = poll_id.clone();
            spawn(async move {
                let req = UpdatePollRequest::ResponseEditable {
                    response_editable: !current,
                };
                if let Err(e) = update_poll(space_id.clone(), poll_id.clone(), req).await {
                    error!("Failed to update response editable: {:?}", e);
                } else {
                    let keys = space_page_actions_poll_key(&space_id, &poll_id);
                    invalidate_query(&keys);
                }
            });
        }
    };

    let on_save = {
        let space_id = space_id.clone();
        let poll_id = poll_id.clone();
        move |_| {
            let qs = questions.read().clone();
            let space_id = space_id.clone();
            let poll_id = poll_id.clone();
            spawn(async move {
                let req = UpdatePollRequest::Question { questions: qs };
                if let Err(e) = update_poll(space_id.clone(), poll_id.clone(), req).await {
                    error!("Failed to save questions: {:?}", e);
                } else {
                    let keys = space_page_actions_poll_key(&space_id, &poll_id);
                    invalidate_query(&keys);
                    editing.set(false);
                }
            });
        }
    };

    let on_discard = {
        let poll_questions = poll.questions.clone();
        move |_| {
            questions.set(poll_questions.clone());
            editing.set(false);
        }
    };

    let on_edit = move |_| {
        editing.set(true);
    };

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            // Back button
            button {
                class: "flex items-center gap-1 text-sm text-neutral-400 hover:text-white transition-colors w-fit",
                onclick: on_back,
                "← {tr.btn_back}"
            }

            // Time range setting
            TimeRangeSetting {
                started_at: poll.started_at,
                ended_at: poll.ended_at,
                on_change: on_time_change,
            }

            // Response editable checkbox
            if can_edit {
                div { class: "flex items-center gap-3",
                    input {
                        r#type: "checkbox",
                        class: "w-4 h-4",
                        checked: poll.response_editable,
                        onchange: on_response_editable_change,
                    }
                    div { class: "flex flex-col gap-0.5",
                        label { class: "text-sm font-medium text-white cursor-pointer",
                            "{tr.response_editable_label}"
                        }
                        p { class: "text-xs text-neutral-400", "{tr.response_editable_description}" }
                    }
                }
            }

            // Edit / Save / Discard toolbar
            if can_edit {
                div { class: "flex justify-end gap-2",
                    if editing() {
                        button {
                            class: "px-4 py-2 rounded-lg bg-blue-600 text-white text-sm hover:bg-blue-500",
                            onclick: on_save,
                            "{tr.btn_save}"
                        }
                        button {
                            class: "px-4 py-2 rounded-lg border border-neutral-600 text-neutral-300 text-sm hover:bg-neutral-800",
                            onclick: on_discard,
                            "{tr.btn_discard}"
                        }
                    } else {
                        button {
                            class: "px-4 py-2 rounded-lg border border-neutral-600 text-neutral-300 text-sm hover:bg-neutral-800",
                            onclick: on_edit,
                            "{tr.btn_edit}"
                        }
                    }
                }
            }

            // Questions area
            if editing() {
                SurveyEditor { questions, on_save: move |_qs: Vec<Question>| {} }
            } else {
                if poll.questions.is_empty() {
                    div { class: "flex justify-center items-center py-10 text-neutral-500",
                        "{tr.no_questions}"
                    }
                }
                for (idx , question) in poll.questions.iter().enumerate() {
                    {
                        let question = question.clone();
                        rsx! {
                            div { class: "p-4 rounded-lg border border-neutral-700 bg-neutral-900",
                                QuestionViewer {
                                    index: idx,
                                    question,
                                    answer: None,
                                    disabled: true,
                                    on_change: move |_ans: Answer| {},
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
