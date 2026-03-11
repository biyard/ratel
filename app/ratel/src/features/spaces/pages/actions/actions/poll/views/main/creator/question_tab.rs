use super::*;

#[component]
pub fn QuestionTab() -> Element {
    let tr: QuestionTabTranslate = use_translate();
    let nav = use_navigator();
    let mut toast = use_toast();

    let ctx = use_space_poll_context();
    let mut poll = ctx.poll;
    let space_id = ctx.space_id;
    let poll_id = ctx.poll_id;

    let mut editing = use_signal(|| false);
    let mut questions = use_signal(|| poll().questions);
    let mut title = use_signal(|| poll().title);

    let on_title_save = move |_| async move {
        let req = UpdatePollRequest::Title { title: title() };
        if let Err(e) = update_poll(space_id(), poll_id(), req).await {
            toast.error(e);
        } else {
            let keys = space_page_actions_poll_key(&space_id(), &poll_id());
            invalidate_query(&keys);
        }
    };

    let on_time_change = move |(start, end): (i64, i64)| async move {
        let req = UpdatePollRequest::Time {
            started_at: start,
            ended_at: end,
        };

        if let Err(e) = update_poll(space_id(), poll_id(), req).await {
            toast.error(e);
        } else {
            poll.restart();
        }
    };

    let on_response_editable_change = move |_| async move {
        let req = UpdatePollRequest::ResponseEditable {
            response_editable: !poll().response_editable,
        };
        if let Err(e) = update_poll(space_id(), poll_id(), req).await {
            toast.error(e);
        } else {
            poll.restart();
        }
    };

    let on_save = move |_| async move {
        let req = UpdatePollRequest::Question {
            questions: questions(),
        };
        if let Err(e) = update_poll(space_id(), poll_id(), req).await {
            toast.error(e);
        } else {
            poll.restart();
            editing.set(false);
        }
    };

    let on_discard = move |_| {
        questions.set(poll().questions);
        editing.set(false);
    };

    let on_edit = move |_| {
        editing.set(true);
    };

    let poll = poll();
    let can_edit = poll.user_response_count == 0;

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            // Title
            div { class: "flex flex-col gap-1",
                label { class: "text-sm font-medium text-neutral-400 light:text-neutral-600",
                    "{tr.title_label}"
                }
                input {
                    class: "py-3 px-4 w-full text-base text-white rounded-lg border bg-neutral-800 light:bg-neutral-100 border-neutral-700 light:border-neutral-300 light:text-neutral-900 placeholder-neutral-500",
                    placeholder: "{tr.title_placeholder}",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                    onblur: on_title_save,
                }
            }

            // Time range setting
            TimeRangeSetting {
                started_at: poll.started_at,
                ended_at: poll.ended_at,
                on_change: on_time_change,
            }

            // Response editable checkbox
            if can_edit {
                div { class: "flex gap-3 items-center",
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
                div { class: "flex gap-2 justify-end",
                    if editing() {
                        button {
                            class: "py-2 px-4 text-sm text-white bg-blue-600 rounded-lg hover:bg-blue-500",
                            onclick: on_save,
                            "{tr.btn_save}"
                        }
                        button {
                            class: "py-2 px-4 text-sm rounded-lg border border-neutral-600 text-neutral-300 hover:bg-neutral-800",
                            onclick: on_discard,
                            "{tr.btn_discard}"
                        }
                    } else {
                        button {
                            class: "py-2 px-4 text-sm rounded-lg border border-neutral-600 text-neutral-300 hover:bg-neutral-800",
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

translate! {
    QuestionTabTranslate;

    btn_edit: {
        en: "Edit",
        ko: "편집",
    },
    btn_save: {
        en: "Save",
        ko: "저장",
    },
    btn_discard: {
        en: "Discard",
        ko: "취소",
    },
    btn_add_question: {
        en: "Add Question",
        ko: "질문 추가",
    },
    btn_back: {
        en: "Back",
        ko: "뒤로",
    },
    response_editable_label: {
        en: "Allow response editing",
        ko: "응답 수정 허용",
    },
    response_editable_description: {
        en: "Participants can modify their answers after submitting.",
        ko: "참가자가 제출 후 답변을 수정할 수 있습니다.",
    },
    no_questions: {
        en: "No questions added yet.",
        ko: "아직 질문이 없습니다.",
    },
    title_label: {
        en: "Title",
        ko: "제목",
    },
    title_placeholder: {
        en: "Enter poll title...",
        ko: "투표 제목을 입력하세요...",
    },
}
