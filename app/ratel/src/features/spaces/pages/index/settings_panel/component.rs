use crate::features::spaces::pages::index::*;

#[component]
pub fn SettingsPanel(open: bool, on_close: EventHandler<()>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let mut theme_service = use_theme();
    let current_theme = theme_service.current();
    let lang = use_language();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "settings-panel",
            "data-testid": "settings-panel",
            "data-open": open,
            div { class: "settings-panel__header",
                span { class: "settings-panel__title", "{tr.settings}" }
                button {
                    aria_label: "Close settings",
                    class: "settings-panel__close",
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
            div { class: "settings-panel__body",
                // Theme
                div { class: "settings-group",
                    span { class: "settings-group__label", "{tr.theme}" }
                    div { class: "settings-options",
                        div {
                            class: "settings-opt",
                            "data-testid": "theme-dark",
                            "aria-selected": matches!(current_theme, Theme::Dark),
                            onclick: move |_| {
                                theme_service.set(Theme::Dark);
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.5",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                            }
                            "{tr.dark}"
                        }
                        div {
                            class: "settings-opt",
                            "data-testid": "theme-light",
                            "aria-selected": matches!(current_theme, Theme::Light),
                            onclick: move |_| {
                                theme_service.set(Theme::Light);
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.5",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                circle { cx: "12", cy: "12", r: "4" }
                                path { d: "M12 2v2" }
                                path { d: "M12 20v2" }
                                path { d: "m4.93 4.93 1.41 1.41" }
                                path { d: "m17.66 17.66 1.41 1.41" }
                                path { d: "M2 12h2" }
                                path { d: "M20 12h2" }
                                path { d: "m6.34 17.66-1.41 1.41" }
                                path { d: "m19.07 4.93-1.41 1.41" }
                            }
                            "{tr.light}"
                        }
                        div {
                            class: "settings-opt",
                            "data-testid": "theme-system",
                            "aria-selected": matches!(current_theme, Theme::System),
                            onclick: move |_| {
                                theme_service.set(Theme::System);
                            },
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.5",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                rect {
                                    height: "14",
                                    rx: "2",
                                    width: "20",
                                    x: "2",
                                    y: "3",
                                }
                                line {
                                    x1: "8",
                                    x2: "16",
                                    y1: "21",
                                    y2: "21",
                                }
                                line {
                                    x1: "12",
                                    x2: "12",
                                    y1: "17",
                                    y2: "21",
                                }
                            }
                            "{tr.system}"
                        }
                    }
                }
                // Language
                div { class: "settings-group",
                    span { class: "settings-group__label", "{tr.language}" }
                    div { class: "settings-options",
                        div {
                            class: "settings-opt",
                            "data-testid": "lang-en",
                            "aria-selected": matches!(lang(), Language::En),
                            onclick: move |_| {
                                set_language(Language::En);
                            },
                            span { class: "settings-opt__icon", "EN" }
                            "{tr.english}"
                        }
                        div {
                            class: "settings-opt",
                            "data-testid": "lang-ko",
                            "aria-selected": {{ matches!(lang(), Language::Ko) }},
                            onclick: move |_| {
                                set_language(Language::Ko);
                            },
                            span { class: "settings-opt__icon", "KO" }
                            "{tr.korean}"
                        }
                    }
                }
            }
        }
    }
}
