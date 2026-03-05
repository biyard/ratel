use crate::components::FollowTabsTranslate;
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FollowTab {
    Followers,
    Followings,
}

impl Default for FollowTab {
    fn default() -> Self {
        FollowTab::Followings
    }
}

#[component]
pub fn FollowTabs(selected: FollowTab, on_select: EventHandler<FollowTab>) -> Element {
    let tr: FollowTabsTranslate = use_translate();
    let is_followers = selected == FollowTab::Followers;
    let is_followings = selected == FollowTab::Followings;
    let follower_class = if is_followers {
        "cursor-pointer flex flex-col min-w-[110px] items-center text-text-primary text-[15px] font-semibold pb-2 border-b border-b-neutral-500 mr-8"
    } else {
        "cursor-pointer flex flex-col min-w-[110px] items-center text-text-secondary text-[15px] font-medium pb-2 border-none mr-8"
    };
    let following_class = if is_followings {
        "cursor-pointer flex flex-col min-w-[110px] items-center text-text-primary text-[15px] font-semibold pb-2 border-b border-b-neutral-500"
    } else {
        "cursor-pointer flex flex-col min-w-[110px] items-center text-text-secondary text-[15px] font-medium pb-2 border-none"
    };

    rsx! {
        div { class: "flex flex-row w-full justify-center items-end gap-[20px]",
            div {
                class: follower_class,
                onclick: move |_| on_select.call(FollowTab::Followers),
                {tr.followers}
            }
            div {
                class: following_class,
                onclick: move |_| on_select.call(FollowTab::Followings),
                {tr.followings}
            }
        }
    }
}
