use crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus;
use crate::features::spaces::pages::index::*;

#[component]
pub fn VerificationCard(
    space_id: ReadSignal<SpacePartition>,
    requirements: Vec<PanelRequirementStatus>,
    on_verified: EventHandler<Vec<PanelRequirementStatus>>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let mut error_message = use_signal(|| Option::<String>::None);
    let mut toast = use_toast();
    let user_ctx = crate::features::auth::hooks::use_user_context();

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }

        div { class: "verification-card", "data-testid": "card-verification",
            span { class: "verification-card__heading", "{tr.verification_heading}" }

            div { class: "verification-card__alert",
                svg {
                    class: "verification-card__alert-icon",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    circle { cx: "12", cy: "12", r: "10" }
                    line {
                        x1: "12",
                        x2: "12",
                        y1: "8",
                        y2: "12",
                    }
                    line {
                        x1: "12",
                        x2: "12.01",
                        y1: "16",
                        y2: "16",
                    }
                }
                "{tr.verification_alert}"
            }

            if let Some(err) = error_message() {
                div { class: "verification-card__error", "{err}" }
            }

            div { class: "verification-card__requirements",
                div { class: "verification-card__req-title", "{tr.required_attributes}" }

                for requirement in requirements.iter() {
                    div {
                        class: "verification-card__req-row",
                        "data-satisfied": requirement.satisfied,
                        if requirement.satisfied {
                            svg {
                                class: "verification-card__req-icon verification-card__req-icon--ok",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                                polyline { points: "22 4 12 14.01 9 11.01" }
                            }
                        } else {
                            svg {
                                class: "verification-card__req-icon verification-card__req-icon--missing",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                circle { cx: "12", cy: "12", r: "10" }
                                line {
                                    x1: "12",
                                    x2: "12",
                                    y1: "8",
                                    y2: "12",
                                }
                                line {
                                    x1: "12",
                                    x2: "12.01",
                                    y1: "16",
                                    y2: "16",
                                }
                            }
                        }
                        span { class: "verification-card__req-label",
                            "{requirement.attribute.translate(&lang())}"
                        }
                        if !requirement.collective && !requirement.required_values.is_empty() {
                            div { class: "verification-card__req-values",
                                for value in requirement.required_values.iter() {
                                    span { class: if requirement.current_value.as_deref() == Some(value.as_str()) { "verification-card__req-value verification-card__req-value--mine" } else { "verification-card__req-value verification-card__req-value--other" },
                                        "{value}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            button {
                class: "cta-verify",
                "data-testid": "btn-verify",
                onclick: move |_| async move {
                    #[cfg(not(feature = "server"))]
                    {
                        let conf = crate::features::social::pages::credentials::config::get();
                        let store_id = conf.portone.store_id.to_string();
                        let channel_key = conf.portone.inicis_channel_key.to_string();
                        let prefix = user_ctx().user_id().unwrap_or_default();
                        let verification_failed_msg = tr.verification_failed.to_string();
                        match crate::features::social::pages::credentials::interop::verify_identity(
                                &store_id,
                                &channel_key,
                                &prefix,
                            )
                            .await
                        {
                            Ok(_) => {
                                match crate::features::spaces::controllers::panel_requirements::get_panel_requirements(
                                        space_id(),
                                    )
                                    .await
                                {
                                    Ok(next_requirements) => {
                                        let all_satisfied = next_requirements
                                            .iter()
                                            .all(|r| r.satisfied);
                                        if all_satisfied {
                                            error_message.set(None);
                                            on_verified.call(next_requirements);
                                        } else {
                                            error_message.set(Some(verification_failed_msg));
                                        }
                                    }
                                    Err(err) => {
                                        toast.error(err);
                                    }
                                }
                            }
                            Err(err) => {
                                toast.error(err);
                            }
                        }
                    }
                },
                "{tr.verify_credentials}"
            }
        }
    }
}
