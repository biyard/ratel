use crate::common::*;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use crate::features::social::pages::team_arena::{use_team_arena, TeamArenaTab};

mod admin_page;
mod viewer_page;

#[allow(unused_imports)]
use admin_page::*;
#[allow(unused_imports)]
use viewer_page::*;

use super::controllers::{
    add_team_member_handler, find_user_handler, get_team_member_permission_handler,
    list_members_handler, remove_team_member_handler, update_member_role_handler,
    FindUserQueryType,
};
use super::dto::{
    AddTeamMemberRequest, FoundUserResponse, RemoveMemberRequest, TeamMemberResponse, TeamRole,
    UpdateMemberRoleRequest,
};
use super::i18n::TeamMemberTranslate;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RoleFilter {
    All,
    Owner,
    Admin,
    Member,
}

#[component]
pub fn Home(username: String) -> Element {
    let tr: TeamMemberTranslate = use_translate();
    let mut toast = use_toast();

    // Sync arena topbar tab.
    let mut arena = use_team_arena();
    use_effect(move || arena.active_tab.set(TeamArenaTab::Members));

    // Load permission + team_pk for the current user.
    let perm_resource = use_loader(use_reactive((&username,), |(name,)| async move {
        Ok::<_, super::Error>(
            get_team_member_permission_handler(name)
                .await
                .map_err(|e| e.to_string()),
        )
    }))?;

    let perm_data = perm_resource.read();
    let perm_ctx = match perm_data.as_ref() {
        Ok(ctx) => ctx.clone(),
        Err(_) => {
            return rsx! {
                document::Link { rel: "stylesheet", href: asset!("./style.css") }
                ViewerPage { username: username.clone() }
            }
        }
    };
    let permissions: TeamGroupPermissions = perm_ctx.permissions.into();
    let can_edit = permissions.contains(TeamGroupPermission::TeamEdit)
        || permissions.contains(TeamGroupPermission::TeamAdmin)
        || permissions.contains(TeamGroupPermission::GroupEdit);
    if !can_edit {
        return rsx! {
            document::Link { rel: "stylesheet", href: asset!("./style.css") }
            ViewerPage { username: username.clone() }
        };
    }

    let team_pk = perm_ctx.team_pk.clone();

    // Fetch members list (refetched on demand via signal).
    let mut refresh = use_signal(|| 0u32);
    let members_resource = use_loader(use_reactive(
        (&team_pk, &refresh()),
        |(pk, _r)| async move {
            Ok::<_, super::Error>(
                list_members_handler(pk, None, Some(100))
                    .await
                    .map(|resp| resp.items)
                    .map_err(|e| e.to_string()),
            )
        },
    ))?;
    let members: Vec<TeamMemberResponse> =
        members_resource.read().clone().unwrap_or_default();

    // ── State: filter + search + invite modal + open menu ──────────
    let mut role_filter = use_signal(|| RoleFilter::All);
    let mut search = use_signal(String::new);
    let mut open_menu_pk = use_signal(|| Option::<String>::None);
    let mut invite_open = use_signal(|| false);

    // Counts
    let total = members.len();
    let owner_count = members.iter().filter(|m| m.is_owner).count();
    let admin_count = members
        .iter()
        .filter(|m| !m.is_owner && matches!(m.role, TeamRole::Admin))
        .count();
    let member_count = members
        .iter()
        .filter(|m| !m.is_owner && matches!(m.role, TeamRole::Member))
        .count();

    let filtered: Vec<TeamMemberResponse> = {
        let q = search().to_lowercase();
        let q = q.trim().to_string();
        members
            .iter()
            .filter(|m| match role_filter() {
                RoleFilter::All => true,
                RoleFilter::Owner => m.is_owner,
                RoleFilter::Admin => !m.is_owner && matches!(m.role, TeamRole::Admin),
                RoleFilter::Member => !m.is_owner && matches!(m.role, TeamRole::Member),
            })
            .filter(|m| {
                if q.is_empty() {
                    return true;
                }
                m.username.to_lowercase().contains(&q)
                    || m.display_name.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    };

    let team_pk_for_actions = team_pk.clone();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "tm-section-label",
            span { class: "tm-section-label__dash" }
            span { class: "tm-section-label__title",
                "Team "
                strong { "{tr.members_label}" }
            }
            span { class: "tm-section-label__dash" }
        }

        div {
            class: "tm-page",
            onclick: move |_| open_menu_pk.set(None),

            div { class: "tm-page-header",
                div { class: "tm-page-header__left",
                    h1 { class: "tm-page-header__title", "{tr.team_management}" }
                    span { class: "tm-page-header__sub",
                        strong { "{total}" }
                        " {tr.members_subhead}"
                    }
                }
                button {
                    class: "tm-btn-primary",
                    r#type: "button",
                    onclick: move |e: Event<MouseData>| {
                        e.stop_propagation();
                        invite_open.set(true);
                    },
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                        circle { cx: "8.5", cy: "7", r: "4" }
                        line { x1: "20", y1: "8", x2: "20", y2: "14" }
                        line { x1: "23", y1: "11", x2: "17", y2: "11" }
                    }
                    "{tr.add_member_btn}"
                }
            }

            div { class: "tm-filter-bar",
                div { class: "tm-search-wrap",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        circle { cx: "11", cy: "11", r: "8" }
                        line { x1: "21", y1: "21", x2: "16.65", y2: "16.65" }
                    }
                    input {
                        class: "tm-search-input",
                        r#type: "text",
                        placeholder: "{tr.search_placeholder}",
                        value: "{search}",
                        oninput: move |e| search.set(e.value()),
                    }
                }
                div { class: "tm-role-tabs",
                    RoleTab { label: tr.filter_all.to_string(), count: total, active: role_filter() == RoleFilter::All, on_click: move |_| role_filter.set(RoleFilter::All) }
                    RoleTab { label: tr.filter_owner.to_string(), count: owner_count, active: role_filter() == RoleFilter::Owner, on_click: move |_| role_filter.set(RoleFilter::Owner) }
                    RoleTab { label: tr.filter_admin.to_string(), count: admin_count, active: role_filter() == RoleFilter::Admin, on_click: move |_| role_filter.set(RoleFilter::Admin) }
                    RoleTab { label: tr.filter_member.to_string(), count: member_count, active: role_filter() == RoleFilter::Member, on_click: move |_| role_filter.set(RoleFilter::Member) }
                }
            }

            if filtered.is_empty() {
                div { class: "tm-empty-state",
                    div { class: "tm-empty-state__icon",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                            circle { cx: "9", cy: "7", r: "4" }
                        }
                    }
                    div { class: "tm-empty-state__title", "{tr.empty_title}" }
                    div { class: "tm-empty-state__desc", "{tr.empty_desc}" }
                }
            } else {
                div { class: "tm-members-grid",
                    for member in filtered.iter().cloned() {
                        MemberCard {
                            key: "{member.user_id}",
                            member: member.clone(),
                            team_pk: team_pk_for_actions.clone(),
                            menu_open: open_menu_pk() == Some(member.user_id.clone()),
                            on_toggle_menu: move |id: String| {
                                let cur = open_menu_pk();
                                open_menu_pk.set(if cur.as_deref() == Some(id.as_str()) { None } else { Some(id) });
                            },
                            on_changed: move |_: ()| {
                                refresh.with_mut(|n| *n = n.wrapping_add(1));
                            },
                        }
                    }
                }
            }
        }

        if invite_open() {
            InviteMemberModal {
                team_pk: team_pk.clone(),
                on_close: move |_: ()| invite_open.set(false),
                on_added: move |_: ()| {
                    invite_open.set(false);
                    refresh.with_mut(|n| *n = n.wrapping_add(1));
                    toast.info(tr.invite_success);
                },
            }
        }
    }
}

