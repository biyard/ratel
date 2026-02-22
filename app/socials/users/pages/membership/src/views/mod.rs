use crate::controllers::get_membership::{get_membership_handler, MembershipResponse};
use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    let resource = use_server_future(move || async move {
        get_membership_handler().await
    })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();

    let membership = match data.as_ref() {
        Ok(data) => data.clone(),
        Err(_) => MembershipResponse::default(),
    };

    rsx! {
        div { class: "flex flex-col gap-6 w-full py-6",
            h2 { class: "text-xl font-bold text-[var(--text-primary)]", "Membership" }
            div { class: "flex flex-col gap-4 p-6 rounded-lg border border-[var(--border-primary)]",
                div { class: "flex flex-row items-center justify-between",
                    div { class: "flex flex-col",
                        p { class: "text-sm text-[var(--text-secondary)]", "Current Plan" }
                        p { class: "text-2xl font-bold text-[var(--text-primary)]", "{membership.tier}" }
                    }
                    div { class: "px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800",
                        "{membership.status}"
                    }
                }

                div { class: "flex flex-row gap-8",
                    div { class: "flex flex-col",
                        p { class: "text-sm text-[var(--text-secondary)]", "Total Credits" }
                        p { class: "text-lg font-semibold text-[var(--text-primary)]",
                            "{membership.total_credits}"
                        }
                    }
                    div { class: "flex flex-col",
                        p { class: "text-sm text-[var(--text-secondary)]", "Remaining Credits" }
                        p { class: "text-lg font-semibold text-[var(--text-primary)]",
                            "{membership.remaining_credits}"
                        }
                    }
                }

                if membership.auto_renew {
                    p { class: "text-sm text-[var(--text-secondary)]",
                        "Auto-renewal is enabled"
                    }
                }
            }
        }
    }
}
