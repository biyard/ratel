use bdk::prelude::*;

use crate::pages::{
    _id::controller::Thread,
    components::{ThreadComments, ThreadContents, ThreadFiles, ThreadHeader},
};

#[component]
pub fn Threads(
    lang: Language,
    thread: Thread,
    ondownload: EventHandler<(String, Option<String>)>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start px-10 gap-25 max-tablet:gap-40",
            ThreadHeader {
                lang,
                profile: thread.profile,
                proposer: thread.proposer,
                title: thread.title,
                number_of_comments: thread.number_of_comments,
                number_of_rewards: thread.number_of_rewards,
                number_of_shared: thread.number_of_shared,
                created_at: thread.created_at,
                content_type: thread.content_type,
                onback: move |_| {},
            }

            div { class: "flex flex-col w-full justify-start items-start gap-10",
                ThreadContents { description: thread.description }
                ThreadFiles { lang, files: thread.files, ondownload }
                ThreadComments {
                    lang,
                    number_of_comments: thread.number_of_comments,
                    comments: thread.comments,
                }
            }
        }
    }
}
