use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionViewerTranslate;
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::common::utils::time::time_ago;

#[component]
pub fn DiscHeader(discussion: SpacePost) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();
    let has_category = !discussion.category_name.is_empty();

    let created_at = if discussion.created_at.abs() < 1_000_000_000_000 {
        discussion.created_at.saturating_mul(1000)
    } else {
        discussion.created_at
    };
    let date_str = time_ago(created_at);

    let title = if discussion.title.is_empty() {
        tr.untitled_discussion.to_string()
    } else {
        discussion.title.clone()
    };

    rsx! {
        div { class: "disc-header",
            span { class: "disc-header__type",
                lucide_dioxus::MessageSquare { class: "w-[14px] h-[14px]" }
                "{tr.discussion_label}"
            }
            h1 { class: "disc-header__title", "{title}" }
            div { class: "disc-header__meta",
                div { class: "disc-header__author",
                    if !discussion.author_profile_url.is_empty() {
                        img {
                            class: "disc-header__avatar",
                            src: "{discussion.author_profile_url}",
                            alt: "{discussion.author_display_name}",
                        }
                    }
                    span { class: "disc-header__author-name", "{discussion.author_display_name}" }
                }
                span { class: "disc-header__separator" }
                span { class: "disc-header__date", "{date_str}" }
                if has_category {
                    span { class: "disc-header__category", "{discussion.category_name}" }
                }
            }
        }
    }
}
