use crate::*;

#[component]
pub fn TeamSelector(
    selected_label: String,
    user_display_name: String,
    user_profile_url: String,
    user_href: String,
    teams: Vec<TeamItem>,
    team_href_prefix: String,
    team_href_suffix: String,
    #[props(optional)] on_select_team: Option<EventHandler<usize>>,
    #[props(optional)] on_create_team: Option<EventHandler<String>>,
    #[props(optional)] on_logout: Option<EventHandler<()>>,
) -> Element {
    let tr: TeamSelectorTranslate = use_translate();
    let mut open = use_signal(|| false);

    let resolved_label = if selected_label.trim().is_empty() {
        user_display_name.clone()
    } else {
        selected_label
    };

    let prefix = {
        let trimmed = team_href_prefix.trim_end_matches('/');
        if trimmed.is_empty() {
            String::new()
        } else if trimmed.starts_with('/') {
            trimmed.to_string()
        } else {
            format!("/{}", trimmed)
        }
    };

    let suffix = if team_href_suffix.is_empty() {
        String::new()
    } else if team_href_suffix.starts_with('/') {
        team_href_suffix
    } else {
        format!("/{}", team_href_suffix)
    };

    rsx! {
        div { class: "relative",
            // Trigger button - shows current user/team name with chevron
            button {
                class: "w-full flex items-center justify-between px-2 py-2 focus:outline-none cursor-pointer",
                onclick: move |_| {
                    open.set(!open());
                },
                span { class: "font-bold text-[18px] text-text-primary truncate", "{resolved_label}" }
                icons::arrows::ChevronDown {
                    width: "16",
                    height: "16",
                    class: "[&>path]:stroke-text-primary",
                }
            }

            if open() {
                // Backdrop overlay to close on click outside
                div {
                    class: "fixed inset-0 z-998",
                    onclick: move |_| {
                        open.set(false);
                    },
                }

                div { class: "absolute left-0 top-full w-full min-w-[200px] rounded-lg border border-divider bg-background p-2 z-999",
                    // Teams label
                    div { class: "text-xs text-text-primary px-2 py-1 font-medium",
                        "{tr.teams}"
                    }

                    // Scrollable team list
                    div { class: "max-h-[300px] overflow-y-auto pr-1 -mr-1",
                        // User entry (personal profile)
                        Link {
                            class: "flex items-center gap-2 w-full px-2 py-2 hover:bg-hover rounded-md cursor-pointer",
                            to: user_href,
                            onclick: move |_| {
                                open.set(false);
                            },
                            if !user_profile_url.is_empty() {
                                img {
                                    src: "{user_profile_url}",
                                    alt: "{user_display_name}",
                                    class: "w-6 h-6 rounded-full object-cover object-top",
                                }
                            } else {
                                div { class: "w-6 h-6 rounded-full border border-neutral-600 bg-neutral-600" }
                            }
                            span { class: "text-sm text-text-primary truncate", "{user_display_name}" }
                        }

                        // Team entries
                        for (idx , team) in teams.iter().enumerate() {
                            {render_team_row(idx, team, &prefix, &suffix, on_select_team.clone(), open)}
                        }
                    }

                    // Separator
                    if on_create_team.is_some() || on_logout.is_some() {
                        div { class: "my-1.5 bg-divider h-px" }
                    }

                    if let Some(handler) = on_create_team {
                        button {
                            class: "w-full flex items-center gap-2 px-2 py-2 hover:bg-hover rounded-md text-sm text-text-primary cursor-pointer text-left",
                            onclick: move |_| {
                                open.set(false);
                                handler.call(tr.create_team.to_string());
                            },
                            "{tr.create_team}"
                        }
                    }

                    if let Some(handler) = on_logout {
                        button {
                            class: "w-full flex items-center gap-2 px-2 py-2 hover:bg-hover rounded-md text-sm text-text-primary cursor-pointer text-left",
                            onclick: move |_| {
                                open.set(false);
                                handler.call(());
                            },
                            "{tr.logout}"
                        }
                    }
                }
            }
        }
    }
}

fn team_label(team: &TeamItem) -> &str {
    if team.nickname.is_empty() {
        &team.username
    } else {
        &team.nickname
    }
}

fn team_href(prefix: &str, suffix: &str, username: &str) -> String {
    if prefix.is_empty() {
        format!("/{}{}", username, suffix)
    } else {
        format!("{}/{}{}", prefix, username, suffix)
    }
}

fn render_team_row(
    idx: usize,
    team: &TeamItem,
    prefix: &str,
    suffix: &str,
    on_select_team: Option<EventHandler<usize>>,
    mut open: Signal<bool>,
) -> Element {
    let label = team_label(team);
    let href = team_href(prefix, suffix, &team.username);
    rsx! {
        Link {
            class: "flex items-center gap-2 w-full px-2 py-2 hover:bg-hover rounded-md cursor-pointer",
            to: href,
            onclick: move |_| {
                open.set(false);
                if let Some(handler) = &on_select_team {
                    handler.call(idx);
                }
            },
            if !team.profile_url.is_empty() {
                img {
                    src: "{team.profile_url}",
                    alt: "{label}",
                    class: "w-6 h-6 rounded-full object-cover object-top",
                }
            } else {
                div { class: "w-6 h-6 rounded-full border border-neutral-600 bg-neutral-600" }
            }
            span { class: "text-sm text-text-primary truncate", "{label}" }
        }
    }
}

translate! {
    TeamSelectorTranslate;

    teams: {
        en: "Teams",
        ko: "팀",
    },

    create_team: {
        en: "Create Team",
        ko: "팀 생성",
    },

    logout: {
        en: "Log Out",
        ko: "로그아웃",
    },
}
