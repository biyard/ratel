use crate::features::spaces::pages::actions::types::{SpaceActionSummary, SpaceActionType};
use crate::features::spaces::pages::index::*;

#[component]
pub fn LockedDependenciesPopup(
    space_id: SpacePartition,
    outstanding: Vec<SpaceActionSummary>,
    on_close: EventHandler<()>,
) -> Element {
    let tr: LockedDependenciesPopupTranslate = use_translate();
    let mut toast = use_toast();
    let _ = space_id;

    let on_pick = move |action: SpaceActionSummary| {
        if scroll_to_card(&action.action_id) {
            on_close.call(());
        } else {
            toast.error(crate::common::Error::NotFound(
                tr.scroll_failed.to_string(),
            ));
        }
    };

    rsx! {
        div { class: "locked-deps",
            div { class: "locked-deps__head",
                span { class: "locked-deps__icon",
                    svg {
                        view_box: "0 0 24 24",
                        width: "18",
                        height: "18",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        rect {
                            x: "3",
                            y: "11",
                            width: "18",
                            height: "11",
                            rx: "2",
                        }
                        path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                    }
                }
                div { class: "locked-deps__head-text",
                    span { class: "locked-deps__title", "{tr.title}" }
                    span { class: "locked-deps__subtitle", "{tr.subtitle}" }
                }
            }

            ul { class: "locked-deps__list",
                for action in outstanding.iter() {
                    {
                        let action_cloned = action.clone();
                        let on_pick = on_pick.clone();
                        rsx! {
                            li { key: "{action.action_id}", class: "locked-deps__item",
                                button {
                                    r#type: "button",
                                    class: "locked-deps__row",
                                    title: "{action.title}",
                                    onclick: move |_| {
                                        let mut on_pick = on_pick.clone();
                                        on_pick(action_cloned.clone());
                                    },
                                    span { class: "locked-deps__row-type", "{type_label(&action.action_type)}" }
                                    span { class: "locked-deps__row-title", "{action.title}" }
                                    svg {
                                        class: "locked-deps__row-chevron",
                                        view_box: "0 0 24 24",
                                        width: "14",
                                        height: "14",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        polyline { points: "9 18 15 12 9 6" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn type_label(kind: &SpaceActionType) -> &'static str {
    match kind {
        SpaceActionType::Poll => "POLL",
        SpaceActionType::Quiz => "QUIZ",
        SpaceActionType::TopicDiscussion => "DISCUSSION",
        SpaceActionType::Follow => "FOLLOW",
    }
}

#[cfg(feature = "web")]
fn scroll_to_card(action_id: &str) -> bool {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Some(document) = window.document() else {
        return false;
    };
    let selector = format!("[data-testid=\"quest-card-{}\"]", action_id.replace('"', ""));
    let Ok(Some(el)) = document.query_selector(&selector) else {
        return false;
    };
    if let Ok(html_el) = el.dyn_into::<web_sys::HtmlElement>() {
        html_el.scroll_into_view();
        true
    } else {
        false
    }
}

#[cfg(not(feature = "web"))]
fn scroll_to_card(_action_id: &str) -> bool {
    false
}

translate! {
    LockedDependenciesPopupTranslate;

    title: { en: "Complete these first", ko: "먼저 완료해 주세요" },
    subtitle: {
        en: "Finish the actions below to unlock this one.",
        ko: "아래 액션을 완료해야 이 액션이 열립니다."
    },
    scroll_failed: {
        en: "Could not locate that action on the dashboard.",
        ko: "대시보드에서 해당 액션을 찾을 수 없습니다."
    },
}
