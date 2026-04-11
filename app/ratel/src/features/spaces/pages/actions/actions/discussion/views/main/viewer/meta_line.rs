use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionViewerTranslate;
use crate::features::spaces::pages::actions::actions::discussion::*;
use dioxus_translate::*;
use lucide_dioxus::{MessageCircle, Tag};

#[component]
pub fn DiscussionMetaLine(discussion: SpacePost) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();
    let has_category = !discussion.category_name.is_empty();
    let comments = discussion.comments.max(0) as usize;

    rsx! {
        div { class: "flex flex-wrap gap-3 items-center text-sm text-foreground-muted",
            // Author
            div { class: "flex gap-2 items-center",
                if !discussion.author_profile_url.is_empty() {
                    img {
                        class: "object-cover w-6 h-6 rounded-full",
                        src: "{discussion.author_profile_url}",
                        alt: "{discussion.author_display_name}",
                    }
                }
                span { class: "font-medium text-text-primary", "{discussion.author_display_name}" }
            }

            // Category
            if has_category {
                span { class: "text-foreground-muted", "·" }
                div { class: "flex gap-1 items-center",
                    Tag { class: "w-3.5 h-3.5 [&>path]:stroke-icon-primary" }
                    span { "{discussion.category_name}" }
                }
            }

            // Comment count
            span { class: "text-foreground-muted", "·" }
            div { class: "flex gap-1 items-center",
                MessageCircle { class: "w-3.5 h-3.5 [&>path]:stroke-icon-primary" }
                span { "{comments} {tr.comments_count}" }
            }
        }
    }
}