// ── Sub-components ─────────────────────────────────────────────

#[component]
fn RoleTab(label: String, count: usize, active: bool, on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "tm-role-tab",
            r#type: "button",
            aria_selected: active,
            onclick: move |_| on_click.call(()),
            "{label} "
            span { class: "tm-role-tab__count", "{count}" }
        }
    }
}

#[component]
fn MemberCard(
    member: TeamMemberResponse,
    team_pk: TeamPartition,
    menu_open: bool,
    on_toggle_menu: EventHandler<String>,
    on_changed: EventHandler<()>,
) -> Element {
    let tr: TeamMemberTranslate = use_translate();
    let mut toast = use_toast();
    let mut working = use_signal(|| false);

    let initial = member
        .display_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| {
            member
                .username
                .chars()
                .next()
                .map(|c| c.to_uppercase().to_string())
                .unwrap_or_else(|| "?".into())
        });
    let handle = format!("@{}", member.username);
    let user_pk = member.user_id.clone();

    let (badge_class, badge_label) = if member.is_owner {
        ("tm-role-badge tm-role-badge--owner", tr.role_owner)
    } else {
        match member.role {
            TeamRole::Admin => ("tm-role-badge tm-role-badge--admin", tr.role_admin),
            TeamRole::Member => ("tm-role-badge tm-role-badge--member", tr.role_member),
        }
    };

    let is_admin_role = matches!(member.role, TeamRole::Admin) && !member.is_owner;
    let toggle_label = if is_admin_role {
        tr.action_make_member
    } else {
        tr.action_make_admin
    };
    let next_role = if is_admin_role {
        TeamRole::Member
    } else {
        TeamRole::Admin
    };

    let user_pk_for_role = user_pk.clone();
    let team_pk_for_role = team_pk.clone();
    let on_toggle_role = move |_: MouseEvent| {
        if working() {
            return;
        }
        let user_pk = user_pk_for_role.clone();
        let team_pk = team_pk_for_role.clone();
        let next_role = next_role.clone();
        working.set(true);
        spawn(async move {
            let req = UpdateMemberRoleRequest {
                user_pk,
                role: next_role,
            };
            match update_member_role_handler(team_pk, req).await {
                Ok(_) => {
                    toast.info(tr.role_updated);
                    on_changed.call(());
                }
                Err(e) => {
                    toast.error(e);
                }
            }
            working.set(false);
        });
    };

    let user_pk_for_remove = user_pk.clone();
    let team_pk_for_remove = team_pk.clone();
    let on_remove = move |_: MouseEvent| {
        if working() {
            return;
        }
        let user_pk = user_pk_for_remove.clone();
        let team_pk = team_pk_for_remove.clone();
        working.set(true);
        spawn(async move {
            let req = RemoveMemberRequest {
                user_pks: vec![user_pk],
            };
            match remove_team_member_handler(team_pk, req).await {
                Ok(_) => {
                    toast.info(tr.member_removed);
                    on_changed.call(());
                }
                Err(e) => {
                    toast.error(e);
                }
            }
            working.set(false);
        });
    };

    let user_pk_for_toggle = user_pk.clone();

    rsx! {
        div {
            class: "tm-member-card",
            onclick: move |e: Event<MouseData>| e.stop_propagation(),
            if !member.profile_url.is_empty() {
                img {
                    class: "tm-member-avatar",
                    src: "{member.profile_url}",
                    alt: "{member.display_name}",
                }
            } else {
                div { class: "tm-member-avatar", "{initial}" }
            }
            div { class: "tm-member-body",
                span { class: "tm-member-name", "{member.display_name}" }
                span { class: "tm-member-handle", "{handle}" }
            }
            span { class: "{badge_class}", "{badge_label}" }
            if !member.is_owner {
                div {
                    class: "tm-member-action-wrap",
                    aria_expanded: menu_open,
                    button {
                        class: "tm-member-action",
                        r#type: "button",
                        aria_label: "More",
                        onclick: move |e: Event<MouseData>| {
                            e.stop_propagation();
                            on_toggle_menu.call(user_pk_for_toggle.clone());
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            circle { cx: "5", cy: "12", r: "1.5" }
                            circle { cx: "12", cy: "12", r: "1.5" }
                            circle { cx: "19", cy: "12", r: "1.5" }
                        }
                    }
                    div { class: "tm-member-menu", role: "menu",
                        button {
                            class: "tm-member-menu__item",
                            r#type: "button",
                            disabled: working(),
                            onclick: on_toggle_role,
                            if is_admin_role {
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                                    circle { cx: "12", cy: "7", r: "4" }
                                }
                            } else {
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                                }
                            }
                            "{toggle_label}"
                        }
                        div { class: "tm-member-menu__divider" }
                        button {
                            class: "tm-member-menu__item tm-member-menu__item--danger",
                            r#type: "button",
                            disabled: working(),
                            onclick: on_remove,
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M18 6L6 18" }
                                path { d: "M6 6l12 12" }
                            }
                            "{tr.action_remove}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InviteMemberModal(
    team_pk: TeamPartition,
    on_close: EventHandler<()>,
    on_added: EventHandler<()>,
) -> Element {
    let tr: TeamMemberTranslate = use_translate();
    let mut toast = use_toast();
    let mut role = use_signal(|| TeamRole::Member);
    let mut identifier = use_signal(String::new);
    let mut selected = use_signal(Vec::<FoundUserResponse>::new);
    let mut searching = use_signal(|| false);
    let mut sending = use_signal(|| false);
    let mut info_msg = use_signal(String::new);

    // Send button activates only when at least one user has been added to the chip list.
    let can_send = use_memo(move || !selected.read().is_empty() && !sending());

    // Search handler — invoked on Enter or comma separation.
    let do_search = move || {
        let raw = identifier();
        let identifiers: Vec<String> = raw
            .split(',')
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .collect();
        if identifiers.is_empty() {
            return;
        }
        searching.set(true);
        info_msg.set(String::new());
        spawn(async move {
            let mut not_found: Vec<String> = Vec::new();
            let mut already_added: Vec<String> = Vec::new();
            for ident in identifiers {
                let qt = detect_query_type(&ident);
                match find_user_handler(qt, ident.clone()).await {
                    Ok(user) => {
                        let exists = selected.read().iter().any(|u| u.pk == user.pk);
                        if exists {
                            already_added.push(user.nickname.clone());
                        } else {
                            selected.write().push(user);
                        }
                    }
                    Err(_) => not_found.push(ident),
                }
            }
            identifier.set(String::new());
            let mut parts = Vec::new();
            if !not_found.is_empty() {
                parts.push(format!("{} {}", tr.invite_not_found, not_found.join(", ")));
            }
            if !already_added.is_empty() {
                parts.push(format!(
                    "{} {}",
                    tr.invite_already_added,
                    already_added.join(", ")
                ));
            }
            if !parts.is_empty() {
                info_msg.set(parts.join(" · "));
            }
            searching.set(false);
        });
    };

    let mut do_search_for_input = do_search;

    let on_submit = {
        let team_pk = team_pk.clone();
        move |_: MouseEvent| {
            if !can_send() {
                return;
            }
            let team_pk = team_pk.clone();
            let user_pks: Vec<String> = selected.read().iter().map(|u| u.pk.clone()).collect();
            let role_val = role();
            sending.set(true);
            spawn(async move {
                let req = AddTeamMemberRequest {
                    user_pks,
                    role: role_val,
                };
                match add_team_member_handler(team_pk, req).await {
                    Ok(_) => {
                        on_added.call(());
                    }
                    Err(e) => {
                        info_msg.set(format!("{e}"));
                        toast.error(e);
                    }
                }
                sending.set(false);
            });
        }
    };

    rsx! {
        div {
            class: "tm-modal-overlay",
            onclick: move |_| on_close.call(()),
            div {
                class: "tm-modal",
                onclick: move |e: Event<MouseData>| e.stop_propagation(),
                div { class: "tm-modal__header",
                    span { class: "tm-modal__title", "{tr.invite_title}" }
                    button {
                        class: "tm-modal__close",
                        r#type: "button",
                        aria_label: "Close",
                        onclick: move |_| on_close.call(()),
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line { x1: "18", y1: "6", x2: "6", y2: "18" }
                            line { x1: "6", y1: "6", x2: "18", y2: "18" }
                        }
                    }
                }
                div { class: "tm-modal__body",
                    div {
                        label { class: "tm-field__label", "{tr.invite_group_label}" }
                        RoleDropdown { role }
                    }
                    div {
                        label {
                            class: "tm-field__label",
                            r#for: "tm-invite-input",
                            "{tr.invite_input_label}"
                        }
                        input {
                            id: "tm-invite-input",
                            class: "tm-field__input",
                            r#type: "text",
                            placeholder: "{tr.invite_input_placeholder}",
                            value: "{identifier}",
                            oninput: move |e| identifier.set(e.value()),
                            onkeydown: move |e: Event<KeyboardData>| {
                                if e.key() == Key::Enter {
                                    e.prevent_default();
                                    do_search_for_input();
                                }
                            },
                        }
                        p { class: "tm-field__hint", "{tr.invite_hint}" }
                        if searching() {
                            p { class: "tm-field__hint", "{tr.invite_searching}" }
                        }
                    }

                    if !selected.read().is_empty() {
                        div { class: "tm-chip-row",
                            for user in selected.read().clone() {
                                {
                                    let pk = user.pk.clone();
                                    rsx! {
                                        div { key: "{pk}", class: "tm-chip",
                                            span { class: "tm-chip__name", "{user.nickname}" }
                                            button {
                                                class: "tm-chip__close",
                                                r#type: "button",
                                                aria_label: "Remove",
                                                onclick: move |_| {
                                                    let pk = pk.clone();
                                                    selected.write().retain(|u| u.pk != pk);
                                                },
                                                svg {
                                                    view_box: "0 0 24 24",
                                                    fill: "none",
                                                    stroke: "currentColor",
                                                    stroke_width: "2.5",
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    line { x1: "18", y1: "6", x2: "6", y2: "18" }
                                                    line { x1: "6", y1: "6", x2: "18", y2: "18" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if !info_msg().is_empty() {
                        p { class: "tm-modal__error", "{info_msg()}" }
                    }

                    button {
                        class: "tm-modal__submit",
                        r#type: "button",
                        disabled: !can_send(),
                        onclick: on_submit,
                        if sending() {
                            "{tr.invite_sending}"
                        } else {
                            "{tr.invite_send}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RoleDropdown(role: Signal<TeamRole>) -> Element {
    let tr: TeamMemberTranslate = use_translate();
    let mut open = use_signal(|| false);
    let label = match role() {
        TeamRole::Admin => tr.invite_role_admin,
        TeamRole::Member => tr.invite_role_member,
    };

    rsx! {
        div { class: "tm-role-dd",
            button {
                class: "tm-role-dd__trigger",
                r#type: "button",
                aria_expanded: open(),
                onclick: move |e: Event<MouseData>| {
                    e.stop_propagation();
                    open.toggle();
                },
                span { class: "tm-role-dd__value", "{label}" }
                svg {
                    class: "tm-role-dd__chevron",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
            if open() {
                div {
                    style: "position:fixed;inset:0;z-index:5;",
                    onclick: move |_| open.set(false),
                }
                div { class: "tm-role-dd__menu", role: "menu",
                    button {
                        class: "tm-role-dd__item",
                        r#type: "button",
                        onclick: move |_| {
                            role.set(TeamRole::Admin);
                            open.set(false);
                        },
                        span { "{tr.invite_role_admin}" }
                        if matches!(role(), TeamRole::Admin) {
                            svg {
                                class: "tm-role-dd__check",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "3",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                        }
                    }
                    button {
                        class: "tm-role-dd__item",
                        r#type: "button",
                        onclick: move |_| {
                            role.set(TeamRole::Member);
                            open.set(false);
                        },
                        span { "{tr.invite_role_member}" }
                        if matches!(role(), TeamRole::Member) {
                            svg {
                                class: "tm-role-dd__check",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "3",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn detect_query_type(value: &str) -> FindUserQueryType {
    if value.contains('@') {
        return FindUserQueryType::Email;
    }
    let digit_like = value
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+' || *c == '-' || *c == ' ')
        .count();
    if digit_like == value.len() && value.chars().any(|c| c.is_ascii_digit()) {
        return FindUserQueryType::PhoneNumber;
    }
    FindUserQueryType::Username
}
