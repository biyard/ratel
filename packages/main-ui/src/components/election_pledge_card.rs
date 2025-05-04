use bdk::prelude::{by_components::icons::emoji::Heart, *};
use dto::*;
use num_format::{Locale, ToFormattedString};

#[component]
pub fn ElectionPledgeCard(promise: ElectionPledge) -> Element {
    let mut liked = use_signal(move || promise.liked);
    let mut likes = use_signal(move || promise.likes);

    rsx! {
        div {
            id: "election-pledge-{promise.id}",
            class: "w-full border border-c-wg-70 py-16 px-20 rounded-[10px]",
            article {
                class: "election-pledge",
                dangerous_inner_html: promise.promise,
            }

            div {
                class: "w-full flex flex-row justify-end items-center gap-4 text-base/25 font-semibold tracking-[0.5px] text-white group",
                "aria-already-liked": liked(),
                button {
                    class: "flex flex-row items-center gap-4 hover:bg-background px-10 py-5 rounded-full cursor-pointer",
                    onclick: move |_| async move {
                        if liked() {
                            btracing::e!(ServiceError::AlreadyLiked);
                            return;
                        }
                        match ElectionPledge::get_client(crate::config::get().main_api_endpoint)
                            .like(promise.id)
                            .await
                        {
                            Ok(_) => {
                                btracing::i!(Info::LikePledge);
                                liked.set(true);
                                likes.set(likes() + 1);
                            }
                            Err(e) => {
                                btracing::e!(e);
                            }
                        }
                    },
                    {likes().to_formatted_string(&Locale::en)}
                    Heart {
                        class: "[&>path]:stroke-neutral-400 group-aria-already-liked:[&>path]:fill-neutral-400",
                        width: "20",
                        height: "20",
                    }
                }
            }

        }
    }
}
