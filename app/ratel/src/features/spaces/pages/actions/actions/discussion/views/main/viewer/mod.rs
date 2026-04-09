mod attachments;
mod comments_drawer;
mod content_body;
mod i18n;
mod layout;
mod meta_line;
mod properties;
mod table_of_contents;
mod toc_context;

pub use attachments::DiscussionAttachments;
pub use comments_drawer::{CommentsDrawer, FloatingCommentsButton};
pub use content_body::DiscussionContentBody;
pub use i18n::DiscussionViewerTranslate;
pub use layout::NotionLayout;
pub use meta_line::DiscussionMetaLine;
pub use properties::DiscussionProperties;
pub use table_of_contents::DiscussionToc;
pub use toc_context::{DiscussionTocContext, TocEntry, use_discussion_toc_context};

use super::*;
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::space_common::hooks::use_space;

#[component]
pub fn ViewerMain(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let _tr: DiscussionViewerTranslate = use_translate();
    let ctx = use_discussion_context();
    let discussion_response = ctx.discussion();
    let _discussion = discussion_response.post;

    rsx! {
        div { class: "flex flex-col gap-6 mx-auto w-full max-w-[1080px] px-4 py-6 md:px-6 md:py-8 desktop:px-8",
            div { class: "text-text-primary", "ViewerMain scaffold — filling in tasks" }
        }
    }
}
