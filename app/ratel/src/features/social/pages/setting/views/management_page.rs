use crate::features::social::pages::member::controllers::{
    get_team_member_permission_handler, list_members_handler,
};
use crate::features::social::pages::member::dto::TeamMemberResponse;
use crate::features::social::pages::group::components::{InviteMemberModal, InviteResult};
use crate::features::social::pages::group::controllers::{
    add_member_handler, list_groups_handler, remove_member_handler,
};
use crate::features::social::pages::group::dto::{AddMemberRequest, RemoveMemberRequest};
use crate::features::social::pages::setting::i18n::TeamSettingsTranslate;
use crate::features::social::*;
use crate::common::hooks::use_infinite_query;
use crate::common::ListResponse;
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;

const PAGE_SIZE: i32 = 9;

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
    let failed_change_group = tr.failed_change_group.to_string();

    // Load groups for invite modal and change group feature
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

    let all_groups: Vec<(String, String)> = groups
        .iter()
        .map(|g| (g.id.clone(), g.name.clone()))
        .collect();

    // Members infinite scroll query
    let team_pk_signal = use_signal(|| team_pk.clone());
    let mut members_query = use_infinite_query::<String, TeamMemberResponse, ListResponse<TeamMemberResponse>, _>(
        move |bookmark| {
            let team_pk = team_pk_signal();
            async move {
                list_members_handler(team_pk, bookmark, Some(PAGE_SIZE)).await
            }
        },
    )?;

    // Restart query when members are added/removed (skip initial render where refresh == 0)
    use_effect(move || {
        let r = refresh();
        if r > 0 {
            members_query.restart();
        }
    });

    let members = members_query.items();

    let on_add_members_click = {
        let mut popup = popup;
        let team_pk = team_pk.clone();
        let username = username.clone();
        let groups = groups.clone();
        move |_| {
            let on_close = { let mut popup = popup; move |_| { popup.close(); } };
            let on_invited = {
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

                // Scrollable rows (fixed height = 9 rows × 60px)
                ScrollArea {
                    direction: ScrollDirection::Vertical,
                    class: "flex flex-col max-h-[540px]",
                    for (idx, member) in members.iter().enumerate() {
                        {
                            let is_last = idx == members.len() - 1;
                            let member = member.clone();
                            let failed_remove = failed_remove_member.clone();
                            let failed_change = failed_change_group.clone();
                            let all_groups = all_groups.clone();
                            rsx! {
                                MemberRow {
                                    key: "{member.user_id}",
                                    member: member.clone(),
                                    is_last,
                                    all_groups,
                                    on_remove: move |_| {
                                        let member = member.clone();
                                        let team_pk = team_pk_signal();
                                        let mut error_msg = error_msg.clone();
                                        let failed_msg = failed_remove.clone();
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
                                    on_change_group: move |(member_id, from_group_id, to_group_id): (String, String, String)| {
                                        let team_pk = team_pk_signal();
                                        let mut error_msg = error_msg.clone();
                                        let failed_msg = failed_change.clone();
                                        spawn(async move {
                                            let remove_ok = remove_member_handler(
                                                team_pk.clone(),
                                                from_group_id.clone(),
                                                RemoveMemberRequest {
                                                    user_pks: vec![member_id.clone()],
                                                },
                                            )
                                            .await
                                            .is_ok();

                                            if remove_ok {
                                                let add_ok = add_member_handler(
                                                    team_pk.clone(),
                                                    to_group_id,
                                                    AddMemberRequest {
                                                        user_pks: vec![member_id.clone()],
                                                    },
                                                )
                                                .await
                                                .is_ok();

                                                if add_ok {
                                                    error_msg.set(None);
                                                    refresh.set(refresh() + 1);
                                                } else {
                                                    // Rollback: re-add to the original group
                                                    let _ = add_member_handler(
                                                        team_pk,
                                                        from_group_id,
                                                        AddMemberRequest {
                                                            user_pks: vec![member_id],
                                                        },
                                                    )
                                                    .await;
                                                    error_msg.set(Some(failed_msg));
                                                }
                                            } else {
                                                error_msg.set(Some(failed_msg));
                                            }
                                        });
                                    },
                                }
                            }
                        }
                    }

                    if members.is_empty() && !members_query.is_loading() {
                        div { class: "px-4 py-8 text-center text-sm text-foreground-muted",
                            {tr.no_members}
                        }
                    }

                    // Infinite scroll sentinel
                    {members_query.more_element()}
                } // ScrollArea
            }
        }
        PopupZone {}
    }
}

