use crate::controllers::get_rewards::{get_rewards_handler, RewardsResponse};
use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    let resource = use_server_future(move || async move {
        get_rewards_handler().await
    })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();

    let rewards = match data.as_ref() {
        Ok(data) => data.clone(),
        Err(_) => RewardsResponse::default(),
    };

    rsx! {
        div { class: "flex flex-col gap-6 w-full py-6",
            h2 { class: "text-xl font-bold text-[var(--text-primary)]", "Rewards" }
            div { class: "flex flex-col gap-4 p-6 rounded-lg border border-[var(--border-primary)]",
                div { class: "flex flex-col gap-2",
                    p { class: "text-sm text-[var(--text-secondary)]", "Total Points" }
                    p { class: "text-3xl font-bold text-[var(--text-primary)]",
                        "{rewards.points}"
                    }
                }
                p { class: "text-sm text-[var(--text-secondary)]",
                    "Points earned through participation in spaces and other activities."
                }
            }
        }
    }
}
