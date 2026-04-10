use crate::features::spaces::pages::actions::actions::follow::components::follow_user_list::i18n::FollowUserListTranslate;
use crate::features::spaces::pages::actions::actions::follow::controllers::list_follow_users::FollowUserItem;
use crate::features::spaces::pages::actions::actions::follow::*;

translate! {
    FollowSmallCardTranslate;

    suggested_profiles: {
        en: "Suggested Profiles",
        ko: "추천 프로필"
    }
    no_profiles: {
        en: "No profiles to follow.",
        ko: "팔로우할 프로필이 없습니다."
    }
    featured_profile: {
        en: "Featured profile",
        ko: "대표 프로필"
    }
}

#[component]
fn FollowSuggestionRow(
    user: FollowUserItem,
    on_follow: EventHandler<Partition>,
    on_unfollow: EventHandler<Partition>,
) -> Element {
    let list_tr: FollowUserListTranslate = use_translate();

    let is_team = matches!(user.user_type, UserType::Team);
    let subscribed = user.subscribed;
    let user_pk = user.user_pk.clone();
    let description = user.description.clone();
    let on_toggle_subscribe = move |_| {
        if subscribed {
            on_unfollow.call(user_pk.clone());
        } else {
            on_follow.call(user_pk.clone());
        }
    };

    rsx! {
        SpaceCard { class: "flex items-start gap-3 rounded-xl! border border-separator p-4!".to_string(),
            div { class: "flex min-w-0 flex-1 items-start gap-3",
                div { class: if is_team { "h-12 w-12 shrink-0 overflow-hidden rounded-[12px] bg-card" } else { "h-12 w-12 shrink-0 overflow-hidden rounded-full bg-card" },
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

                div { class: "min-w-0 flex-1",
                    p { class: "truncate text-[16px]/[20px] font-semibold text-text-primary",
                        "{user.display_name}"
                    }
                    p { class: "truncate text-[14px]/[18px] text-foreground-muted",
                        "@{user.username}"
                    }

                    if !description.is_empty() {
                        p { class: "mt-2 line-clamp-2 text-[14px]/[21px] text-text-secondary",
                            "{description}"
                        }
                    }
                }
            }

            div { class: "shrink-0",
                Button {
                    class: "min-w-24",
                    style: if subscribed { ButtonStyle::Secondary } else { ButtonStyle::Primary },
                    onclick: on_toggle_subscribe,
                    if subscribed {
                        {list_tr.subscribed}
                    } else {
                        {list_tr.subscribe}
                    }
                }
            }
        }
    }
}

#[component]
pub fn FollowSmallCard(
    users: Vec<FollowUserItem>,
    on_follow: EventHandler<Partition>,
    on_unfollow: EventHandler<Partition>,
    more_element: Element,
) -> Element {
    let tr: FollowSmallCardTranslate = use_translate();
    let list_tr: FollowUserListTranslate = use_translate();

    let Some(featured_user) = users.first().cloned() else {
        return rsx! {
            SpaceCard { class: "w-full rounded-[18px]! px-5! py-6!",
                p { class: "text-sm font-medium text-foreground-muted", {tr.no_profiles} }
            }
        };
    };

    let rest_users = users.iter().skip(1).cloned().collect::<Vec<_>>();
    let desktop_rest_users = rest_users.clone();
    let mobile_rest_users = rest_users;
    let desktop_more_element = more_element.clone();
    let mobile_more_element = more_element;

    let featured_is_team = matches!(featured_user.user_type, UserType::Team);
    let featured_subscribed = featured_user.subscribed;
    let featured_description = featured_user.description.clone();

    let desktop_featured_user_pk = featured_user.user_pk.clone();
    let desktop_on_follow = on_follow.clone();
    let desktop_on_unfollow = on_unfollow.clone();
    let desktop_on_toggle_featured = move |_| {
        if featured_subscribed {
            desktop_on_unfollow.call(desktop_featured_user_pk.clone());
        } else {
            desktop_on_follow.call(desktop_featured_user_pk.clone());
        }
    };

    let mobile_featured_user_pk = featured_user.user_pk.clone();
    let mobile_on_follow = on_follow.clone();
    let mobile_on_unfollow = on_unfollow.clone();
    let mobile_on_toggle_featured = move |_| {
        if featured_subscribed {
            mobile_on_unfollow.call(mobile_featured_user_pk.clone());
        } else {
            mobile_on_follow.call(mobile_featured_user_pk.clone());
        }
    };

    rsx! {
        div { class: "hidden w-full desktop:grid desktop:grid-cols-[280px_minmax(0,1fr)] desktop:items-start desktop:gap-6",
            div { class: "flex flex-col gap-4",
                div { class: if featured_is_team { "aspect-square w-full max-w-[240px] overflow-hidden rounded-[24px] border border-separator bg-card" } else { "aspect-square w-full max-w-[240px] overflow-hidden rounded-full border border-separator bg-card" },
                    if !featured_user.profile_url.is_empty() {
                        img {
                            src: "{featured_user.profile_url}",
                            alt: "{featured_user.display_name}",
                            class: "h-full w-full object-cover object-top",
                        }
                    } else {
                        div { class: "h-full w-full bg-input-box-border" }
                    }
                }

                div { class: "flex min-w-0 flex-col gap-1",
                    div { class: "flex min-w-0 items-center gap-2",
                        p { class: "min-w-0 truncate text-[30px]/[36px] font-semibold tracking-[-0.03em] text-text-primary",
                            "{featured_user.display_name}"
                        }
                        span { class: "shrink-0 rounded-full border border-separator px-2 py-0.5 text-[11px]/[14px] font-semibold text-foreground-muted",
                            {tr.featured_profile}
                        }
                    }
                    p { class: "text-[16px]/[22px] text-foreground-muted",
                        "@{featured_user.username}"
                    }
                }

                Button {
                    class: "w-full max-w-60",
                    style: if featured_subscribed { ButtonStyle::Secondary } else { ButtonStyle::Outline },
                    onclick: desktop_on_toggle_featured,
                    if featured_subscribed {
                        {list_tr.subscribed}
                    } else {
                        {list_tr.subscribe}
                    }
                }

                if !featured_description.is_empty() {
                    p { class: "w-full max-w-60 text-[15px]/[22px] text-text-secondary",
                        "{featured_description}"
                    }
                }
            }

            div { class: "flex flex-col gap-4",
                p { class: "text-[20px]/[26px] font-semibold text-text-primary",
                    {tr.suggested_profiles}
                }

                if desktop_rest_users.is_empty() {
                    p { class: "text-sm font-medium text-foreground-muted", {tr.no_profiles} }
                } else {
                    div { class: "grid grid-cols-2 gap-4",
                        for user in desktop_rest_users.into_iter() {
                            FollowSuggestionRow {
                                key: "{user.user_pk}",
                                user,
                                on_follow: on_follow.clone(),
                                on_unfollow: on_unfollow.clone(),
                            }
                        }
                    }
                }

                {desktop_more_element}
            }
        }

        div { class: "flex w-full flex-col gap-4 desktop:hidden",
            SpaceCard { class: "flex flex-col gap-4 rounded-[18px]! p-5!",
                div { class: "flex items-center gap-4",
                    div { class: if featured_is_team { "aspect-square w-full max-w-[96px] shrink-0 overflow-hidden rounded-[20px] border border-separator bg-card" } else { "aspect-square w-full max-w-[96px] shrink-0 overflow-hidden rounded-full border border-separator bg-card" },
                        if !featured_user.profile_url.is_empty() {
                            img {
                                src: "{featured_user.profile_url}",
                                alt: "{featured_user.display_name}",
                                class: "h-full w-full object-cover object-top",
                            }
                        } else {
                            div { class: "h-full w-full bg-input-box-border" }
                        }
                    }

                    div { class: "min-w-0 flex-1",
                        div { class: "flex min-w-0 items-center gap-2",
                            p { class: "min-w-0 truncate text-[28px]/[34px] font-semibold tracking-[-0.03em] text-text-primary",
                                "{featured_user.display_name}"
                            }
                            span { class: "shrink-0 rounded-full border border-separator px-2 py-0.5 text-[11px]/[14px] font-semibold text-foreground-muted",
                                {tr.featured_profile}
                            }
                        }
                        p { class: "text-[16px]/[22px] text-foreground-muted",
                            "@{featured_user.username}"
                        }
                    }
                }

                if !featured_description.is_empty() {
                    p { class: "text-[15px]/[22px] text-text-secondary", "{featured_description}" }
                }

                Button {
                    class: "w-full",
                    style: if featured_subscribed { ButtonStyle::Secondary } else { ButtonStyle::Outline },
                    onclick: mobile_on_toggle_featured,
                    if featured_subscribed {
                        {list_tr.subscribed}
                    } else {
                        {list_tr.subscribe}
                    }
                }
            }

            SpaceCard { class: "flex flex-col gap-4 rounded-[18px]! p-5!",
                p { class: "text-[20px]/[26px] font-semibold text-text-primary",
                    {tr.suggested_profiles}
                }

                if mobile_rest_users.is_empty() {
                    p { class: "text-sm font-medium text-foreground-muted", {tr.no_profiles} }
                } else {
                    div { class: "flex flex-col gap-3",
                        for user in mobile_rest_users.into_iter() {
                            FollowSuggestionRow {
                                key: "{user.user_pk}",
                                user,
                                on_follow: on_follow.clone(),
                                on_unfollow: on_unfollow.clone(),
                            }
                        }
                    }
                }

                {mobile_more_element}
            }
        }
    }
}
