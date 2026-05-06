use crate::common::*;
use crate::features::my_follower::controllers::{
    check_follow_status_handler, follow_user, unfollow_user,
};
use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::social::pages::team_arena::settings_panel::ArenaSettingsPanel;
use crate::features::social::pages::team_arena::topbar::{ArenaTopbar, TeamArenaTab};
use crate::route::Route;
use crate::social::controllers::Wall;
use crate::social::hooks::{use_wall_context, UseWallContext};

/// Context exposed by `TeamArenaLayout` so child pages can highlight the correct
/// HUD tab and read resolved team data without re-fetching it.
#[derive(Clone, Copy, DioxusController)]
pub struct TeamArenaContext {
    pub active_tab: Signal<TeamArenaTab>,
    pub username: Signal<String>,
    pub display_name: Signal<String>,
    pub profile_url: Signal<String>,
    pub can_edit: Signal<bool>,
    pub is_admin: Signal<bool>,
    pub is_member: Signal<bool>,
    /// Bump this signal to force the layout to refetch the team profile
    /// (used by the settings page after Save Changes so the topbar reflects
    /// the new name/logo immediately).
    pub refresh_trigger: Signal<u32>,
    pub role: Signal<Option<crate::social::pages::member::dto::TeamRole>>,
    pub following: Signal<bool>,
    pub team_id: Signal<TeamPartition>,
}

impl From<UseWallContext> for TeamArenaContext {
    fn from(wall_ctx: UseWallContext) -> Self {
        match wall_ctx.data() {
            Wall::Team {
                id,
                username,
                display_name,
                profile_url,
                role,
                following,
                ..
            } => {
                let (can_edit, is_admin, is_member) = if let Some(role) = role {
                    (role.is_admin_or_owner(), role.is_admin_or_owner(), true)
                } else {
                    (false, false, false)
                };

                TeamArenaContext {
                    active_tab: Signal::new(TeamArenaTab::Home),
                    username: Signal::new(username),
                    display_name: Signal::new(display_name),
                    profile_url: Signal::new(profile_url),
                    can_edit: Signal::new(can_edit),
                    is_admin: Signal::new(is_admin),
                    is_member: Signal::new(is_member),
                    refresh_trigger: Signal::new(0),
                    role: Signal::new(role),
                    following: Signal::new(following),
                    team_id: Signal::new(id),
                }
            }
            _ => panic!("Wall context is not a team"),
        }
    }
}

pub fn use_team_arena() -> TeamArenaContext {
    use_context::<TeamArenaContext>()
}

/// Arena-style layout wrapping every team route. Replaces the old
/// `SocialLayout` + `TeamSettingLayout` for team-scoped pages.
#[component]
pub fn TeamArenaLayout(username: ReadSignal<String>) -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let wall_ctx = use_wall_context();

    let mut settings_open = use_signal(|| false);

    let team_ctx: TeamArenaContext = wall_ctx.into();

    let mut ctx = use_context_provider(|| team_ctx);

    // Category filter signal consumed by TeamHome (carried over from SocialLayout).
    use_context_provider(|| Signal::new(Option::<String>::None));

    let profile_url = ctx.profile_url();
    let display_name = ctx.display_name();
    let can_edit = ctx.can_edit();
    let is_member = ctx.is_member();
    let logged_in = user_ctx().user.is_some();

    let mut follow_processing = use_signal(|| false);

    let on_follow_click = move |_| {
        if follow_processing() {
            return;
        }
        follow_processing.set(true);
        spawn(async move {
            let pk = ctx.team_id().into();

            let result = if ctx.following() {
                unfollow_user(pk).await.map(|_| false)
            } else {
                follow_user(pk).await.map(|_| true)
            };
            match result {
                Ok(next) => {
                    ctx.following.set(next);
                }
                Err(e) => debug!("follow toggle failed: {:?}", e),
            }
            follow_processing.set(false);
        });
    };

    rsx! {
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
                username: username(),
                display_name: display_name.clone(),
                profile_url: profile_url.clone(),
                active: ctx.active_tab(),
                can_edit,
                show_follow: logged_in && !is_member,
                is_following: ctx.following(),
                on_follow: on_follow_click,
                on_open_settings: move |_| settings_open.set(true),
            }

            div { class: "team-arena__content",
                SuspenseBoundary { Outlet::<Route> {} }
            }

            ArenaSettingsPanel {
                open: settings_open(),
                on_close: move |_| settings_open.set(false),
                username: username(),
                can_edit,
            }
        }
    }
}
