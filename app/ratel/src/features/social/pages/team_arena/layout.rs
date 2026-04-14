use crate::common::*;
use crate::features::my_follower::controllers::{
    check_follow_status_handler, follow_user, unfollow_user,
};
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::social::pages::team_arena::settings_panel::ArenaSettingsPanel;
use crate::features::social::pages::team_arena::topbar::{ArenaTopbar, TeamArenaTab};
use crate::route::Route;

/// Context exposed by `TeamArenaLayout` so child pages can highlight the correct
/// HUD tab and read resolved team data without re-fetching it.
#[derive(Clone, Copy)]
pub struct TeamArenaContext {
    pub active_tab: Signal<TeamArenaTab>,
    pub username: Signal<String>,
    pub display_name: Signal<String>,
    pub profile_url: Signal<String>,
    pub can_edit: Signal<bool>,
    pub is_admin: Signal<bool>,
    pub is_member: Signal<bool>,
}

pub fn use_team_arena() -> TeamArenaContext {
    use_context::<TeamArenaContext>()
}

/// Arena-style layout wrapping every team route. Replaces the old
/// `SocialLayout` + `TeamSettingLayout` for team-scoped pages.
#[component]
pub fn TeamArenaLayout(username: String) -> Element {
    crate::common::contexts::TeamContext::init();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut team_ctx = crate::common::contexts::use_team_context();

    let active_tab = use_signal(|| TeamArenaTab::Home);
    let mut settings_open = use_signal(|| false);
    let username_signal = use_signal(|| username.clone());
    let display_signal = use_signal(|| username.clone());
    let profile_signal = use_signal(String::new);
    let can_edit_signal = use_signal(|| false);
    let is_admin_signal = use_signal(|| false);
    let is_member_signal = use_signal(|| false);

    use_context_provider(|| TeamArenaContext {
        active_tab,
        username: username_signal,
        display_name: display_signal,
        profile_url: profile_signal,
        can_edit: can_edit_signal,
        is_admin: is_admin_signal,
        is_member: is_member_signal,
    });

    // Category filter signal consumed by TeamHome (carried over from SocialLayout).
    use_context_provider(|| Signal::new(Option::<String>::None));

    // Hydrate team list into shared context (for switcher dropdown etc.)
    let _teams_loader = use_resource(move || async move {
        let user = user_ctx().user.clone();
        if user.is_some() {
            match crate::get_user_teams_handler().await {
                Ok(teams) => {
                    team_ctx.set_teams(teams);
                }
                Err(e) => {
                    debug!("TeamArenaLayout: failed to load teams: {:?}", e);
                }
            }
        }
    });

    // Fetch the team record so we have display name, logo, and permissions.
    let team_resource = use_loader(use_reactive((&username,), |(name,)| async move {
        Ok::<_, crate::features::social::Error>(
            find_team_handler(name).await.map_err(|e| e.to_string()),
        )
    }))?;

    // Derive render-time values from the resource + team context fallback.
    let (display_name, profile_url, permissions_mask) = {
        let data = team_resource.read();
        let fallback = {
            let teams = team_ctx.teams.read();
            teams
                .iter()
                .find(|t| t.username == username)
                .cloned()
        };

        match data.as_ref() {
            Ok(team) if !team.pk.is_empty() && !team.username.is_empty() => {
                let perms: Vec<u8> = team
                    .permissions
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|p| p as u8)
                    .collect();
                let mut mask = 0i64;
                for v in &perms {
                    mask |= 1i64 << (*v as i32);
                }
                let nickname = if team.nickname.is_empty() {
                    team.username.clone()
                } else {
                    team.nickname.clone()
                };
                (
                    nickname,
                    team.profile_url.clone().unwrap_or_default(),
                    mask,
                )
            }
            _ => {
                if let Some(team) = fallback {
                    let mut mask = 0i64;
                    for v in &team.permissions {
                        mask |= 1i64 << (*v as i32);
                    }
                    let nickname = if team.nickname.is_empty() {
                        team.username.clone()
                    } else {
                        team.nickname.clone()
                    };
                    (nickname, team.profile_url.clone(), mask)
                } else {
                    (username.clone(), String::new(), 0i64)
                }
            }
        }
    };

    let permissions: TeamGroupPermissions = permissions_mask.into();
    let is_admin = permissions.contains(TeamGroupPermission::TeamAdmin);
    let can_edit = is_admin || permissions.contains(TeamGroupPermission::TeamEdit);
    let is_member = permissions_mask != 0;
    let logged_in = user_ctx().user.is_some();

    // Load follow status for non-members (members never see the follow btn).
    let follow_status = use_server_future({
        let name = username.clone();
        move || {
            let name = name.clone();
            async move { check_follow_status_handler(name).await.ok() }
        }
    })?;

    let (initial_following, follow_target_pk) = {
        let val = follow_status.read();
        match val.as_ref().and_then(|o| o.as_ref()) {
            Some(s) => (s.is_following, Some(s.target_pk.clone())),
            None => (false, None),
        }
    };

    let mut is_following = use_signal(|| initial_following);
    let mut follow_processing = use_signal(|| false);
    let target_pk_for_follow = follow_target_pk.clone();

    let on_follow_click = move |_| {
        if follow_processing() {
            return;
        }
        let Some(pk) = target_pk_for_follow.clone() else {
            return;
        };
        follow_processing.set(true);
        spawn(async move {
            let currently = is_following();
            let result = if currently {
                unfollow_user(pk).await.map(|_| false)
            } else {
                follow_user(pk).await.map(|_| true)
            };
            match result {
                Ok(next) => is_following.set(next),
                Err(e) => debug!("follow toggle failed: {:?}", e),
            }
            follow_processing.set(false);
        });
    };

    // Push resolved values into the context so child routes can read them.
    let mut ctx = use_team_arena();
    use_effect({
        let username = username.clone();
        let display_name = display_name.clone();
        let profile_url = profile_url.clone();
        move || {
            ctx.username.set(username.clone());
            ctx.display_name.set(display_name.clone());
            ctx.profile_url.set(profile_url.clone());
            ctx.can_edit.set(can_edit);
            ctx.is_admin.set(is_admin);
            ctx.is_member.set(is_member);
        }
    });

    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;800;900&family=Outfit:wght@300;400;500;600;700&display=swap",
        }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { defer: true, src: asset!("./script.js") }

        div { class: "team-arena", "data-testid": "team-arena-layout",

            // Decorative BG layers
            div { class: "team-arena__bg-grid",
                div { class: "team-arena__bg-grid-plane" }
            }
            div { class: "team-arena__bg-orbs",
                div { class: "team-arena__bg-orb team-arena__bg-orb--1" }
                div { class: "team-arena__bg-orb team-arena__bg-orb--2" }
                div { class: "team-arena__bg-orb team-arena__bg-orb--3" }
            }
            div { class: "team-arena__bg-stars" }
            div { class: "team-arena__bg-scanline" }

            ArenaTopbar {
                username: username.clone(),
                display_name: display_name.clone(),
                profile_url: profile_url.clone(),
                active: active_tab(),
                can_edit,
                show_follow: logged_in && !is_member,
                is_following: is_following(),
                on_follow: on_follow_click,
                on_open_settings: move |_| settings_open.set(true),
            }

            div { class: "team-arena__content", Outlet::<Route> {} }

            ArenaSettingsPanel {
                open: settings_open(),
                on_close: move |_| settings_open.set(false),
                username: username.clone(),
                can_edit,
            }
        }
    }
}
