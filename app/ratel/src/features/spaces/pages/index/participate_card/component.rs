use crate::features::spaces::controllers::panel_requirements::PanelRequirementStatus;
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::controllers::get_user_role;
use crate::features::spaces::space_common::providers::use_space_context;

#[component]
pub fn ParticipateCard(
    space_id: ReadSignal<SpacePartition>,
    participants: String,
    remaining: String,
    rewards: String,
    #[props(default)] require_consent: bool,
    on_joined: EventHandler<()>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let mut show_consent = use_signal(|| false);
    let mut ctx = use_space_context();
    let panel_requirements = ctx.panel_requirements();

    rsx! {

        if show_consent() {
            ConsentModal {
                space_id,
                requirements: panel_requirements.clone(),
                on_cancel: move |_| {
                    show_consent.set(false);
                },
                on_joined,
            }
        } else {
            div { class: "participate-card", "data-testid": "card-participate",
                span { class: "participate-card__heading", "{tr.join_heading}" }
                p { class: "participate-card__desc", "{tr.join_desc}" }
                div { class: "participate-card__stats",
                    div { class: "stat",
                        span { class: "stat__value", "{participants}" }
                        span { class: "stat__label", "{tr.participants}" }
                    }
                    div { class: "stat",
                        span { class: "stat__value", "{remaining}" }
                        span { class: "stat__label", "{tr.remaining}" }
                    }
                    div { class: "stat",
                        span { class: "stat__value", "{rewards}" }
                        span { class: "stat__label", "{tr.rewards}" }
                    }
                }
                button {
                    class: "cta-participate",
                    "data-testid": "btn-participate",
                    onclick: move |_| async move {
                        if require_consent {
                            show_consent.set(true);
                        } else {
                            let req = crate::features::spaces::controllers::participate_space::ParticipateSpaceRequest {
                                informed_agreed: true,
                            };
                            if crate::features::spaces::controllers::participate_space::participate_space(
                                    space_id(),
                                    req,
                                )
                                .await
                                .is_ok()
                            {
                                on_joined.call(());
                                ctx.restart();
                            }
                        }
                    },
                    "{tr.participate}"
                }
            }
        }
    }
}

#[component]
fn ConsentModal(
    space_id: ReadSignal<SpacePartition>,
    requirements: Vec<PanelRequirementStatus>,
    on_cancel: EventHandler<()>,
    on_joined: EventHandler<()>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let mut consent_checked = use_signal(|| false);
    let mut ctx = use_space_context();

    rsx! {
        div { class: "consent-card", "data-testid": "card-consent",
            span { class: "consent-card__heading", "{tr.consent_heading}" }
            p { class: "consent-card__desc", "{tr.consent_desc}" }

            if !requirements.is_empty() {
                div { class: "consent-card__attributes",
                    div { class: "consent-card__attr-title", "{tr.required_attributes}" }
                    for requirement in requirements.iter() {
                        div {
                            class: "consent-card__attr-row",
                            "data-satisfied": requirement.satisfied,
                            if requirement.satisfied {
                                svg {
                                    class: "consent-card__attr-icon consent-card__attr-icon--ok",
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
                                    class: "consent-card__attr-icon consent-card__attr-icon--missing",
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
                            span { class: "consent-card__attr-label",
                                "{requirement.attribute.translate(&lang())}"
                            }
                            if !requirement.collective && !requirement.required_values.is_empty() {
                                div { class: "consent-card__attr-values",
                                    for value in requirement.required_values.iter() {
                                        span { class: if requirement.current_value.as_deref() == Some(value.as_str()) { "consent-card__attr-value consent-card__attr-value--mine" } else { "consent-card__attr-value consent-card__attr-value--other" },
                                            "{value}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            label { class: "consent-card__checkbox",
                input {
                    r#type: "checkbox",
                    checked: consent_checked(),
                    onchange: move |evt: FormEvent| {
                        consent_checked.set(evt.checked());
                    },
                }
                span { class: "consent-card__checkbox-label", "{tr.consent_label}" }
            }

            div { class: "consent-card__actions",
                button {
                    class: "consent-card__cancel",
                    onclick: move |_| {
                        on_cancel.call(());
                    },
                    "{tr.consent_cancel}"
                }
                button {
                    class: "consent-card__confirm",
                    "data-testid": "btn-consent-confirm",
                    disabled: !consent_checked(),
                    onclick: move |_| async move {
                        let req = crate::features::spaces::controllers::participate_space::ParticipateSpaceRequest {
                            informed_agreed: true,
                        };
                        if crate::features::spaces::controllers::participate_space::participate_space(
                                space_id(),
                                req,
                            )
                            .await
                            .is_ok()
                        {
                            on_joined.call(());
                            ctx.restart();
                        }
                    },
                    "{tr.consent_confirm}"
                }
            }
        }
    }
}
