use crate::features::my_follower::components::FollowListTranslate;
use crate::features::my_follower::components::follow_tabs::FollowTab;
use crate::features::my_follower::controllers::FollowUserItem;
use crate::features::my_follower::*;

#[component]
pub fn FollowList(
    users: Vec<FollowUserItem>,
    selected: FollowTab,
    loading: bool,
    on_follow: EventHandler<Partition>,
    on_unfollow: EventHandler<Partition>,
    more_element: Element,
) -> Element {
    let tr: FollowListTranslate = use_translate();
    rsx! {
        Card { class: "flex flex-col w-full px-4 py-5 gap-0",
            if users.is_empty() {
                if loading {
                    div { class: "flex flex-row w-full h-fit justify-start items-center py-4 font-medium text-base text-gray-500",
                        {tr.loading}
                    }
                } else {
                    div { class: "flex flex-row w-full h-fit justify-start items-center py-4 font-medium text-base text-gray-500",
                        {tr.empty}
                    }
                }
            } else {
                div { class: "flex flex-col",
                    for (idx , user) in users.iter().enumerate() {
                        {
                            let user_pk = user.user_pk.clone();
                            let is_team = matches!(user.user_type, UserType::Team);
                            let is_following = user.is_following;
                            let is_followers_tab = selected == FollowTab::Followers;
                            let on_follow = on_follow.clone();
                            let on_unfollow = on_unfollow.clone();
                            let is_last = idx + 1 == users.len();
                            let row_class = if is_last {
                                "flex flex-col w-full gap-[5px] py-5"
                            } else {
                                "flex flex-col w-full gap-[5px] py-5 border-b border-b-neutral-800 light:border-b-[#e5e5e5]"
                            };

                            rsx! {
                                div { key: "{user_pk}", class: row_class,
                                    div { class: "flex flex-row w-full justify-between items-start",
                                        div { class: "flex flex-row w-fit gap-2",
                                            if !user.profile_url.is_empty() {
                                                img {
                                                    src: "{user.profile_url}",
                                                    alt: "{user.display_name}",
                                                    class: if is_team { "w-8 h-8 rounded-lg object-cover object-top" } else { "w-8 h-8 rounded-full object-cover object-top" },
                                                }
                                            } else {
                                                div { class: if is_team { "w-8 h-8 rounded-lg bg-neutral-500" } else { "w-8 h-8 rounded-full bg-neutral-500" } }
                                            }

                                            div { class: "flex flex-col",
                                                div { class: "font-semibold text-text-primary text-sm/[20px]", "{user.display_name}" }
                                                div { class: "font-medium text-neutral-500 text-[12px]", "@{user.username}" }
                                            }
                                        }

                                        if is_followers_tab {
                                            if !is_following {
                                                FollowButton { on_click: move |_| on_follow.call(user_pk.clone()) }
                                            }
                                        } else {
                                            UnfollowButton { on_click: move |_| on_unfollow.call(user_pk.clone()) }
                                        }
                                    }

                                    if !user.description.is_empty() {
                                        div { class: "font-medium text-[12px] text-neutral-300 light:text-text-primary",
                                            "{user.description}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                {more_element}
            }
        }
    }
}

#[component]
fn FollowButton(on_click: EventHandler<MouseEvent>) -> Element {
    let tr: FollowListTranslate = use_translate();
    rsx! {
        Button {
            size: ButtonSize::Small,
            style: ButtonStyle::Outline,
            class: "w-fit h-fit px-[10px] py-[5px] rounded-[50px] items-center border border-neutral-700 hover:border-[#ff4d4f] hover:bg-[#ffe3e3] text-neutral-700"
                .to_string(),
            onclick: on_click,
            span { class: "inline-flex items-center gap-1 font-bold text-xs",
                common::lucide_dioxus::Plus {
                    size: 14,
                    class: "[&>path]:stroke-[#000203] light:[&>path]:stroke-[#000203]",
                }
                {tr.follow}
            }
        }
    }
}

#[component]
fn UnfollowButton(on_click: EventHandler<MouseEvent>) -> Element {
    let tr: FollowListTranslate = use_translate();
    rsx! {
        Button {
            size: ButtonSize::Small,
            style: ButtonStyle::Outline,
            class: "w-fit h-fit px-[10px] py-[5px] rounded-[50px] items-center border border-neutral-700 hover:border-[#ff4d4f] hover:bg-[#ffe3e3] text-neutral-700"
                .to_string(),
            onclick: on_click,
            span { class: "font-bold text-xs", {tr.unfollow} }
        }
    }
}
