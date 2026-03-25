mod analyze;

pub use analyze::*;

use crate::common::hooks::use_infinite_query;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::space_common::hooks::use_space_role;

#[component]
pub fn SpaceAnalyzesAppPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let role = use_space_role()();
    let nav = use_navigator();

    if role != SpaceUserRole::Creator {
        return rsx! {};
    }

    let mut polls_query =
        use_infinite_query(move |bookmark| list_analyze_polls(space_id(), bookmark))?;
    let polls = polls_query.items();
    let is_loading = polls_query.is_loading();
    let has_more = polls_query.has_more();

    #[cfg(feature = "local-dev")]
    let mut discussions_query =
        use_infinite_query(move |bookmark| list_analyze_discussions(space_id(), bookmark))?;
    #[cfg(feature = "local-dev")]
    let discussions = discussions_query.items();
    #[cfg(feature = "local-dev")]
    let discussions_is_loading = discussions_query.is_loading();
    #[cfg(feature = "local-dev")]
    let discussions_has_more = discussions_query.has_more();

    #[cfg(feature = "local-dev")]
    let discussion_section = rsx! {
        div { class: "flex w-full flex-col gap-2.5",
            h3 { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                {tr.discussion_section_title}
            }
            if discussions.is_empty() {
                div { class: "py-8 text-center text-sm text-font-secondary", {tr.no_discussions} }
            } else {
                for discussion in discussions {
                    {
                        let discussion_id = discussion.discussion_id.clone();
                        let title = if discussion.title.trim().is_empty() {
                            tr.untitled_discussion.to_string()
                        } else {
                            discussion.title.clone()
                        };

                        rsx! {
                            Card { class: "w-full flex flex-row items-center justify-between".to_string(),
                                div { class: "flex w-full flex-col gap-1",
                                    div { class: "text-[12px] font-semibold leading-[20px] text-neutral-300",
                                        {tr.discussion_section_title}
                                    }
                                    div { class: "text-base font-medium leading-[20px] text-text-primary", {title} }
                                }
                                div { class: "flex shrink-0 gap-2",
                                    Button {
                                        class: "min-w-[120px] whitespace-nowrap bg-btn-analyze-view-bg",
                                        shape: ButtonShape::Square,
                                        onclick: move |_| {
                                            nav.push(Route::SpaceAnalyzeDiscussionPage {
                                                space_id: space_id(),
                                                discussion_id: discussion_id.clone(),
                                            });
                                        },
                                        {tr.view_analyze}
                                    }
                                }
                            }
                        }
                    }
                }

                if discussions_has_more {
                    button {
                        class: "self-center mt-2 rounded-md border border-divider px-4 py-2 hover:bg-white/5 disabled:opacity-60",
                        disabled: discussions_is_loading,
                        onclick: move |_| {
                            discussions_query.next();
                        },
                        {tr.more}
                    }
                }
            }
        }
    };

    #[cfg(not(feature = "local-dev"))]
    let discussion_section = rsx! {};

    rsx! {
        div { class: "flex w-full flex-col gap-5",
            div { class: "flex w-full flex-col gap-2.5",
                h3 { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                    {tr.poll_section_title}
                }
                if polls.is_empty() {
                    div { class: "py-8 text-center text-sm text-font-secondary", {tr.no_polls} }
                } else {
                    for poll in polls {
                        if poll.questions_count > 0 {
                            {
                                let poll_id = poll.poll_id.clone();
                                let title = poll.title.clone();
                                let questions = tr.questions.to_string();
                                rsx! {
                                    Card { class: "w-full flex flex-row items-center justify-between".to_string(),
                                        div { class: "flex w-full flex-col gap-1",
                                            div { class: "text-[12px] font-semibold leading-[20px] text-neutral-300",
                                                "{poll.questions_count} {questions}"
                                            }
                                            div { class: "text-base font-medium leading-[20px] text-text-primary", {title} }
                                        }
                                        div { class: "flex shrink-0 gap-2",
                                            Button {
                                                class: "min-w-[120px] whitespace-nowrap bg-btn-analyze-view-bg",
                                                shape: ButtonShape::Square,
                                                onclick: move |_| {
                                                    nav.push(Route::SpaceAnalyzeDetailPage {
                                                        space_id: space_id(),
                                                        poll_id: poll_id.clone(),
                                                    });
                                                },
                                                {tr.view_analyze}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if has_more {
                        button {
                            class: "self-center mt-2 rounded-md border border-divider px-4 py-2 hover:bg-white/5 disabled:opacity-60",
                            disabled: is_loading,
                            onclick: move |_| {
                                polls_query.next();
                            },
                            {tr.more}
                        }
                    }
                }
            }
            {discussion_section}
        }
    }
}
