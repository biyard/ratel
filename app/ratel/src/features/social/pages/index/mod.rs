use crate::features::social::pages::home::Home as TeamHome;
use crate::features::social::user_views::Home as UserHomeRoot;
use crate::features::social::*;
use crate::*;

#[component]
pub fn SocialIndex(username: ReadSignal<String>) -> Element {
    let ctx = use_wall_context();

    rsx! {
        if ctx.is_user() {
            UserHomeRoot { username: username() }
        } else if ctx.is_team() {
            TeamHome { username }
        }
    }
}
