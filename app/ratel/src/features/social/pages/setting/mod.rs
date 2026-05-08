pub mod team;
pub mod user;

// Re-export team-only sub-route components so route.rs can import them under a single path.
pub use team::ManagementPage;
pub use team::SubscriptionPage;

use crate::features::social::*;
use crate::*;

#[component]
pub fn SocialSetting(username: ReadSignal<String>) -> Element {
    let ctx = use_wall_context();

    rsx! {
        if ctx.is_user() {
            user::Home { username: username() }
        } else if ctx.is_team() {
            team::Home { username }
        }
    }
}
