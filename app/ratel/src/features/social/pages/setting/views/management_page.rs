use crate::features::social::pages::member::controllers::{
    get_team_member_permission_handler, list_members_handler,
};
use crate::features::social::pages::member::dto::TeamMemberResponse;
use crate::features::social::pages::group::components::{InviteMemberModal, InviteResult};
use crate::features::social::pages::group::controllers::{list_groups_handler, remove_member_handler};
use crate::features::social::pages::group::dto::RemoveMemberRequest;
use crate::features::social::pages::setting::i18n::TeamSettingsTranslate;
use crate::features::social::*;
use dioxus::prelude::*;

const PAGE_SIZE: i32 = 10;

#[component]
pub fn ManagementPage(username: String) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    use_context_provider(|| PopupService::new());
    let mut popup = use_popup();


    let ctx_resource = use_loader(use_reactive((&username,), |(name,)| async move {
        Ok::<_, super::super::Error>(
            get_team_member_permission_handler(name)
                .await
                .map_err(|e| e.to_string()),
        )
    }))?;

    let ctx = ctx_resource.read();
    let ctx = match ctx.as_ref() {
        Ok(ctx) => ctx.clone(),
        Err(_) => {
            return rsx! {
                div { class: "flex flex-col gap-2 p-4",
                    span { class: "text-sm font-semibold text-text-primary", {tr.no_permission_title} }
                    span { class: "text-sm text-foreground-muted", {tr.no_permission_description} }
                }
            };
        }
    };

    let team_pk = ctx.team_pk.clone();

    let mut refresh = use_signal(|| 0u64);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let failed_remove_member = tr.failed_remove_member.to_string();

    // Load groups for invite modal
    let group_resource = use_loader(use_reactive((&team_pk,), move |(team_pk,)| {
        let _ = refresh();
        async move {
            Ok::<_, super::super::Error>(
                list_groups_handler(team_pk, None)
                    .await
                    .map_err(|e| e.to_string()),
            )
        }
    }))?;

    let groups = {
        let data = group_resource.read();
        match data.as_ref() {
            Ok(list) => list.items.clone(),
            Err(_) => vec![],
        }
    };

    let on_add_members_click = {
        let mut popup = popup;
        let team_pk = team_pk.clone();
        let username = username.clone();
        let groups = groups.clone();
        let mut refresh = refresh.clone();
        move |_| {
            let on_close = { let mut popup = popup; move |_| { popup.close(); } };
            let on_invited = {
                let mut refresh = refresh.clone();
                move |result: InviteResult| {
                    if result.total_added > 0 {
                        refresh.set(refresh() + 1);
                    }
                }
            };
            popup.open(rsx! {
                InviteMemberModal {
                    team_pk: team_pk.clone(),
                    username: username.clone(),
                    groups: groups.clone(),
                    on_close,
                    on_invited,
                }
            });
        }
    };

    // page_cursors[i] = bookmark to fetch page i (None for first page)
    let mut page_cursors: Signal<Vec<Option<String>>> = use_signal(|| vec![None]);
    let mut current_page = use_signal(|| 0usize); // 0-indexed

    let member_resource = use_loader(use_reactive(
        (&team_pk, &current_page, &refresh),
        move |(team_pk, page, _refresh)| {
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
    if let Some(ref bm) = next_bookmark {
        let page = current_page();
        if page_cursors.read().len() <= page + 1 {
            page_cursors.write().push(Some(bm.clone()));
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
                h1 { class: "text-xl font-bold text-text-primary", {tr.team_management} }
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Rounded,
                    size: ButtonSize::Small,
                    class: "flex items-center gap-2".to_string(),
                    onclick: on_add_members_click,
                    lucide_dioxus::UserPlus {
                        class: "w-4 h-4 [&>path]:stroke-btn-primary-text [&>line]:stroke-btn-primary-text",
                    }
                    {tr.add_members}
                }
            }

            // Error message
            if let Some(msg) = error_msg() {
                div { class: "px-4 py-3 rounded-[10px] border border-destructive bg-destructive/10 text-sm text-destructive",
                    "{msg}"
                }
            }

            // Members list
            div { class: "flex flex-col rounded-[10px] border border-border",
                // Section header
                div { class: "px-4 py-3 border-b border-border",
                    span { class: "text-sm font-semibold text-text-primary",
                        "{tr.members} ({members.len()})"
                    }
                }

                // Rows
                for (idx, member) in members.iter().enumerate() {
                    {
                        let is_last = idx == members.len() - 1;
                        let member = member.clone();
                        let team_pk = team_pk.clone();
                        let mut refresh = refresh.clone();
                        let failed_remove_member = failed_remove_member.clone();
                        rsx! {
                            MemberRow {
                                key: "{member.user_id}",
                                member: member.clone(),
                                is_last,
                                on_remove: move |_| {
                                    let member = member.clone();
                                    let team_pk = team_pk.clone();
                                    let mut refresh = refresh.clone();
                                    let mut error_msg = error_msg.clone();
                                    let failed_msg = failed_remove_member.clone();
                                    spawn(async move {
                                        let mut failed = false;
                                        for group in &member.groups {
                                            if remove_member_handler(
                                                team_pk.clone(),
                                                group.group_id.clone(),
                                                RemoveMemberRequest {
                                                    user_pks: vec![member.user_id.clone()],
                                                },
                                            )
                                            .await
                                            .is_err()
                                            {
                                                failed = true;
                                            }
                                        }
                                        if failed {
                                            error_msg.set(Some(failed_msg));
                                        } else {
                                            error_msg.set(None);
                                            refresh.set(refresh() + 1);
                                        }
                                    });
                                },
                            }
                        }
                    }
                }

                if members.is_empty() {
                    div { class: "px-4 py-8 text-center text-sm text-foreground-muted",
                        {tr.no_members}
                    }
                }
            }

            // Pagination
            if total_pages > 1 || can_prev {
                div { class: "flex items-center justify-center gap-1",
                    Button {
                        style: ButtonStyle::Text,
                        size: ButtonSize::Icon,
                        shape: ButtonShape::Square,
                        class: "flex items-center justify-center w-8 h-8 text-foreground-muted disabled:opacity-30 disabled:cursor-not-allowed".to_string(),
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
                            rsx! {
                                Button {
                                    key: "{p}",
                                    style: ButtonStyle::Text,
                                    size: ButtonSize::Icon,
                                    shape: ButtonShape::Square,
                                    class: "w-8 h-8 text-sm text-foreground-muted aria-selected:bg-btn-secondary-bg aria-selected:text-btn-secondary-text aria-selected:font-semibold".to_string(),
                                    "aria-selected": is_active,
                                    onclick: move |_| current_page.set(p),
                                    "{p + 1}"
                                }
                            }
                        }
                    }

                    Button {
                        style: ButtonStyle::Text,
                        size: ButtonSize::Icon,
                        shape: ButtonShape::Square,
                        class: "flex items-center justify-center w-8 h-8 text-foreground-muted disabled:opacity-30 disabled:cursor-not-allowed".to_string(),
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
        PopupZone {}
    }
}

#[component]
fn MemberRow(member: TeamMemberResponse, is_last: bool, on_remove: EventHandler<()>) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    let border_class = if is_last { "" } else { "border-b border-border" };
    let mut show_menu = use_signal(|| false);

    let display = if member.display_name.is_empty() {
        member.username.clone()
    } else {
        member.display_name.clone()
    };

    rsx! {
        div { class: "relative flex items-center gap-3 px-4 py-3 {border_class}",
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
                span { class: "text-sm text-foreground-muted shrink-0", {tr.owner} }
            } else {
                span { class: "text-sm text-foreground-muted shrink-0", {tr.member_role} }
            }

            // More options
            if !member.is_owner {
                div { class: "relative shrink-0",
                    Button {
                        style: ButtonStyle::Text,
                        size: ButtonSize::Icon,
                        shape: ButtonShape::Square,
                        class: "flex items-center justify-center w-7 h-7 !rounded-md".to_string(),
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            show_menu.toggle();
                        },
                        lucide_dioxus::Ellipsis {
                            class: "w-4 h-4 [&>circle]:fill-text-primary [&>circle]:stroke-none",
                        }
                    }
                    if show_menu() {
                        div {
                            class: "fixed inset-0 z-10",
                            onclick: move |_| show_menu.set(false),
                        }
                        div {
                            class: "absolute right-0 top-8 z-20 w-44 bg-popover border border-border rounded-lg shadow-lg py-1 overflow-hidden",
                            onclick: move |e| e.stop_propagation(),
                            Button {
                                style: ButtonStyle::Text,
                                size: ButtonSize::Small,
                                shape: ButtonShape::Square,
                                class: "flex items-center gap-2 w-full text-destructive justify-start".to_string(),
                                onclick: move |_| {
                                    show_menu.set(false);
                                    on_remove.call(());
                                },
                                lucide_dioxus::UserMinus {
                                    class: "w-4 h-4 [&>path]:stroke-destructive [&>line]:stroke-destructive",
                                }
                                {tr.remove_from_team}
                            }
                        }
                    }
                }
            } else {
                div { class: "w-7 shrink-0" }
            }
        }
    }
}
