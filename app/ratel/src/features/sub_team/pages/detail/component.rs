//! Sub-team detail (activity dashboard) page. Consumes
//! `UseSubTeamActivity` which bundles overview/counts/per-member rows.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::types::ActivityWindow;
use crate::features::sub_team::{
    use_sub_team_activity, SubTeamTranslate, UseSubTeamActivity,
};
use crate::route::Route;
use crate::*;

#[component]
pub fn TeamSubTeamDetailPage(username: String, sub_team_id: String) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_pk = team_resource().pk;
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    use_context_provider(|| team_id);
    // sub_team_id seeded via context for the hook.
    let sub_team_id_for_ctx = sub_team_id.clone();
    use_context_provider(move || sub_team_id_for_ctx.clone());

    rsx! {
        SeoMeta { title: "Sub-team · {tr.tab_sub_teams}" }

        DetailView { username: username.clone(), sub_team_id: sub_team_id.clone() }
    }
}

#[component]
fn DetailView(username: String, sub_team_id: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let UseSubTeamActivity {
        mut window,
        detail,
        counts,
        mut members,
        ..
    } = use_sub_team_activity()?;

    // Overview (detail)
    let overview = detail();
    let _counts_data = counts();

    let display_name = overview.display_name.clone();
    let handle = overview.username.clone();
    let privacy_notice_ko = overview.privacy_notice.ko.clone();

    let post_count = overview.post_count;
    let space_count = overview.space_count;
    let active_members = overview.active_member_count;

    let deregister_username = username.clone();
    let deregister_sub_team = sub_team_id.clone();
    let initials: String = display_name.chars().take(2).collect::<String>().to_uppercase();

    let member_rows = members.items();

    rsx! {
        div { class: "arena sub-team-detail",
            div { class: "page page--wide",

                // Team hero
                div { class: "team-hero",
                    div { class: "avatar avatar--lg avatar--teal", "{initials}" }
                    div {
                        div { class: "team-hero__title", "{display_name}" }
                        div { class: "team-hero__handle", "@{handle}" }
                    }
                    div { class: "window-toggle", role: "tablist",
                        button {
                            class: "window-toggle__btn",
                            role: "tab",
                            "aria-selected": "{window() == ActivityWindow::Weekly}",
                            onclick: move |_| window.set(ActivityWindow::Weekly),
                            "{tr.window_weekly}"
                        }
                        button {
                            class: "window-toggle__btn",
                            role: "tab",
                            "aria-selected": "{window() == ActivityWindow::Monthly}",
                            onclick: move |_| window.set(ActivityWindow::Monthly),
                            "{tr.window_monthly}"
                        }
                    }
                }

                // Metrics
                div { class: "metric-grid",
                    div { class: "metric metric--posts",
                        div { class: "metric__label",
                            lucide_dioxus::FileText { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.post_count}"
                        }
                        div {
                            class: "metric__value",
                            id: "m-posts",
                            "data-testid": "sub-team-detail-post-count",
                            "{post_count}"
                        }
                    }
                    div { class: "metric metric--spaces",
                        div { class: "metric__label",
                            lucide_dioxus::Hash { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.space_count}"
                        }
                        div {
                            class: "metric__value",
                            id: "m-spaces",
                            "data-testid": "sub-team-detail-space-count",
                            "{space_count}"
                        }
                    }
                    div { class: "metric metric--members",
                        div { class: "metric__label",
                            lucide_dioxus::Users { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.active_member_count}"
                        }
                        div {
                            class: "metric__value",
                            id: "m-members",
                            "data-testid": "sub-team-detail-active-members",
                            "{active_members}"
                        }
                    }
                }

                // Privacy notice
                div {
                    class: "notice notice--teal",
                    "data-testid": "sub-team-detail-privacy-notice",
                    div { class: "notice__icon",
                        lucide_dioxus::Lock { class: "w-4 h-4 [&>path]:stroke-current" }
                    }
                    div { class: "notice__body",
                        span { class: "notice__title", "{tr.privacy_notice_short}" }
                        span { class: "notice__text", "{privacy_notice_ko}" }
                    }
                }

                // Member activity table
                section { class: "card",
                    div { class: "card__head",
                        h2 { class: "card__title", "Per-member activity" }
                        span { class: "card__dash" }
                        span { class: "card__meta", "{member_rows.len()}" }
                    }
                    if member_rows.is_empty() {
                        div { class: "inline-note", "{tr.empty_list}" }
                    } else {
                        table { class: "member-table",
                            thead {
                                tr {
                                    th { "Handle" }
                                    th { "Posts" }
                                    th { "Spaces" }
                                    th { "Last active" }
                                }
                            }
                            tbody { id: "member-rows",
                                for m in member_rows.iter() {
                                    tr { key: "{m.user_id}",
                                        td {
                                            span { class: "member-handle",
                                                span { class: "avatar avatar--sm avatar--teal",
                                                    {m.display_name.chars().take(2).collect::<String>().to_uppercase()}
                                                }
                                                span { class: "member-handle__info",
                                                    span { class: "member-handle__name",
                                                        "@{m.handle}"
                                                    }
                                                    span { class: "member-handle__role",
                                                        "{m.display_name}"
                                                    }
                                                }
                                            }
                                        }
                                        td {
                                            span { class: "member-metric", "{m.post_count}" }
                                        }
                                        td {
                                            span { class: "member-metric",
                                                "{m.space_count_participated}"
                                            }
                                        }
                                        td {
                                            if let Some(ts) = m.last_active_at {
                                                "{ts}"
                                            } else {
                                                "—"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {members.more_element()}
                }

                // Danger zone (deregister)
                section { class: "card",
                    div { class: "card__head",
                        h2 {
                            class: "card__title",
                            style: "color:var(--sub-team-coral,#ef4444)",
                            "Danger zone"
                        }
                        span { class: "card__dash" }
                    }
                    div { class: "danger-zone",
                        div { class: "danger-zone__icon",
                            lucide_dioxus::TriangleAlert { class: "w-4 h-4 [&>path]:stroke-current" }
                        }
                        div { class: "danger-zone__body",
                            div { class: "danger-zone__title", "{tr.deregister_title}" }
                        }
                        a {
                            class: "btn btn--danger",
                            onclick: move |_| {
                                nav.push(Route::TeamSubTeamDeregisterPage {
                                    username: deregister_username.clone(),
                                    sub_team_id: deregister_sub_team.clone(),
                                });
                            },
                            lucide_dioxus::X { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.deregister_confirm}"
                        }
                    }
                }
            }
        }
    }
}
