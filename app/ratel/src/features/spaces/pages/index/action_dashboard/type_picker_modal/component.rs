use crate::features::spaces::pages::actions::types::SpaceActionType;
use crate::features::spaces::pages::index::*;

#[component]
pub fn TypePickerModal(
    open: bool,
    on_close: EventHandler<()>,
    on_pick: EventHandler<SpaceActionType>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();

    if !open {
        return rsx! {};
    }

    rsx! {
        div {
            class: "type-sheet open",
            "data-testid": "type-picker-modal",
            onclick: move |_| {
                on_close.call(());
            },
            div {
                class: "type-sheet__panel",
                role: "dialog",
                onclick: move |e| {
                    e.stop_propagation();
                },
                div { class: "type-sheet__header",
                    span { class: "type-sheet__title", "{tr.choose_action_type}" }
                    button {
                        aria_label: "Close",
                        class: "type-sheet__close",
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
                div { class: "type-grid",
                    // Poll
                    button {
                        class: "type-option",
                        "data-testid": "type-option-poll",
                        "data-type": "poll",
                        onclick: move |_| {
                            on_pick.call(SpaceActionType::Poll);
                            on_close.call(());
                        },
                        div { class: "type-option__icon",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M18 20V10" }
                                path { d: "M12 20V4" }
                                path { d: "M6 20v-6" }
                            }
                        }
                        div { class: "type-option__name", "{tr.poll_name}" }
                        div { class: "type-option__desc", "{tr.poll_desc}" }
                    }
                    // Discussion
                    button {
                        class: "type-option",
                        "data-testid": "type-option-discuss",
                        "data-type": "discuss",
                        onclick: move |_| {
                            on_pick.call(SpaceActionType::TopicDiscussion);
                            on_close.call(());
                        },
                        div { class: "type-option__icon",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                            }
                        }
                        div { class: "type-option__name", "{tr.discussion_name}" }
                        div { class: "type-option__desc", "{tr.discussion_desc}" }
                    }
                    // Quiz
                    button {
                        class: "type-option",
                        "data-testid": "type-option-quiz",
                        "data-type": "quiz",
                        onclick: move |_| {
                            on_pick.call(SpaceActionType::Quiz);
                            on_close.call(());
                        },
                        div { class: "type-option__icon",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                circle { cx: "12", cy: "12", r: "10" }
                                path { d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
                                line {
                                    x1: "12",
                                    x2: "12.01",
                                    y1: "17",
                                    y2: "17",
                                }
                            }
                        }
                        div { class: "type-option__name", "{tr.quiz_name}" }
                        div { class: "type-option__desc", "{tr.quiz_desc}" }
                    }
                    // Follow
                    button {
                        class: "type-option",
                        "data-testid": "type-option-follow",
                        "data-type": "follow",
                        onclick: move |_| {
                            on_pick.call(SpaceActionType::Follow);
                            on_close.call(());
                        },
                        div { class: "type-option__icon",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                                circle { cx: "9", cy: "7", r: "4" }
                                line {
                                    x1: "19",
                                    x2: "19",
                                    y1: "8",
                                    y2: "14",
                                }
                                line {
                                    x1: "22",
                                    x2: "16",
                                    y1: "11",
                                    y2: "11",
                                }
                            }
                        }
                        div { class: "type-option__name", "{tr.follow_name}" }
                        div { class: "type-option__desc", "{tr.follow_desc}" }
                    }
                }
            }
        }
    }
}
