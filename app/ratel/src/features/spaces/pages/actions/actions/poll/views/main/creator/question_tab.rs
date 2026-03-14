use super::*;

#[component]
pub fn QuestionTab() -> Element {
    let tr: QuestionTabTranslate = use_translate();
    let mut toast = use_toast();

    let ctx = use_space_poll_context();
    let mut poll = ctx.poll;
    let space_id = ctx.space_id;
    let poll_id = ctx.poll_id;

    let mut questions = use_signal(|| poll().questions);
    let mut title = use_signal(|| poll().title);
    let can_edit = poll().user_response_count == 0;

    let save_title = move || async move {
        if !can_edit {
            return;
        }
        let req = UpdatePollRequest::Title { title: title() };
        if let Err(e) = update_poll(space_id(), poll_id(), req).await {
            toast.error(e);
        } else {
            poll.with_mut(|poll| poll.title = title());
        }
    };

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            // Title
            div { class: "flex flex-col gap-1",
                Label { html_for: "title", {tr.title_label} }
                if can_edit {
                    Input {
                        id: "title",
                        name: "title",
                        placeholder: tr.title_placeholder,
                        value: title,
                        oninput: move |e: FormEvent| title.set(e.value()),
                        onblur: move |_| async move {
                            save_title().await;
                        },
                        onconfirm: move |_| async move {
                            save_title().await;
                        },
                        oncancel: move |_| {
                            title.set(poll().title);
                        },
                    }
                } else {
                    div { class: "rounded-lg border border-input-box-border bg-input-box-bg px-4 py-3 text-text-primary",
                        "{poll().title}"
                    }
                }
            }

            // Questions area
            if can_edit {
                SurveyEditor {
                    questions,
                    on_save: move |questions: Vec<Question>| async move {
                        let req = UpdatePollRequest::Question {
                            questions: questions.clone(),
                        };
                        if let Err(e) = update_poll(space_id(), poll_id(), req).await {
                            toast.error(e);
                        } else {
                            poll.with_mut(move |poll| poll.questions = questions);
                        }
                    },
                }
            } else {
                div { class: "flex w-full flex-col gap-4",
                    for (idx, question) in poll().questions.iter().enumerate() {
                        div { class: "rounded-lg border border-neutral-700 bg-neutral-900 p-4",
                            div { class: "mb-2 text-xs text-neutral-500", "Question {idx + 1}" }
                            QuestionViewer {
                                index: idx,
                                question: question.clone(),
                                answer: None,
                                disabled: true,
                                on_change: move |_| {},
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
