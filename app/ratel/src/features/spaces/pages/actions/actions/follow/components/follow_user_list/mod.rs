use crate::features::spaces::pages::actions::actions::follow::controllers::delete_follow_user;
use crate::features::spaces::pages::actions::actions::follow::controllers::list_follow_users::FollowUserItem;
use crate::features::spaces::pages::actions::actions::follow::*;
pub mod i18n;
use i18n::FollowUserListTranslate;

#[component]
pub fn FollowUserList(
    space_id: SpacePartition,
    users: Vec<FollowUserItem>,
    can_delete: bool,
    on_refresh: EventHandler<()>,
    on_follow: EventHandler<Partition>,
    on_unfollow: EventHandler<Partition>,
    more_element: Element,
) -> Element {
    let tr: FollowUserListTranslate = use_translate();
    let list_empty = use_memo({
        let users = users.clone();
        move || users.is_empty()
    });

    rsx! {
        SpaceCard { class: "overflow-visible w-full shrink-0 rounded-[12px] !p-0".to_string(),
            div { class: "flex justify-between items-center self-stretch px-5 py-4 border-b border-separator",
                p { class: "font-semibold text-center sp-dash-font-raleway text-[17px]/[20px] tracking-[-0.18px] text-font-primary",
                    {tr.title}
                }
            }
            div { class: "flex flex-col items-start self-stretch p-5 gap-3 bg-card max-mobile:p-4",
                if list_empty() {
                    p { class: "font-medium leading-6 sp-dash-font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                        {tr.empty}
                    }
                } else {
                    for (idx , user) in users.iter().enumerate() {
                        {
                            let user_pk = user.user_pk.clone();
                            let is_creator = idx == 0;
                            let subscribed = user.subscribed;
                            let is_team = matches!(user.user_type, UserType::Team);
                            let on_refresh = on_refresh.clone();
                            let on_toggle_subscribe = {
                                let user_pk = user_pk.clone();
                                let on_follow = on_follow.clone();
                                let on_unfollow = on_unfollow.clone();
                                move |_| {
                                    if subscribed {
                                        on_unfollow.call(user_pk.clone());
                                    } else {
                                        on_follow.call(user_pk.clone());
                                    }
                                }
                            };
                            let on_delete = {
                                let space_id = space_id.clone();
                                let user_pk = user_pk.clone();
                                let on_refresh = on_refresh.clone();
                                move |_| {
                                    if is_creator {
                                        return;
                                    }
                                    let space_id = space_id.clone();
                                    let user_pk = user_pk.clone();
                                    let on_refresh = on_refresh.clone();
                                    spawn(async move {
                                        let _ = delete_follow_user(space_id, user_pk).await;
                                        on_refresh.call(());
                                    });
                                }
                            };

                            rsx! {
                                SpaceCard {
                                    class: "flex justify-between items-center w-full max-tablet:flex-col max-tablet:items-start max-tablet:gap-3 !rounded-[12px] !bg-card !px-4 !py-3"
                                        .to_string(),
                                    div { class: "flex items-center gap-[10px]",
                                        if !user.profile_url.is_empty() {
                                            img {
                                                src: "{user.profile_url}",
                                                alt: "{user.display_name}",
                                                class: if is_team { "object-cover object-top w-8 h-8 rounded-[8px]" } else { "object-cover object-top w-8 h-8 rounded-full" },
                                            }
                                        } else {
                                            div { class: if is_team { "w-8 h-8 rounded-[8px] bg-neutral-600" } else { "w-8 h-8 rounded-full bg-neutral-600" } }
                                        }
                                        div { class: "flex flex-col gap-1 items-start",
                                            div { class: "flex gap-2 items-center",
                                                p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                                                    "{user.display_name}"
                                                }
                                                if is_creator {
                                                    span { class: "text-[12px]/[16px] font-semibold px-2 py-0.5 rounded-full bg-btn-primary-bg text-btn-primary-text",
                                                        {tr.creator_badge}
                                                    }
                                                }
                                            }
                                            p { class: "font-semibold leading-4 sp-dash-font-raleway text-[13px] tracking-[-0.14px] text-web-font-neutral",
                                                "@{user.username}"
                                            }
                                        }
                                    }

                                    div { class: "flex items-center gap-2 max-tablet:w-full max-tablet:justify-end",
                                        Button {
                                            class: "min-w-[110px]",
                                            style: if subscribed { ButtonStyle::Secondary } else { ButtonStyle::Primary },
                                            onclick: on_toggle_subscribe,
                                            if subscribed {
                                                {tr.unsubscribe}
                                            } else {
                                                {tr.subscribe}
                                            }
                                        }
                                        if can_delete {
                                            Button {
                                                class: "min-w-[90px]",
                                                style: ButtonStyle::Outline,
                                                disabled: is_creator,
                                                onclick: on_delete,
                                                {tr.remove}
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
}
