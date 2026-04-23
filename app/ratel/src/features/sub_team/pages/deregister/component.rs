//! Deregister sub-team confirmation page. Consumes
//! `UseSubTeamDeregister`. Submit requires reason + confirm checkbox.

use crate::features::social::controllers::find_team::find_team_handler;
use crate::features::sub_team::{
    use_sub_team_deregister, DeregisterRequest, SubTeamTranslate, UseSubTeamDeregister,
};
use crate::*;

#[component]
pub fn TeamSubTeamDeregisterPage(username: String, sub_team_id: String) -> Element {
    let tr: SubTeamTranslate = use_translate();

    let username_for_load = username.clone();
    let team_resource = use_loader(move || {
        let name = username_for_load.clone();
        async move { find_team_handler(name).await }
    })?;

    let team_pk = team_resource().pk;
    let team_id: TeamPartition = team_pk
        .parse::<TeamPartition>()
        .unwrap_or(TeamPartition(String::new()));
    use_context_provider(|| team_id);
    let sub_team_id_for_ctx = sub_team_id.clone();
    use_context_provider(move || sub_team_id_for_ctx.clone());

    rsx! {
        SeoMeta { title: "{tr.deregister_title}" }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        DeregisterForm {}
    }
}

#[component]
fn DeregisterForm() -> Element {
    let tr: SubTeamTranslate = use_translate();
    let nav = use_navigator();

    let UseSubTeamDeregister {
        mut reason,
        mut handle_deregister,
        ..
    } = use_sub_team_deregister()?;

    let mut confirmed: Signal<bool> = use_signal(|| false);
    let ready = confirmed() && !reason().trim().is_empty();

    rsx! {
        div { class: "arena sub-team-deregister",
            div { class: "page page--narrow",
                div { class: "dereg",
                    div { class: "dereg__head",
                        div { class: "dereg__icon",
                            lucide_dioxus::TriangleAlert { class: "w-5 h-5 [&>path]:stroke-current" }
                        }
                        div {
                            h1 { class: "dereg__title", "{tr.deregister_title}" }
                        }
                    }

                    // Reason
                    div { class: "field",
                        label { class: "field__label",
                            "{tr.deregister_reason} "
                            span { class: "req", "*" }
                        }
                        textarea {
                            class: "field__textarea",
                            id: "dereg-reason",
                            value: "{reason()}",
                            oninput: move |e| reason.set(e.value()),
                        }
                    }

                    // Confirmation
                    div { class: "field",
                        label { class: "checkbox",
                            input {
                                r#type: "checkbox",
                                id: "confirm-check",
                                checked: confirmed(),
                                onchange: move |e| confirmed.set(e.checked()),
                            }
                            span { class: "checkbox__box",
                                lucide_dioxus::Check { class: "w-3 h-3 [&>path]:stroke-current" }
                            }
                            span { class: "checkbox__label", "Confirm deregistration" }
                        }
                    }

                    div { class: "u-flex u-gap-10 u-justify-between",
                        button {
                            class: "btn btn--ghost",
                            onclick: move |_| {
                                nav.go_back();
                            },
                            "{tr.cancel}"
                        }
                        button {
                            class: "btn btn--danger",
                            id: "confirm-btn",
                            disabled: !ready,
                            onclick: move |_| {
                                if !ready {
                                    return;
                                }
                                handle_deregister
                                    .call(DeregisterRequest {
                                        reason: reason().clone(),
                                    });
                            },
                            lucide_dioxus::LogOut { class: "w-3 h-3 [&>path]:stroke-current" }
                            "{tr.deregister_confirm}"
                        }
                    }
                }
            }
        }
    }
}
