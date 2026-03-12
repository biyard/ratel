use super::*;
use crate::features::spaces::space_common::types::space_page_actions_quiz_key;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OverviewStatus {
    Idle,
    Saving,
    Saved,
    Unsaved,
}

#[component]
pub fn OverviewTab(can_edit: bool) -> Element {
    let ctx = use_space_quiz_context();
    let tr: QuizCreatorTranslate = use_translate();
    let mut toast = use_toast();
    let initial_title = ctx.quiz.read().title.clone();
    let initial_description = ctx.quiz.read().description.clone();
    let mut title = use_signal(|| initial_title.clone());
    let mut description = use_signal(|| initial_description.clone());
    let mut last_saved = use_signal(|| (initial_title, initial_description));
    let mut status = use_signal(|| OverviewStatus::Idle);
    let mut save_version = use_signal(|| 0u64);
    let title_count = std::cmp::min(title().chars().count(), 50);
    let space_id = ctx.space_id;
    let quiz_id = ctx.quiz_id;

    use_effect(move || {
        if !can_edit {
            return;
        }

        let version = save_version();
        if version == 0 {
            return;
        }

        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(3)).await;

            if save_version() != version {
                return;
            }

            let current_title = title();
            let current_description = description();
            if (current_title.clone(), current_description.clone()) == last_saved() {
                return;
            }

            status.set(OverviewStatus::Saving);
            let req = UpdateQuizRequest {
                title: Some(current_title.clone()),
                description: Some(current_description.clone()),
                ..Default::default()
            };

            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to auto-save overview: {:?}", err);
                status.set(OverviewStatus::Unsaved);
            } else {
                last_saved.set((current_title, current_description));
                status.set(OverviewStatus::Saved);
                let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                invalidate_query(&keys);
            }
        });
    });

    let on_save = move |_| {
        if !can_edit {
            return;
        }
        let mut toast = toast;
        spawn(async move {
            status.set(OverviewStatus::Saving);
            let req = UpdateQuizRequest {
                title: Some(title()),
                description: Some(description()),
                ..Default::default()
            };
            if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                error!("Failed to update overview: {:?}", err);
                status.set(OverviewStatus::Unsaved);
                toast.error(err);
            } else {
                last_saved.set((title(), description()));
                status.set(OverviewStatus::Saved);
                let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                invalidate_query(&keys);
            }
        });
    };

    rsx! {
        div { class: "flex min-h-0 w-full flex-1 flex-col gap-[10px]",
            div { class: "flex w-full flex-col justify-center gap-2",
                div { class: "flex w-full items-center justify-between gap-4",
                    div { class: "flex flex-row items-center",
                        span { class: "text-[15px]/[18px] font-bold tracking-[-0.16px] text-quiz-overview-label",
                            {tr.title_label}
                        }
                        if can_edit {
                            span { class: "text-[15px]/[18px] font-bold text-red-500",
                                "*"
                            }
                        }
                    }
                    if can_edit {
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "min-w-[110px] inline-flex items-center justify-center gap-2 border-white text-white hover:text-white",
                            disabled: status() == OverviewStatus::Saving,
                            onclick: on_save,
                            crate::common::icons::other_devices::Save { class: "w-5 h-5 [&>path]:stroke-white [&>path]:fill-transparent" }
                            {tr.btn_save}
                        }
                    }
                }

                div { class: "flex w-full flex-col gap-1",
                    div { class: "relative flex h-12 w-full items-center",
                        Input {
                            class: "w-full bg-transparent pr-14".to_string(),
                            placeholder: tr.title_placeholder,
                            value: title(),
                            disabled: !can_edit,
                            maxlength: 50,
                            oninput: move |evt: Event<FormData>| {
                                let next_title = evt.value();
                                title.set(next_title.clone());
                                if (next_title, description()) == last_saved() {
                                    status.set(OverviewStatus::Saved);
                                } else {
                                    status.set(OverviewStatus::Unsaved);
                                    save_version += 1;
                                }
                            },
                        }
                        if can_edit {
                            span { class: "pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-[12px]/[15px] font-semibold text-quiz-overview-counter",
                                "{title_count}/50"
                            }
                        }
                    }
                }
            }

            div { class: "flex min-h-0 w-full flex-1 flex-col overflow-hidden rounded-[8px] border border-quiz-editor-border bg-quiz-editor-bg px-3 pb-1 pt-3",
                div { class: "flex min-h-0 w-full grow flex-1 flex-col gap-3",
                    crate::common::components::TiptapEditor {
                        class: "min-h-0 h-full w-full flex-1 [&>div]:flex [&>div]:h-full [&>div]:min-h-0 [&>div]:flex-1 [&>div]:overflow-hidden [&>div]:bg-transparent [&>div]:border-0 [&_[data-tiptap-toolbar]]:border-b [&_[data-tiptap-toolbar]]:border-quiz-editor-toolbar-border [&_[data-tiptap-toolbar]]:bg-transparent [&_[contenteditable='true']]:h-full [&_[contenteditable='true']]:min-h-[96px] [&_[contenteditable='true']]:overflow-y-auto [&_[contenteditable='true']]:bg-transparent [&_[contenteditable='true']]:px-0 [&_[contenteditable='true']]:text-[15px] [&_[contenteditable='true']]:leading-[22px] [&_[contenteditable='true']]:font-medium [&_[contenteditable='true']]:text-text-primary [&_[contenteditable='true']]:outline-none [&_[contenteditable='true']]:placeholder:text-quiz-editor-placeholder",
                        content: description(),
                        editable: can_edit,
                        placeholder: tr.description_placeholder,
                        on_content_change: move |html: String| {
                            description.set(html.clone());
                            if (title(), html) == last_saved() {
                                status.set(OverviewStatus::Saved);
                            } else {
                                status.set(OverviewStatus::Unsaved);
                                save_version += 1;
                            }
                        },
                    }
                }
            }

            if can_edit && status() != OverviewStatus::Idle {
                div { class: "mt-1 flex w-full justify-end",
                    div { class: "rounded bg-card px-2 py-1 text-xs text-text-tertiary",
                        match status() {
                            OverviewStatus::Saving => rsx! {
                                {tr.saving}
                            },
                            OverviewStatus::Saved => rsx! {
                                {tr.all_changes_saved}
                            },
                            OverviewStatus::Unsaved => rsx! {
                                {tr.unsaved_changes}
                            },
                            OverviewStatus::Idle => rsx! { "" },
                        }
                    }
                }
            }
        }
    }
}
