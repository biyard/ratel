use crate::*;

/// `ArcadeHomePage` — Phase A stub. Phase B fills this with the full
/// lobby.html mockup (featured card + my-stats + history + game
/// catalog + leaderboard tab).
#[component]
pub fn ArcadeHomePage() -> Element {
    rsx! {
        SeoMeta { title: "Ratel Arcade" }
        section { class: "ff-home",
            div { style: "padding: 60px 20px; text-align: center; color: var(--text-faint); font-style: italic",
                "Arcade home — coming online next step."
            }
        }
    }
}
