use crate::features::teams::pages::member::controllers::{
    get_team_member_permission_handler, list_members_handler,
};
use crate::features::teams::pages::member::dto::TeamMemberResponse;
use crate::features::teams::*;
use dioxus::prelude::*;

const PAGE_SIZE: i32 = 10;

#[component]
pub fn ManagementPage(teamname: String) -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let current_user_pk = user_ctx()
        .user
        .as_ref()
        .map(|u| u.pk.to_string())
        .unwrap_or_default();

    let ctx_resource = use_loader(use_reactive((&teamname,), |(name,)| async move {
        Ok::<_, super::super::Error>(
            get_team_member_permission_handler(name)
                .await
                .map_err(|e| e.to_string()),
        )
    }))?;

    let ctx = ctx_resource.read();
    let Ok(ctx) = ctx.as_ref() else {
        return rsx! { div { class: "text-foreground-muted text-sm p-4", "Loading..." } };
    };

    let team_pk = ctx.team_pk.clone();

    // page_cursors[i] = bookmark to fetch page i (None for first page)
    let mut page_cursors: Signal<Vec<Option<String>>> = use_signal(|| vec![None]);
    let mut current_page = use_signal(|| 0usize); // 0-indexed

    let member_resource = use_loader(use_reactive(
        (&team_pk, &current_page),
        move |(team_pk, page)| {
            let cursor = page_cursors.read().get(page()).cloned().flatten();
            async move {
                Ok::<_, super::super::Error>(
                    list_members_handler(team_pk, cursor, Some(PAGE_SIZE))
                        .await
                        .map_err(|e| e.to_string()),
                )
            }
        },
    ))?;

    let data = member_resource.read();
    let (members, next_bookmark) = match data.as_ref() {
        Ok(list) => (list.items.clone(), list.bookmark.clone()),
        Err(_) => (vec![], None),
    };

    // Store next page cursor when it arrives
    {
        let page = current_page();
        let mut cursors = page_cursors.write();
        if let Some(ref bm) = next_bookmark {
            if cursors.len() <= page + 1 {
                cursors.push(Some(bm.clone()));
            }
        }
    }

    let total_pages = {
        let cursors = page_cursors.read();
        if next_bookmark.is_some() {
            cursors.len()
        } else {
            current_page() + 1
        }
    };

    let can_prev = current_page() > 0;
    let can_next = next_bookmark.is_some();

    rsx! {
        div { class: "flex flex-col gap-6 w-full max-w-2xl",
            // Header
            div { class: "flex items-center justify-between",
                h1 { class: "text-xl font-bold text-text-primary", "Team management" }
                button {
                    class: "flex items-center gap-2 px-4 py-2 rounded-full bg-white text-neutral-900 text-sm font-medium hover:bg-neutral-200 transition-colors cursor-pointer",
                    lucide_dioxus::UserPlus {
                        class: "w-4 h-4 [&>path]:stroke-neutral-900 [&>line]:stroke-neutral-900",
                    }
                    "Add members"
                }
            }

            // Members list
            div { class: "flex flex-col rounded-[10px] border border-border overflow-hidden",
                // Section header
                div { class: "px-4 py-3 border-b border-border",
                    span { class: "text-sm font-semibold text-text-primary",
                        "Members ({members.len()})"
                    }
                }

                // Rows
                for (idx, member) in members.iter().enumerate() {
                    {
                        let is_last = idx == members.len() - 1;
                        let is_you = member.user_id == current_user_pk;
                        let member = member.clone();
                        rsx! {
                            MemberRow {
                                key: "{member.user_id}",
                                member,
                                is_you,
                                is_last,
                            }
                        }
                    }
                }

                if members.is_empty() {
                    div { class: "px-4 py-8 text-center text-sm text-foreground-muted",
                        "No members found."
                    }
                }
            }

            // Pagination
            if total_pages > 1 || can_prev {
                div { class: "flex items-center justify-center gap-1",
                    button {
                        class: "flex items-center justify-center w-8 h-8 rounded-lg transition-colors text-foreground-muted hover:bg-white/5 disabled:opacity-30 disabled:cursor-not-allowed",
                        disabled: !can_prev,
                        onclick: move |_| {
                            if can_prev {
                                current_page.set(current_page() - 1);
                            }
                        },
                        lucide_dioxus::ChevronLeft {
                            class: "w-4 h-4 [&>polyline]:stroke-current",
                        }
                    }

                    for p in 0..total_pages {
                        {
                            let is_active = p == current_page();
                            let active_class = if is_active {
                                "bg-white text-neutral-900 font-semibold"
                            } else {
                                "text-foreground-muted hover:bg-white/5"
                            };
                            rsx! {
                                button {
                                    key: "{p}",
                                    class: "flex items-center justify-center w-8 h-8 rounded-lg text-sm transition-colors {active_class}",
                                    onclick: move |_| current_page.set(p),
                                    "{p + 1}"
                                }
                            }
                        }
                    }

                    button {
                        class: "flex items-center justify-center w-8 h-8 rounded-lg transition-colors text-foreground-muted hover:bg-white/5 disabled:opacity-30 disabled:cursor-not-allowed",
                        disabled: !can_next,
                        onclick: move |_| {
                            if can_next {
                                current_page.set(current_page() + 1);
                            }
                        },
                        lucide_dioxus::ChevronRight {
                            class: "w-4 h-4 [&>polyline]:stroke-current",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MemberRow(member: TeamMemberResponse, is_you: bool, is_last: bool) -> Element {
    let border_class = if is_last { "" } else { "border-b border-border" };
    let role_label = if member.is_owner { "Owner" } else { "Member" };

    let display = if member.display_name.is_empty() {
        member.username.clone()
    } else {
        member.display_name.clone()
    };

    rsx! {
        div { class: "flex items-center gap-3 px-4 py-3 {border_class}",
            // Avatar
            if !member.profile_url.is_empty() {
                img {
                    src: "{member.profile_url}",
                    alt: "{display}",
                    class: "w-9 h-9 rounded-full object-cover shrink-0",
                }
            } else {
                div { class: "w-9 h-9 rounded-full bg-neutral-600 shrink-0 flex items-center justify-center",
                    span { class: "text-xs font-semibold text-white",
                        "{display.chars().next().unwrap_or('?').to_uppercase()}"
                    }
                }
            }

            // Name + username
            div { class: "flex flex-col min-w-0 flex-1",
                span { class: "text-sm font-semibold text-text-primary truncate", "{display}" }
                span { class: "text-xs text-foreground-muted truncate", "@{member.username}" }
            }

            // Role
            if member.is_owner {
                span { class: "text-sm text-foreground-muted shrink-0", "Owner" }
            } else {
                button {
                    class: "flex items-center gap-1 text-sm text-foreground-muted hover:text-text-primary transition-colors shrink-0 cursor-pointer",
                    "Member"
                    lucide_dioxus::ChevronDown {
                        class: "w-3.5 h-3.5 [&>polyline]:stroke-current",
                    }
                }
            }

            // "You" badge
            if is_you {
                span { class: "text-xs text-foreground-muted shrink-0", "You" }
            }

            // More options
            if !member.is_owner {
                button {
                    class: "flex items-center justify-center w-7 h-7 rounded-md hover:bg-white/10 transition-colors shrink-0 cursor-pointer",
                    lucide_dioxus::Ellipsis {
                        class: "w-4 h-4 [&>circle]:fill-foreground-muted [&>circle]:stroke-none",
                    }
                }
            } else {
                div { class: "w-7 shrink-0" }
            }
        }
    }
}
