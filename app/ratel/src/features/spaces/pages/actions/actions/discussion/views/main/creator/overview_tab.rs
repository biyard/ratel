use super::*;
use crate::features::spaces::space_common::types::space_page_actions_discussion_key;

#[derive(Clone, Copy, PartialEq, Eq)]
enum OverviewStatus {
    Idle,
    Saving,
    Saved,
    Unsaved,
}

#[component]
pub fn OverviewTab() -> Element {
    let ctx = use_discussion_context();
    let tr: DiscussionCreatorTranslate = use_translate();
    let mut toast = use_toast();
    let discussion = ctx.discussion.read().clone();
    let initial_title = discussion.post.title.clone();
    let initial_description = discussion.post.html_contents.clone();
    let initial_category = discussion.post.category_name.clone();
    let mut title = use_signal(|| initial_title.clone());
    let mut description = use_signal(|| initial_description.clone());
    let mut category_name = use_signal(|| initial_category.clone());
    let mut last_saved = use_signal(|| (initial_title, initial_description, initial_category));
    let mut status = use_signal(|| OverviewStatus::Idle);
    let mut save_version = use_signal(|| 0u64);
    let mut query = use_query_store();
    let title_count = std::cmp::min(title().chars().count(), 50);
    let space_id = ctx.space_id;
    let discussion_id = ctx.discussion_id;

    use_effect(move || {
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
            let current_category = category_name();
            if (current_title.clone(), current_description.clone(), current_category.clone())
                == last_saved()
            {
                return;
            }

            status.set(OverviewStatus::Saving);
            let req = UpdateDiscussionRequest {
                title: Some(current_title.clone()),
                html_contents: Some(current_description.clone()),
                category_name: if current_category.is_empty() {
                    None
                } else {
                    Some(current_category.clone())
                },
                started_at: None,
                ended_at: None,
            };

            if let Err(err) = update_discussion(space_id(), discussion_id(), req).await {
                error!("Failed to auto-save discussion: {:?}", err);
                status.set(OverviewStatus::Unsaved);
            } else {
                last_saved.set((current_title, current_description, current_category));
                status.set(OverviewStatus::Saved);
                let keys = space_page_actions_discussion_key(&space_id(), &discussion_id());
                query.invalidate(&keys);
            }
        });
    });

    let on_save = move |_| {
        let mut toast = toast;
        spawn(async move {
            status.set(OverviewStatus::Saving);
            let req = UpdateDiscussionRequest {
                title: Some(title()),
                html_contents: Some(description()),
                category_name: if category_name().is_empty() {
                    None
                } else {
                    Some(category_name())
                },
                started_at: None,
                ended_at: None,
            };
            if let Err(err) = update_discussion(space_id(), discussion_id(), req).await {
                error!("Failed to update discussion: {:?}", err);
                status.set(OverviewStatus::Unsaved);
                toast.error(err);
            } else {
                last_saved.set((title(), description(), category_name()));
                status.set(OverviewStatus::Saved);
                let keys = space_page_actions_discussion_key(&space_id(), &discussion_id());
                query.invalidate(&keys);
            }
        });
    };

    let mut mark_changed = move || {
        let current = (title(), description(), category_name());
        if current == last_saved() {
            status.set(OverviewStatus::Saved);
        } else {
            status.set(OverviewStatus::Unsaved);
            save_version += 1;
        }
    };

    rsx! {
        div { class: "flex min-h-0 w-full flex-1 flex-col gap-[10px]",
            div { class: "flex w-full flex-col justify-center gap-2",
                div { class: "flex w-full items-center justify-between gap-4",
                    div { class: "flex flex-row items-center",
                        span { class: "text-[15px]/[18px] font-bold tracking-[-0.16px] text-text-primary",
                            {tr.title_label}
                        }
                        span { class: "text-[15px]/[18px] font-bold text-red-500",
                            "*"
                        }
                    }
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

                div { class: "flex w-full flex-col gap-1",
                    div { class: "relative flex h-12 w-full items-center",
                        Input {
                            class: "w-full bg-transparent pr-14".to_string(),
                            placeholder: tr.title_placeholder,
                            value: title(),
                            maxlength: 50,
                            oninput: move |evt: Event<FormData>| {
                                title.set(evt.value());
                                mark_changed();
                            },
                        }
                        span { class: "pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-[12px]/[15px] font-semibold text-text-tertiary",
                            "{title_count}/50"
                        }
                    }
                }
            }

            // Category input
            div { class: "flex w-full flex-col gap-2",
                span { class: "text-[15px]/[18px] font-bold tracking-[-0.16px] text-text-primary",
                    {tr.category_label}
                }
                Input {
                    class: "w-full bg-transparent".to_string(),
                    placeholder: tr.category_placeholder,
                    value: category_name(),
                    oninput: move |evt: Event<FormData>| {
                        category_name.set(evt.value());
                        mark_changed();
                    },
                }
            }

            // Rich text editor for content
            div { class: "flex min-h-0 w-full flex-1 flex-col overflow-hidden rounded-[8px] border border-c-wg-30 bg-background px-3 pb-1 pt-3",
                div { class: "flex min-h-0 w-full grow flex-1 flex-col gap-3",
                    crate::common::components::TiptapEditor {
                        class: "min-h-0 h-full w-full flex-1 [&>div]:flex [&>div]:h-full [&>div]:min-h-0 [&>div]:flex-1 [&>div]:overflow-hidden [&>div]:bg-transparent [&>div]:border-0 [&_[data-tiptap-toolbar]]:border-b [&_[data-tiptap-toolbar]]:border-c-wg-30 [&_[data-tiptap-toolbar]]:bg-transparent [&_[contenteditable='true']]:h-full [&_[contenteditable='true']]:min-h-[96px] [&_[contenteditable='true']]:overflow-y-auto [&_[contenteditable='true']]:bg-transparent [&_[contenteditable='true']]:px-0 [&_[contenteditable='true']]:text-[15px] [&_[contenteditable='true']]:leading-[22px] [&_[contenteditable='true']]:font-medium [&_[contenteditable='true']]:text-text-primary [&_[contenteditable='true']]:outline-none [&_[contenteditable='true']]:placeholder:text-text-tertiary",
                        content: description(),
                        editable: true,
                        placeholder: tr.description_placeholder,
                        on_content_change: move |html: String| {
                            description.set(html);
                            mark_changed();
                        },
                    }
                }
            }

            if status() != OverviewStatus::Idle {
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
