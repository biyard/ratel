use bdk::prelude::*;
use dto::Space;

use crate::pages::components::{ThreadComments, ThreadContents, ThreadFiles, ThreadHeader};

#[component]
pub fn Threads(
    lang: Language,
    space: Space,
    ondownload: EventHandler<(String, Option<String>)>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start px-10 gap-25 max-tablet:gap-40",
            ThreadHeader {
                lang,
                profile: space.proposer_profile.unwrap_or_default(),
                proposer: space.proposer_nickname.unwrap_or_default(),
                title: space.title.unwrap_or_default(),
                number_of_comments: space.comments,
                number_of_rewards: space.rewards,
                number_of_shared: space.shares,
                created_at: space.created_at,
                content_type: space.content_type,
                onback: move |_| {},
            }

            div { class: "flex flex-col w-full justify-start items-start gap-10",
                ThreadContents { description: space.html_contents }
                ThreadFiles { lang, files: space.files, ondownload }
                ThreadComments {
                    lang,
                    number_of_comments: space.comments,
                    comments: vec![],
                }
            }
        }
    }
}
