use super::*;

#[component]
pub fn OverviewPage(
    space_id: ReadSignal<SpacePartition>,
    quiz_id: ReadSignal<SpaceQuizEntityType>,
    initial_title: String,
    initial_description: String,
    can_edit: bool,
    #[props(default)] current_section: Option<Signal<QuizCreatorSection>>,
    #[props(default = true)] show_save: bool,
) -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let mut toast = use_toast();
    let mut title = use_signal(|| initial_title);
    let mut description = use_signal(|| initial_description);
    let title_count = std::cmp::max(title().chars().count(), 50);

    let on_save = {
        move |_| {
            let mut toast = toast;
            spawn(async move {
                let req = UpdateQuizRequest {
                    title: Some(title()),
                    description: Some(description()),
                    ..Default::default()
                };
                if let Err(err) = update_quiz(space_id(), quiz_id(), req).await {
                    error!("Failed to update overview: {:?}", err);
                    toast.error(err);
                } else {
                    let keys = space_page_actions_quiz_key(&space_id(), &quiz_id());
                    invalidate_query(&keys);
                }
            });
        }
    };

    rsx! {
        // Previous flat editor layout is intentionally kept conceptually,
        // but rendered as the new overview section for the tab flow.
        div { class: "flex w-full max-w-[1024px] flex-col gap-[10px]",
            div { class: "flex w-full flex-col justify-center gap-2",
                div { class: "flex items-center gap-1",
                    div { class: "flex flex-row items-center",
                        span { class: "text-[15px]/[18px] font-bold tracking-[-0.16px] text-[#8C8C8C]",
                            {tr.title_label}
                        }
                        span { class: "text-[15px]/[18px] font-bold text-[#EF4444]",
                            "*"
                        }
                    }
                                // crate::common::lucide_dioxus::Info { size: 18, class: "text-[#737373]" }
                }

                div { class: "flex w-full flex-col items-end gap-1",
                    div { class: "flex h-12 w-full items-center rounded-[8px] border border-[#525252] bg-[#101010] light:bg-[#f5f5f5] px-3",
                        Input {
                            variant: InputVariant::Plain,
                            class: "h-6 w-full text-[15px]/[24px] tracking-[0.5px] bg-[#101010] light:bg-[#f5f5f5] text-text-primary placeholder:text-[#525252] light:placeholder:text-[#a3a3a3] outline-none ring-0 focus:border-primary focus:outline-none focus:ring-0 focus-visible:outline-none focus-visible:ring-0"
                                .to_string(),
                            placeholder: tr.title_placeholder,
                            value: title(),
                            disabled: !can_edit,
                            maxlength: 50,
                            oninput: move |evt: Event<FormData>| title.set(evt.value()),
                        }
                    }
                    span { class: "text-[12px]/[15px] font-semibold text-[#6b6b6b]",
                        "{title_count}/50"
                    }
                }
            }

            div { class: "flex h-48 w-full flex-col rounded-[8px] border border-[#525252] bg-[#101010] light:bg-[#f5f5f5] px-3 pb-1 pt-3",
                div { class: "flex h-full w-full flex-col justify-between gap-3",
                    crate::common::components::TiptapEditor {
                        class: "w-full h-full [&>div]:bg-transparent [&>div]:border-0 [&_[data-tiptap-toolbar]]:border-b [&_[data-tiptap-toolbar]]:border-[#262626] [&_[data-tiptap-toolbar]]:bg-transparent [&_[contenteditable='true']]:min-h-[96px] [&_[contenteditable='true']]:bg-transparent [&_[contenteditable='true']]:px-0 [&_[contenteditable='true']]:text-[15px] [&_[contenteditable='true']]:leading-[22px] [&_[contenteditable='true']]:font-medium [&_[contenteditable='true']]:text-white light:[&_[contenteditable='true']]:text-[#a3a3a3] [&_[contenteditable='true']]:outline-none [&_[contenteditable='true']]:placeholder:text-[#525252]",
                        content: description(),
                        editable: can_edit,
                        placeholder: tr.description_placeholder,
                        on_content_change: move |html: String| {
                            description.set(html);
                        },
                    }
                }
            }

            if show_save || current_section.is_some() {
                div { class: "flex w-full justify-end gap-2.5 mt-10",
                    if show_save {
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            class: "min-w-[110px] inline-flex items-center justify-center gap-2 border-white text-white hover:text-white",
                            disabled: !can_edit,
                            onclick: on_save,
                            crate::common::icons::other_devices::Save { class: "w-5 h-5 [&>path]:stroke-white [&>path]:fill-transparent" }
                            {tr.btn_save}
                        }
                    }
                    if let Some(mut current_section) = current_section {
                        Button {
                            style: ButtonStyle::Primary,
                            shape: ButtonShape::Square,
                            class: "min-w-[110px] inline-flex items-center justify-center gap-2",
                            onclick: move |_| current_section.set(QuizCreatorSection::Upload),
                            {tr.btn_next}
                            icons::arrows::ArrowRight {
                                width: "20",
                                height: "20",
                                class: "shrink-0 [&>path]:stroke-current",
                            }
                        }
                    }
                }
            }
        }
    }
}
