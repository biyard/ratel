use crate::*;

#[component]
pub fn AppLayout() -> Element {
    TeamContext::init();
    let user_ctx = ratel_auth::hooks::use_user_context();
    let mut team_ctx = use_team_context();

    // Load teams when user is logged in
    let _teams_loader = use_resource(move || async move {
        let user = user_ctx().user.clone();
        if user.is_some() {
            match get_user_teams_handler().await {
                Ok(teams) => {
                    team_ctx.set_teams(teams);
                }
                Err(e) => {
                    debug!("Failed to load teams: {:?}", e);
                }
            }
        }
    });

    rsx! {
        div { class: "antialiased bg-bg",
            AppMenu {}
            Outlet::<Route> {}
        }
    }
}