#[component]
fn MemberRow(
    member: TeamMemberResponse,
    is_last: bool,
    all_groups: Vec<(String, String)>,
    on_remove: EventHandler<()>,
    on_change_group: EventHandler<(String, String, String)>,
) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    let border_class = if is_last { "" } else { "border-b border-border" };
    let mut show_menu = use_signal(|| false);
    let mut show_change_group = use_signal(|| false);

    let first_group_id = member.groups.first().map(|g| g.group_id.clone()).unwrap_or_default();
    let mut from_group_id = use_signal(|| first_group_id);
    let mut to_group_id = use_signal(|| String::new());

    let display = if member.display_name.is_empty() {
        member.username.clone()
    } else {
        member.display_name.clone()
    };

    let has_groups = !member.groups.is_empty();
    let can_change_group = has_groups && all_groups.len() > 1;

    rsx! {
        div { class: "flex flex-col {border_class}",
            // Main row
            div { class: "relative flex items-center gap-3 px-4 py-3",
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
                    span { class: "text-sm text-foreground-muted shrink-0",
                        {
                            if member.groups.is_empty() {
                                tr.member_role.to_string()
                            } else {
                                member.groups.iter().map(|g| g.group_name.as_str()).collect::<Vec<_>>().join(", ")
                            }
                        }
                    }
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
                                show_change_group.set(false);
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
                                class: if is_last { "absolute right-0 bottom-8 z-20 w-44 bg-popover border border-border rounded-lg shadow-lg py-1 overflow-hidden" } else { "absolute right-0 top-8 z-20 w-44 bg-popover border border-border rounded-lg shadow-lg py-1 overflow-hidden" },
                                onclick: move |e| e.stop_propagation(),
                                if can_change_group {
                                    Button {
                                        style: ButtonStyle::Text,
                                        size: ButtonSize::Small,
                                        shape: ButtonShape::Square,
                                        class: "flex items-center gap-2 w-full justify-start".to_string(),
                                        onclick: {
                                            let first_group_id = member.groups.first().map(|g| g.group_id.clone()).unwrap_or_default();
                                            move |_| {
                                                show_menu.set(false);
                                                // Re-sync from_group_id to the current first group in case
                                                // member.groups changed after a refresh (signal persists for
                                                // keyed components and won't update automatically from props).
                                                from_group_id.set(first_group_id.clone());
                                                show_change_group.set(true);
                                            }
                                        },
                                        lucide_dioxus::ArrowLeftRight {
                                            class: "w-4 h-4 [&>path]:stroke-text-primary [&>polyline]:stroke-text-primary",
                                        }
                                        {tr.change_group}
                                    }
                                }
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

            // Change group panel
            if show_change_group() {
                div { class: "flex flex-col gap-3 px-4 pb-3",
                    div { class: "flex flex-col gap-2",
                        // From group (only show select if member has multiple groups)
                        if member.groups.len() > 1 {
                            div { class: "flex flex-col gap-1",
                                span { class: "text-xs font-semibold text-foreground-muted", {tr.from_group} }
                                select {
                                    class: "w-full px-3 py-2 rounded-[8px] border border-input-box-border bg-input-box-bg text-text-primary text-sm",
                                    value: from_group_id(),
                                    onchange: move |e| from_group_id.set(e.value()),
                                    for group in member.groups.iter() {
                                        option { value: "{group.group_id}", "{group.group_name}" }
                                    }
                                }
                            }
                        }

                        // To group
                        div { class: "flex flex-col gap-1",
                            span { class: "text-xs font-semibold text-foreground-muted", {tr.to_group} }
                            select {
                                class: "w-full px-3 py-2 rounded-[8px] border border-input-box-border bg-input-box-bg text-text-primary text-sm",
                                value: to_group_id(),
                                onchange: move |e| to_group_id.set(e.value()),
                                option { value: "", disabled: true, "—" }
                                for (gid, gname) in all_groups.iter().filter(|(gid, _)| *gid != from_group_id()) {
                                    option { key: "{gid}", value: "{gid}", "{gname}" }
                                }
                            }
                        }
                    }

                    // Action buttons
                    div { class: "flex gap-2",
                        Button {
                            style: ButtonStyle::Primary,
                            shape: ButtonShape::Square,
                            size: ButtonSize::Small,
                            disabled: to_group_id().is_empty(),
                            onclick: move |_| {
                                let to = to_group_id();
                                if to.is_empty() {
                                    return;
                                }
                                on_change_group.call((
                                    member.user_id.clone(),
                                    from_group_id(),
                                    to,
                                ));
                                show_change_group.set(false);
                                to_group_id.set(String::new());
                            },
                            {tr.apply}
                        }
                        Button {
                            style: ButtonStyle::Outline,
                            shape: ButtonShape::Square,
                            size: ButtonSize::Small,
                            onclick: move |_| {
                                show_change_group.set(false);
                                to_group_id.set(String::new());
                            },
                            {tr.cancel}
                        }
                    }
                }
            }
        }
    }
}
