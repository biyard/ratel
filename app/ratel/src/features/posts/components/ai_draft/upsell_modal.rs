use crate::common::*;
use crate::features::posts::components::ai_draft::i18n::AiDraftTranslate;

#[component]
pub fn UpsellModal(on_close: EventHandler<()>, on_upgrade: EventHandler<()>) -> Element {
    let tr: AiDraftTranslate = use_translate();
    rsx! {
        div { class: "ai-scrim",
            section {
                class: "ai-modal ai-modal--upsell",
                role: "dialog",
                aria_modal: "true",
                "aria-labelledby": "ai-upsell-title",
                "data-testid": "ai-upsell-modal",

                button {
                    class: "ai-modal__close ai-modal__close--upsell",
                    r#type: "button",
                    aria_label: "Close",
                    onclick: move |_| on_close.call(()),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        path { d: "M6 6l12 12M18 6L6 18" }
                    }
                }

                header { class: "ai-modal__head ai-modal__head--upsell",
                    div { class: "ai-sparkle-orb",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M12 3l1.6 4.4L18 9l-4.4 1.6L12 15l-1.6-4.4L6 9l4.4-1.6L12 3z" }
                            path { d: "M19 14l.8 2.2L22 17l-2.2.8L19 20l-.8-2.2L16 17l2.2-.8L19 14z" }
                        }
                    }
                    div { class: "ai-modal__eyebrow", "{tr.upsell_eyebrow}" }
                    h2 {
                        class: "ai-modal__title ai-modal__title--upsell",
                        id: "ai-upsell-title",
                        "{tr.upsell_title_lead}"
                        em { "{tr.upsell_title_accent}" }
                        "{tr.upsell_title_tail}"
                    }
                    p { class: "ai-modal__sub", "{tr.upsell_sub}" }
                }

                div { class: "ai-modal__body",
                    div { class: "ai-benefits",
                        div { class: "ai-benefit",
                            div { class: "ai-benefit__icon",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M12 3l1.6 4.4L18 9l-4.4 1.6L12 15l-1.6-4.4L6 9l4.4-1.6L12 3z" }
                                }
                            }
                            div { class: "ai-benefit__text",
                                strong { "{tr.upsell_benefit_1_title}" }
                                span { "{tr.upsell_benefit_1_desc}" }
                            }
                        }
                        div { class: "ai-benefit",
                            div { class: "ai-benefit__icon",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    circle { cx: "12", cy: "12", r: "9" }
                                    path { d: "M12 7v5l3 2" }
                                }
                            }
                            div { class: "ai-benefit__text",
                                strong { "{tr.upsell_benefit_2_title}" }
                                span { "{tr.upsell_benefit_2_desc}" }
                            }
                        }
                        div { class: "ai-benefit",
                            div { class: "ai-benefit__icon",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M3 5h12M3 12h12M3 19h8" }
                                    path { d: "M19 8v8M15 12h8" }
                                }
                            }
                            div { class: "ai-benefit__text",
                                strong { "{tr.upsell_benefit_3_title}" }
                                span { "{tr.upsell_benefit_3_desc}" }
                            }
                        }
                    }
                }

                footer { class: "ai-modal__foot ai-modal__foot--upsell",
                    button {
                        class: "ai-cta-primary",
                        r#type: "button",
                        "data-testid": "ai-upsell-upgrade",
                        onclick: move |_| on_upgrade.call(()),
                        span { "{tr.upsell_cta}" }
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2.5",
                            stroke_linecap: "round",
                            path { d: "M5 12h14M13 6l6 6-6 6" }
                        }
                    }
                    button {
                        class: "ai-cta-secondary",
                        r#type: "button",
                        onclick: move |_| on_close.call(()),
                        "{tr.upsell_dismiss}"
                    }
                }

                div { class: "ai-modal__tier", "{tr.upsell_tier_note}" }
            }
        }
    }
}
