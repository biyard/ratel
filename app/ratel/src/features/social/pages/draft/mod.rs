pub mod team;
pub mod user;

use crate::features::social::*;
use crate::*;

#[component]
pub fn SocialDraft(username: ReadSignal<String>) -> Element {
    let ctx = use_wall_context();

    rsx! {
        if ctx.is_user() {
            user::Home { username: username() }
        } else if ctx.is_team() {
            team::Home { username }
        }
    }
}
