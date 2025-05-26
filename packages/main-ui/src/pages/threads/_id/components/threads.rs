use bdk::prelude::*;
use dto::Feed;

use crate::pages::components::{ThreadContents, ThreadFiles, ThreadHeader};

#[component]
pub fn Threads(
    lang: Language,
    feed: Feed,
    user_id: i64,
    create_space: EventHandler<MouseEvent>,
    ondownload: EventHandler<(String, Option<String>)>,
    onprev: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start px-10 gap-25 max-tablet:gap-40",
            ThreadHeader {
                lang,
                is_creator: user_id == feed.user_id,
                profile: feed.profile_image.unwrap_or_default(),
                proposer: feed.proposer_name.unwrap_or_default(),
                title: feed.title.unwrap_or_default(),
                number_of_comments: feed.comments,
                number_of_rewards: feed.rewards,
                number_of_shared: feed.shares,
                created_at: feed.created_at,
                feed_type: feed.feed_type,
                create_space,
                onprev,
            }

            div { class: "flex flex-col w-full justify-start items-start gap-10",
                ThreadContents { description: feed.html_contents }
                ThreadFiles { lang, files: vec![], ondownload }
                        // ThreadComments {
            //     lang,
            //     number_of_comments: feed.comments,
            //     comments: vec![],
            // }
            }
        }
    }
}
