use crate::features::spaces::pages::actions::actions::follow::controllers::{
    FollowUserItem, follow_user, list_follow_users,
};
use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::index::action_pages::quiz::CompletedActionCard;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::providers::use_space_context;

const DEFAULT_PROFILE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[component]
pub fn FollowActionCard(
    action: SpaceActionSummary,
    space_id: ReadSignal<SpacePartition>,
    #[props(default)] is_admin: bool,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let nav = use_navigator();
    let mut space_ctx = use_space_context();
    let action_id_edit = action.action_id.clone();

    let follow_id: SpaceActionFollowEntityType = action.action_id.clone().into();
    let mut follow_users =
        use_loader(move || async move { list_follow_users(space_id(), None).await })?;
    let users_response = follow_users();
    let users = users_response.items.clone();
    let total = users.len();
    let followed_count = users.iter().filter(|u| u.subscribed).count();
    let mut completed_action: CompletedActionCard = use_context();

    rsx! {
        div {
            class: "quest-card quest-card--follow",
            "data-type": "follow",
            "data-prerequisite": action.prerequisite,
            "data-testid": "quest-card-{action.action_id}",
            "data-credits": "{action.credits}",

            svg {
                class: "quest-card__hero",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "0.5",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                circle { cx: "9", cy: "7", r: "4" }
                line {
                    x1: "19",
                    x2: "19",
                    y1: "8",
                    y2: "14",
                }
                line {
                    x1: "22",
                    x2: "16",
                    y1: "11",
                    y2: "11",
                }
            }

            div { class: "quest-card__top",
                span { class: "quest-card__type quest-card__type--follow",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                        circle { cx: "9", cy: "7", r: "4" }
                        line {
                            x1: "19",
                            x2: "19",
                            y1: "8",
                            y2: "14",
                        }
                        line {
                            x1: "22",
                            x2: "16",
                            y1: "11",
                            y2: "11",
                        }
                    }
                    "{action.action_type.translate(&lang())}"
                }
                div { class: "quest-card__top-actions",
                    if action.prerequisite {
                        span { class: "quest-card__badge quest-card__badge--prerequisite",
                            "{tr.required_label}"
                        }
                    }
                    if is_admin {
                        QuestEditButton {
                            action_id: action.action_id.clone(),
                            on_edit: move |_| {
                                space_ctx.current_role.set(SpaceUserRole::Creator);
                                let follow_id: SpaceActionFollowEntityType = action_id_edit.clone().into();
                                nav.push(crate::Route::FollowActionPage {
                                    space_id: space_id(),
                                    follow_id,
                                });
                            },
                        }
                    }
                }
            }

            div { class: "quest-card__body",
                div { class: "quest-card__title", "{action.title}" }
                div { class: "quest-card__detail",
                    div { class: "quest-follow-list",
                        for user in users.iter() {
                            {
                                let action_id = action.action_id.clone();
                                // After this follow, check if all are now followed
                                let new_followed = followed_count + 1;
                                rsx! {
                                    FollowUserRow {
                                        key: "{user.user_pk}",
                                        user: user.clone(),
                                        space_id,
                                        follow_id: follow_id.clone(),
                                        credits_per_follow: if total > 0 { action.credits / total as u64 } else { 0 },
                                        on_followed: move |_| {
                                            follow_users.restart();
                                            if new_followed >= total {
                                                completed_action.0.set(Some(action_id.clone()));
                                            }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "quest-card__footer",
                div { class: "quest-card__reward",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        circle { cx: "12", cy: "12", r: "10" }
                        path { d: "M12 6v12" }
                        path { d: "M16 10H8" }
                    }
                    "{action.credits} CR"
                }
                span { class: "quest-card__follow-count",
                    "{followed_count} / {total} {tr.followed_label}"
                }
            }
        }
    }
}

#[component]
fn FollowUserRow(
    user: ReadSignal<FollowUserItem>,
    space_id: ReadSignal<SpacePartition>,
    follow_id: ReadSignal<SpaceActionFollowEntityType>,
    credits_per_follow: u64,
    on_followed: EventHandler<()>,
) -> Element {
    let profile = if user().profile_url.is_empty() {
        DEFAULT_PROFILE.to_string()
    } else {
        user().profile_url.clone()
    };
    let bio = if user().description.is_empty() {
        user().username.clone()
    } else {
        user().description.clone()
    };
    let display_name = user().display_name;
    let mut show_points = use_signal(|| false);

    rsx! {
        div { class: "quest-follow-user",
            img {
                class: "quest-follow-user__avatar",
                src: "{profile}",
                alt: "{display_name}",
            }
            div { class: "quest-follow-user__info",
                div { class: "quest-follow-user__name", "{display_name}" }
                div { class: "quest-follow-user__bio", "{bio}" }
            }
            if user().subscribed {
                div { class: "quest-follow-user__btn-wrap",
                    button {
                        class: "quest-follow-user__btn",
                        "data-followed": "true",
                        "Following"
                    }
                    if show_points() {
                        span { class: "points-anim", "+{credits_per_follow}" }
                    }
                }
            } else {
                button {
                    class: "quest-follow-user__btn",
                    onclick: move |_| async move {
                        let _ = follow_user(space_id(), follow_id(), user().user_pk).await;
                        show_points.set(true);
                        on_followed.call(());
                        crate::common::utils::time::sleep(std::time::Duration::from_secs(1)).await;
                        show_points.set(false);
                    },
                    "Follow"
                }
            }
        }
    }
}
