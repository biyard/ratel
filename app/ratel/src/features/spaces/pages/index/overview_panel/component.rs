use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::controllers::SpaceResponse;

const DEFAULT_PROFILE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

#[component]
pub fn OverviewPanel(
    open: bool,
    on_close: EventHandler<()>,
    space: SpaceResponse,
    participants: String,
    remaining: String,
    rewards: String,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();

    let author_profile = if space.author_profile_url.is_empty() {
        DEFAULT_PROFILE.to_string()
    } else {
        space.author_profile_url.clone()
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "overview-panel",
            "data-testid": "overview-panel",
            "data-open": open,
            div { class: "overview-panel__header",
                span { class: "overview-panel__title", "{tr.overview}" }
                button {
                    aria_label: "Close overview",
                    class: "overview-panel__close",
                    onclick: move |_| {
                        on_close.call(());
                    },
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        line {
                            x1: "18",
                            x2: "6",
                            y1: "6",
                            y2: "18",
                        }
                        line {
                            x1: "6",
                            x2: "18",
                            y1: "6",
                            y2: "18",
                        }
                    }
                }
            }
            div { class: "overview-panel__body",
                div { class: "overview-section",
                    span { class: "overview-section__label", "{tr.about}" }
                    div {
                        class: "overview-section__content",
                        dangerous_inner_html: "{space.content}",
                    }
                }
                div { class: "overview-section",
                    span { class: "overview-section__label", "{tr.key_metrics}" }
                    div { class: "overview-info-grid",
                        div { class: "overview-info-card",
                            span { class: "overview-info-card__value", "{participants}" }
                            span { class: "overview-info-card__label", "{tr.participants}" }
                        }
                        div { class: "overview-info-card",
                            span { class: "overview-info-card__value", "{remaining}" }
                            span { class: "overview-info-card__label", "{tr.spots_left}" }
                        }
                        div { class: "overview-info-card",
                            span { class: "overview-info-card__value", "{rewards}" }
                            span { class: "overview-info-card__label", "{tr.reward_pool}" }
                        }
                        div { class: "overview-info-card",
                            span { class: "overview-info-card__value", "{space.likes}" }
                            span { class: "overview-info-card__label", "{tr.likes}" }
                        }
                    }
                }
                div { class: "overview-section",
                    span { class: "overview-section__label", "{tr.created_by}" }
                    div { class: "overview-author-row",
                        img {
                            alt: "Author",
                            class: "overview-author-avatar",
                            src: "{author_profile}",
                        }
                        div {
                            div { class: "overview-author-name", "{space.author_display_name}" }
                            div { class: "overview-author-username", "@{space.author_username}" }
                        }
                    }
                }
            }
        }
    }
}
