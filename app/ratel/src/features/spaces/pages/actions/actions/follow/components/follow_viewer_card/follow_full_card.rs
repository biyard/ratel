use crate::features::spaces::pages::actions::actions::follow::components::follow_user_list::i18n::FollowUserListTranslate;
use crate::features::spaces::pages::actions::actions::follow::controllers::list_follow_users::FollowUserItem;
use crate::features::spaces::pages::actions::actions::follow::*;

translate! {
    FollowFullCardTranslate;

    suggested_for_you: {
        en: "Suggested for you",
        ko: "Suggested for you"
    }
    featured_profile: {
        en: "Featured profile",
        ko: "대표 프로필"
    }
}

#[component]
pub fn FollowFullCard(
    users: Vec<FollowUserItem>,
    on_follow: EventHandler<Partition>,
    on_unfollow: EventHandler<Partition>,
    more_element: Element,
) -> Element {
    let tr: FollowFullCardTranslate = use_translate();
    let list_tr: FollowUserListTranslate = use_translate();

    rsx! {
        SpaceCard {
            class: "w-full overflow-hidden rounded-3xl! border border-separator bg-card! px-0! py-0!"
                .to_string(),
            div { class: "flex flex-col",
                div { class: "px-6 pt-6 pb-3 max-mobile:px-5",
                    h3 { class: "text-[32px]/[38px] font-extrabold tracking-[-0.03em] text-text-primary max-mobile:text-[26px]/[32px]",
                        {tr.suggested_for_you}
                    }
                }

                div { class: "flex flex-col",
                    for (idx , user) in users.iter().enumerate() {
                        {
                            let is_creator = idx == 0;
                            let is_last = idx + 1 == users.len();
                            let is_team = matches!(user.user_type, UserType::Team);
                            let subscribed = user.subscribed;
                            let user_pk = user.user_pk.clone();
                            let description = user.description.clone();
                            let on_follow = on_follow.clone();
                            let on_unfollow = on_unfollow.clone();
                            let on_toggle_subscribe = move |_| {
                                if subscribed {
                                    on_unfollow.call(user_pk.clone());
                                } else {
                                    on_follow.call(user_pk.clone());
                                }
                            };

                            rsx! {
                                div {
                                    key: "{user.user_pk}",
                                    class: if is_last { "flex items-start gap-4 px-6 py-4 max-mobile:px-5" } else { "flex items-start gap-4 border-b border-separator px-6 py-4 max-mobile:px-5" },
                                    div { class: if is_team { "h-14 w-14 shrink-0 overflow-hidden rounded-[12px] bg-card" } else { "h-14 w-14 shrink-0 overflow-hidden rounded-full bg-card" },
                                        if !user.profile_url.is_empty() {
                                            img {
                                                src: "{user.profile_url}",
                                                alt: "{user.display_name}",
                                                class: "h-full w-full object-cover object-top",
                                            }
                                        } else {
                                            div { class: "h-full w-full bg-input-box-border" }
                                        }
                                    }
                                    div { class: "flex min-w-0 flex-1 flex-col gap-1",
                                        div { class: "flex items-start justify-between gap-4",
                                            div { class: "min-w-0",
                                                div { class: "flex min-w-0 flex-wrap items-center gap-2",
                                                    p { class: "truncate text-[18px]/[22px] font-bold text-text-primary",
                                                        "{user.display_name}"
                                                    }
                                                    if is_creator {
                                                        span { class: "rounded-full border border-separator px-2 py-0.5 text-[11px]/[14px] font-semibold text-foreground-muted",
                                                            {tr.featured_profile}
                                                        }
                                                    }
                                                }
                                                p { class: "truncate text-[16px]/[20px] font-medium text-foreground-muted",
                                                    "@{user.username}"
                                                }
                                            }
                                            Button {
                                                class: "min-w-26 rounded-full!",
                                                style: if subscribed { ButtonStyle::Secondary } else { ButtonStyle::Primary },
                                                onclick: on_toggle_subscribe,
                                                if subscribed {
                                                    {list_tr.subscribed}
                                                } else {
                                                    {list_tr.subscribe}
                                                }
                                            }
                                        }

                                        if !description.is_empty() {
                                            p { class: "text-[16px]/[22px] text-text-primary", {description} }
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
