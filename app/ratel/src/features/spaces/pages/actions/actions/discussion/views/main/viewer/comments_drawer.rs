use crate::features::spaces::pages::actions::actions::discussion::*;

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
    rsx! {}
}

#[component]
pub fn FloatingCommentsButton(open: Signal<bool>, comment_count: usize) -> Element {
    rsx! {}
}
