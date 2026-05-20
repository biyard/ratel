use crate::common::*;
use crate::features::my_follower::controllers::{
    check_follow_status_handler, follow_user, unfollow_user,
};
use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::social::pages::team_arena::settings_panel::ArenaSettingsPanel;
use crate::features::social::pages::team_arena::team_arena_context::TeamArenaContext;
use crate::features::social::pages::team_arena::topbar::ArenaTopbar;
use crate::route::Route;
use crate::social::hooks::use_wall_context;

/// Arena-style layout wrapping every team route. Replaces the old
/// `SocialLayout` + `TeamSettingLayout` for team-scoped pages.
#[component]
pub fn TeamArenaLayout(username: ReadSignal<String>) -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let wall_ctx = use_wall_context();

    let mut settings_open = use_signal(|| false);

    let team_ctx: TeamArenaContext = wall_ctx.into();

    let mut ctx = provide_context(team_ctx);

    // Also install the raw `TeamPartition` so sub-team hooks that
    // expect `use_context::<TeamPartition>()` (parent_relationship,
    // sub_team_queue, etc.) can run anywhere inside the team arena
    // without each page re-providing it. ParentHudPanel relies on this.
    let team_partition = ctx.team_id();
    provide_context(team_partition);

    // Category filter signal consumed by TeamHome (carried over from SocialLayout).
    provide_context(Signal::new(Option::<String>::None));

    let profile_url = ctx.profile_url();
    let display_name = ctx.display_name();
    let can_edit = ctx.can_edit();
    let is_member = ctx.is_member();
    let logged_in = user_ctx().user.is_some();

    // Sub-team activation comes from the wall context's turn-key team
    // payload — no extra per-page fetch needed.
    let is_parent_eligible = ctx.is_parent_eligible();

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
                is_member,
                is_parent_eligible,
                pending_applicant_username: (ctx.viewer_pending_applicant_username)(),
                viewer_has_eligible_applicant_team: (ctx.viewer_has_eligible_applicant_team)(),
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
