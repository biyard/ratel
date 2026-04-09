use crate::common::components::{
    Button, ButtonShape, ButtonSize, Sheet, SheetContent, SheetHeader, SheetSide, SheetTitle,
};
use crate::features::spaces::pages::actions::actions::discussion::components::DiscussionComments;
use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionViewerTranslate;
use crate::features::spaces::pages::actions::actions::discussion::*;
use lucide_dioxus::MessageCircle;

#[component]
pub fn CommentsDrawer(
    open: Signal<bool>,
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
    can_comment: bool,
    can_manage_comments: bool,
    current_user_pk: Option<String>,
    comment_count: usize,
) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();
    let mut open_sig = open;

    rsx! {
        Sheet {
            open: open_sig(),
            on_open_change: move |v| open_sig.set(v),
            SheetContent {
                side: SheetSide::Right,
                class: "w-full max-w-[420px] max-mobile:max-w-full max-mobile:h-[85vh] max-mobile:w-full",
                SheetHeader {
                    SheetTitle { "{tr.comments} ({comment_count})" }
                }
                div { class: "overflow-y-auto flex-1 px-4 pb-6",
                    DiscussionComments {
                        space_id,
                        discussion_id,
                        can_comment,
                        can_manage_comments,
                        current_user_pk,
                    }
                }
            }
        }
    }
}

#[component]
pub fn FloatingCommentsButton(open: Signal<bool>, comment_count: usize) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();
    let mut open_sig = open;

    if open_sig() {
        return rsx! {};
    }

    rsx! {
        div { class: "fixed right-6 bottom-6 z-40",
            Button {
                size: ButtonSize::Icon,
                shape: ButtonShape::Rounded,
                class: "w-14 h-14 shadow-lg transition-transform hover:scale-105",
                "aria-label": "{tr.open_comments}",
                onclick: move |_| open_sig.set(true),
                MessageCircle { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
                if comment_count > 0 {
                    span { class: "flex absolute -top-1 -right-1 justify-center items-center px-1 h-5 text-xs font-bold rounded-full min-w-5 bg-primary text-btn-primary-text",
                        "{comment_count}"
                    }
                }
            }
        }
    }
}
