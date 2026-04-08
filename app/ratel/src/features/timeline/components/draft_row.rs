use crate::common::utils::time::time_ago;
use crate::common::*;
use crate::features::posts::components::FeedContents;
use crate::features::posts::controllers::delete_post::delete_post_handler;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::controllers::list_user_drafts::list_user_drafts_handler;
use crate::features::posts::types::*;
use dioxus_translate::use_language;
use std::collections::HashSet;

translate! {
    DraftTimelineTranslate;
    drafts_title: {
        en: "Drafts",
        ko: "초안",
    },
    delete_draft_title: {
        en: "Delete Draft",
        ko: "초안 삭제",
    },
    delete_draft_description: {
        en: "Are you sure you want to delete this draft? This action cannot be undone.",
        ko: "이 초안을 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    confirm: {
        en: "Confirm",
        ko: "확인",
    },
}

/// Shared horizontal-scrolling row of draft post cards.
///
/// Responsibilities:
/// - Render the "Drafts" heading and the scroll container
/// - Render each card (badge, title, preview, updated_at)
/// - Handle the delete button + confirmation popup
/// - Track left/right scroll affordance buttons via `onmounted`/`onscroll`
///
/// The parent is responsible for fetching the draft list and passing it in.
/// The `aria_label` is used both as the accessibility label and as the JS
/// query selector anchor, so it must be unique within the rendered page
/// (e.g. `"Drafts section"` vs `"Team Drafts section"`).
#[component]
pub fn DraftScrollRow(
    items: Vec<PostResponse>,
    aria_label: String,
    #[props(default)] test_id: Option<String>,
) -> Element {
    let tr: DraftTimelineTranslate = use_translate();
    let nav = use_navigator();
    let lang = use_language();
    let mut popup = use_popup();
    let mut can_scroll_left = use_signal(|| false);
    let mut can_scroll_right = use_signal(|| false);
    let deleted = use_signal(HashSet::<String>::new);

    let deleted_keys = deleted.read().clone();
    let visible: Vec<PostResponse> = items
        .into_iter()
        .filter(|post| !deleted_keys.contains(&post.pk.to_string()))
        .collect();

    if visible.is_empty() {
        return rsx! {};
    }

    // JS selectors scoped to the specific aria-label so multiple instances
    // on the same page don't collide. Built once per render.
    let check_scroll_js = format!(
        r#"
        const el = document.querySelector('[aria-label="{label}"] .scrollbar-none');
        if (el) {{
            dioxus.send([el.scrollLeft > 0, el.scrollLeft + el.clientWidth < el.scrollWidth - 1]);
        }} else {{
            dioxus.send([false, false]);
        }}
        "#,
        label = aria_label,
    );
    let scroll_left_js = format!(
        r#"
        const el = document.querySelector('[aria-label="{label}"] .scrollbar-none');
        if (el) el.scrollBy({{ left: -340, behavior: 'smooth' }});
        "#,
        label = aria_label,
    );
    let scroll_right_js = format!(
        r#"
        const el = document.querySelector('[aria-label="{label}"] .scrollbar-none');
        if (el) el.scrollBy({{ left: 340, behavior: 'smooth' }});
        "#,
        label = aria_label,
    );

    let check_js_mount = check_scroll_js.clone();
    let check_js_scroll = check_scroll_js.clone();
    let left_js = scroll_left_js.clone();
    let right_js = scroll_right_js.clone();

    rsx! {
        section {
            class: "flex flex-col gap-3 w-full",
            aria_label: "{aria_label}",
            "data-testid": test_id.clone().unwrap_or_default(),
            h2 { class: "px-1 text-lg font-semibold text-text-primary", "{tr.drafts_title}" }
            div { class: "relative",
                div {
                    class: "flex overflow-x-auto gap-4 pb-2 snap-x snap-mandatory scrollbar-none",
                    onmounted: move |_| {
                        let js = check_js_mount.clone();
                        spawn(async move {
                            let mut eval = document::eval(&js);
                            if let Ok(val) = eval.recv::<Vec<bool>>().await {
                                can_scroll_left.set(val[0]);
                                can_scroll_right.set(val[1]);
                            }
                        });
                    },
                    onscroll: move |_| {
                        let js = check_js_scroll.clone();
                        spawn(async move {
                            let mut eval = document::eval(&js);
                            if let Ok(val) = eval.recv::<Vec<bool>>().await {
                                can_scroll_left.set(val[0]);
                                can_scroll_right.set(val[1]);
                            }
                        });
                    },
                    for post in visible {
                        {
                            let post_pk_for_nav = post.pk.clone();
                            let post_pk_for_delete = post.pk.clone();
                            rsx! {
                                div {
                                    key: "draft-{post.pk}",
                                    class: "flex relative flex-col pt-5 pb-5 border cursor-pointer snap-start shrink-0 w-[340px] max-mobile:w-[280px] rounded-[10px] bg-card-bg-secondary border-card-enable-border group",
                                    onclick: move |_| {
                                        let nav = nav.clone();
                                        let post_pk: FeedPartition = post_pk_for_nav.clone().into();
                                        nav.push(format!("/posts/{post_pk}/edit"));
                                    },
                                    button {
                                        class: "absolute top-2 right-2 p-1.5 rounded-full transition-opacity cursor-pointer opacity-60 group-hover:opacity-100 z-[10] hover:bg-destructive/10",
                                        aria_label: "Delete draft",
                                        onclick: move |e: MouseEvent| {
                                            e.stop_propagation();
                                            e.prevent_default();
                                            let post_pk = post_pk_for_delete.clone();
                                            let on_cancel = {
                                                let mut popup = popup;
                                                move |_: MouseEvent| {
                                                    popup.close();
                                                }
                                            };
                                            let on_confirm = {
                                                let popup = popup;
                                                let deleted = deleted;
                                                move |_: MouseEvent| {
                                                    let post_pk = post_pk.clone();
                                                    let mut popup = popup;
                                                    let mut deleted = deleted;
                                                    spawn(async move {
                                                        let feed_pk: FeedPartition = post_pk.clone().into();
                                                        if delete_post_handler(feed_pk, None).await.is_ok() {
                                                            deleted.write().insert(post_pk.to_string());
                                                        }
                                                        popup.close();
                                                    });
                                                }
                                            };
                                            popup
                                                .open(rsx! {
                                                    DeleteDraftConfirmation { on_cancel, on_confirm }
                                                });
                                        },
                                        icons::edit::Delete2 {
                                            width: "18",
                                            height: "18",
                                            class: "[&>path]:stroke-icon-primary hover:[&>path]:stroke-destructive [&>path]:fill-transparent",
                                        }
                                    }
                                    Col { class: "gap-2 px-5 w-full",
                                        Badge {
                                            size: BadgeSize::Small,
                                            variant: BadgeVariant::Rounded,
                                            color: BadgeColor::Orange,
                                            {post.status.translate(&lang())}
                                        }
                                        p { class: "w-full text-base font-normal text-text-primary truncate line-clamp-1",
                                            "{post.title}"
                                        }
                                    }
                                    FeedContents {
                                        contents: post.html_contents.chars().take(200).collect::<String>(),
                                        urls: post.urls.clone(),
                                    }
                                    div { class: "flex flex-row items-center px-5 mt-4",
                                        p { class: "text-sm font-light align-middle text-foreground-muted",
                                            "{time_ago(post.updated_at)}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if can_scroll_left() {
                    button {
                        class: "absolute left-0 top-1/2 p-1 rounded-full transition-colors -translate-y-1/2 cursor-pointer z-[101] hover:bg-accent/20",
                        aria_label: "Scroll Drafts left",
                        onclick: move |_| {
                            let js = left_js.clone();
                            let _ = document::eval(&js);
                        },
                        lucide_dioxus::ChevronLeft {
                            size: 20,
                            class: "transition-colors [&>path]:stroke-foreground-muted hover:[&>path]:stroke-text-primary",
                        }
                    }
                }
                if can_scroll_right() {
                    button {
                        class: "absolute right-0 top-1/2 p-1 rounded-full transition-colors -translate-y-1/2 cursor-pointer z-[101] hover:bg-accent/20",
                        aria_label: "Scroll Drafts right",
                        onclick: move |_| {
                            let js = right_js.clone();
                            let _ = document::eval(&js);
                        },
                        lucide_dioxus::ChevronRight {
                            size: 20,
                            class: "transition-colors [&>path]:stroke-foreground-muted hover:[&>path]:stroke-text-primary",
                        }
                    }
                }
            }
        }
    }
}

/// A horizontal row of the current user's draft posts, displayed at the top
/// of the personal timeline. Thin wrapper around [`DraftScrollRow`].
#[component]
pub fn DraftTimeline() -> Element {
    let drafts = use_server_future(move || async move {
        let result = list_user_drafts_handler(None).await;
        if let Err(ref e) = result {
            tracing::error!("Failed to load drafts: {:?}", e);
        }
        result
    })?;

    let val = drafts.read();
    let res = val.as_ref().unwrap();

    let items = match res {
        Ok(resp) => resp.items.clone(),
        Err(_) => vec![],
    };

    if items.is_empty() {
        return rsx! {};
    }

    rsx! {
        DraftScrollRow {
            items,
            aria_label: "Drafts section".to_string(),
            test_id: "draft-timeline".to_string(),
        }
    }
}

#[component]
fn DeleteDraftConfirmation(
    on_confirm: EventHandler<MouseEvent>,
    on_cancel: EventHandler<MouseEvent>,
) -> Element {
    let tr: DraftTimelineTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col w-[400px] max-w-full gap-6",
            div { class: "flex flex-col gap-2",
                div { class: "text-lg font-bold text-center text-text-primary",
                    "{tr.delete_draft_title}"
                }
                div { class: "text-sm leading-6 text-center text-foreground-muted",
                    "{tr.delete_draft_description}"
                }
            }
            div { class: "flex gap-3 justify-end items-center",
                Button {
                    style: ButtonStyle::Outline,
                    size: ButtonSize::Small,
                    onclick: on_cancel,
                    "{tr.cancel}"
                }
                Button {
                    style: ButtonStyle::Primary,
                    size: ButtonSize::Small,
                    onclick: on_confirm,
                    "{tr.confirm}"
                }
            }
        }
    }
}
