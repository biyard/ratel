use crate::common::hooks::use_infinite_query;
use crate::common::ListResponse;
use crate::features::social::pages::member::components::{InviteMemberModal, InviteResult};
use crate::features::social::pages::member::controllers::{
    get_team_member_permission_handler, list_members_handler,
};
use crate::features::social::pages::member::dto::{TeamMemberResponse, TeamRole};
use crate::features::social::pages::setting::i18n::TeamSettingsTranslate;
use crate::features::social::*;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;

#[derive(Clone)]
struct ChangeRolePayload {
    user_id: String,
    new_role: TeamRole,
}

const PAGE_SIZE: i32 = 10;

#[component]
pub fn ManagementPage(username: String) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    use_context_provider(|| PopupService::new());
    let mut popup = use_popup();

    let perm_res = use_server_future(use_reactive((&username,), |(name,)| async move {
        get_team_member_permission_handler(name).await
    }))?;

    let (team_pk, can_manage) = match &*perm_res.read() {
        Some(Ok(ctx)) => {
            let permissions: TeamGroupPermissions = ctx.permissions.into();
            let can = permissions.contains(TeamGroupPermission::TeamAdmin)
                || permissions.contains(TeamGroupPermission::TeamEdit)
                || permissions.contains(TeamGroupPermission::GroupEdit);
            (ctx.team_pk.clone(), can)
        }
        Some(Err(_)) => {
            return rsx! {
                div { class: "flex flex-col gap-2 p-4",
                    span { class: "text-sm font-semibold text-text-primary", {tr.no_permission_title} }
                    span { class: "text-sm text-foreground-muted", {tr.no_permission_description} }
                }
            };
        }
        None => {
            return rsx! {
                div { class: "flex justify-center items-center w-full py-10",
                    crate::common::components::LoadingIndicator {}
                }
            };
        }
    };

    let mut error_msg = use_signal(|| Option::<String>::None);
    let failed_remove_member = tr.failed_remove_member.to_string();
    let failed_change_role = tr.failed_change_role.to_string();

    let user_ctx = crate::features::auth::hooks::use_user_context();
    let current_user_pk: Option<String> =
        user_ctx().user.as_ref().map(|u| u.pk.to_string());

    // Members infinite scroll query
    let team_pk_signal = use_signal(|| team_pk.clone());
    let mut members_query =
        use_infinite_query::<String, TeamMemberResponse, ListResponse<TeamMemberResponse>, _>(
            move |bookmark| {
                let team_pk = team_pk_signal();
                async move {
                    match list_members_handler(team_pk, bookmark, Some(PAGE_SIZE)).await {
                        Ok(response) => Ok(response),
                        Err(_) => Ok(ListResponse::<TeamMemberResponse>::default()),
                    }
                }
            },
        )?;

    let members = members_query.items();

    let on_add_members_click = {
        let mut popup = popup;
        let team_pk = team_pk.clone();
        let username = username.clone();
        move |_| {
            let on_close = {
                let mut popup = popup;
                move |_| {
                    popup.close();
                }
            };
            let on_invited = {
                let mut members_query = members_query;
                move |result: InviteResult| {
                    if result.total_added > 0 {
                        members_query.refresh();
                    }
                }
            };
            popup.open(rsx! {
                InviteMemberModal {
                    team_pk: team_pk.clone(),
                    username: username.clone(),
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
                if can_manage {
                    Button {
                        style: ButtonStyle::Primary,
                        shape: ButtonShape::Rounded,
                        size: ButtonSize::Small,
                        class: "flex items-center gap-2".to_string(),
                        onclick: on_add_members_click,
                        lucide_dioxus::UserPlus { class: "w-4 h-4 [&>path]:stroke-btn-primary-text [&>line]:stroke-btn-primary-text" }
                        {tr.add_members}
                    }
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
                    for (idx , member) in members.iter().enumerate() {
                        {
                            let is_last = idx == members.len() - 1;
                            let member = member.clone();
                            let failed_remove = failed_remove_member.clone();
                            let failed_role = failed_change_role.clone();
                            let is_self = current_user_pk
                                .as_deref()
                                .map(|pk| pk == member.user_id.as_str())
                                .unwrap_or(false);
                            rsx! {
                                MemberRow {
                                    key: "{member.user_id}",
                                    member: member.clone(),
                                    is_last,
                                    can_manage,
                                    is_self,
                                    on_remove: move |_| {
                                        let member = member.clone();
                                        let team_pk = team_pk_signal();
                                        let mut error_msg = error_msg.clone();
                                        let failed_msg = failed_remove.clone();
                                        let mut members_query = members_query;
                                        spawn(async move {
                                            let result = crate::features::social::pages::member::controllers::remove_team_member_handler(
                                                    team_pk.clone(),
                                                    crate::features::social::pages::member::dto::RemoveMemberRequest {
                                                        user_pks: vec![member.user_id.clone()],
                                                    },
                                                )
                                                .await;
                                            if result.is_err() {
                                                error_msg.set(Some(failed_msg));
                                            } else {
                                                error_msg.set(None);
                                                members_query.refresh();
                                            }
                                        });
                                    },
                                    on_change_role: move |payload: ChangeRolePayload| {
                                        let team_pk = team_pk_signal();
                                        let mut error_msg = error_msg.clone();
                                        let failed_msg = failed_role.clone();
                                        let mut members_query = members_query;
                                        spawn(async move {
                                            let result = crate::features::social::pages::member::controllers::update_member_role_handler(
                                                    team_pk.clone(),
                                                    crate::features::social::pages::member::dto::UpdateMemberRoleRequest {
                                                        user_pk: payload.user_id,
                                                        role: payload.new_role,
                                                    },
                                                )
                                                .await;
                                            if result.is_err() {
                                                error_msg.set(Some(failed_msg));
                                            } else {
                                                error_msg.set(None);
                                                members_query.refresh();
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
    can_manage: bool,
    is_self: bool,
    on_remove: EventHandler<()>,
    on_change_role: EventHandler<ChangeRolePayload>,
) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    let border_class = if is_last {
        ""
    } else {
        "border-b border-border"
    };
    let mut show_menu = use_signal(|| false);
    let can_change_role = can_manage && !member.is_owner && !is_self;

    let display = if member.display_name.is_empty() {
        member.username.clone()
    } else {
        member.display_name.clone()
    };

    rsx! {
        div { class: "flex flex-col {border_class}",
            div { class: "relative flex items-center gap-3 px-4 py-3",
                // Avatar
                if !member.profile_url.is_empty() {
                    img {
                        src: "{member.profile_url}",
                        alt: "{display}",
                        class: "w-9 h-9 rounded-full object-cover shrink-0",
                    }
                } else {
                    div { class: "w-9 h-9 rounded-full bg-profile-bg shrink-0 flex items-center justify-center",
                        span { class: "text-xs font-semibold text-text-primary",
                            "{display.chars().next().unwrap_or('?').to_uppercase()}"
                        }
                    }
                }

                // Name + username
                div { class: "flex flex-col min-w-0 flex-1",
                    span { class: "text-sm font-semibold text-text-primary truncate",
                        "{display}"
                    }
                    span { class: "text-xs text-foreground-muted truncate", "@{member.username}" }
                }

                // Role
                if member.is_owner {
                    span { class: "text-sm text-foreground-muted shrink-0", {tr.owner} }
                } else {
                    span { class: "text-sm text-foreground-muted shrink-0",
                        {
                            if member.role == TeamRole::Admin {
                                tr.admin_role.to_string()
                            } else {
                                tr.member_role.to_string()
                            }
                        }
                    }
                }

                // More options (only for users with manage permission)
                if can_manage && !member.is_owner {
                    div { class: "relative shrink-0",
                        Button {
                            style: ButtonStyle::Text,
                            size: ButtonSize::Icon,
                            shape: ButtonShape::Square,
                            class: "flex items-center justify-center w-7 h-7".to_string(),
                            onclick: move |e: MouseEvent| {
                                e.stop_propagation();
                                show_menu.toggle();
                            },
                            lucide_dioxus::Ellipsis { class: "w-4 h-4 [&>circle]:fill-text-primary [&>circle]:stroke-none" }
                        }
                        if show_menu() {
                            div {
                                class: "fixed inset-0 z-10",
                                onclick: move |_| show_menu.set(false),
                            }
                            div {
                                class: if is_last { "absolute right-0 bottom-8 z-20 w-44 bg-popover border border-border rounded-lg shadow-lg py-1 overflow-hidden" } else { "absolute right-0 top-8 z-20 w-44 bg-popover border border-border rounded-lg shadow-lg py-1 overflow-hidden" },
                                onclick: move |e| e.stop_propagation(),
                                if can_change_role && member.role != TeamRole::Admin {
                                    Button {
                                        style: ButtonStyle::Text,
                                        size: ButtonSize::Small,
                                        shape: ButtonShape::Square,
                                        class: "flex items-center gap-2 w-full text-text-primary justify-start".to_string(),
                                        onclick: {
                                            let user_id = member.user_id.clone();
                                            move |_| {
                                                show_menu.set(false);
                                                on_change_role
                                                    .call(ChangeRolePayload {
                                                        user_id: user_id.clone(),
                                                        new_role: TeamRole::Admin,
                                                    });
                                            }
                                        },
                                        lucide_dioxus::ShieldCheck { class: "w-4 h-4 [&>path]:stroke-text-primary" }
                                        {tr.make_admin}
                                    }
                                }
                                if can_change_role && member.role != TeamRole::Member {
                                    Button {
                                        style: ButtonStyle::Text,
                                        size: ButtonSize::Small,
                                        shape: ButtonShape::Square,
                                        class: "flex items-center gap-2 w-full text-text-primary justify-start".to_string(),
                                        onclick: {
                                            let user_id = member.user_id.clone();
                                            move |_| {
                                                show_menu.set(false);
                                                on_change_role
                                                    .call(ChangeRolePayload {
                                                        user_id: user_id.clone(),
                                                        new_role: TeamRole::Member,
                                                    });
                                            }
                                        },
                                        lucide_dioxus::User { class: "w-4 h-4 [&>path]:stroke-text-primary [&>circle]:stroke-text-primary" }
                                        {tr.make_member}
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
                                    lucide_dioxus::UserMinus { class: "w-4 h-4 [&>path]:stroke-destructive [&>line]:stroke-destructive" }
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
}
