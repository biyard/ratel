use crate::common::*;
use crate::features::auth::hooks::use_user_context;
use crate::features::auth::{LoginModal, SignupModal};
use crate::features::social::pages::team_arena::i18n::TeamArenaTranslate;
use crate::route::Route;

#[component]
pub fn ArenaSettingsPanel(
    open: bool,
    on_close: EventHandler<()>,
    username: String,
    #[props(default = false)] can_edit: bool,
) -> Element {
    let tr: TeamArenaTranslate = use_translate();
    let mut theme_service = use_theme();
    let current_theme = theme_service.current();
    let lang = use_language();
    let user_ctx = use_user_context();
    let is_logged_in = user_ctx.read().user.is_some();
    let mut popup = use_popup();
    let nav = use_navigator();

    rsx! {
        div {
            class: "ta-settings-panel__backdrop",
            "data-open": open,
            onclick: move |_| on_close.call(()),
        }
        div {
            class: "ta-settings-panel",
            "data-testid": "team-settings-panel",
            "data-open": open,

            div { class: "ta-settings-panel__header",
                span { class: "ta-settings-panel__title", "{tr.settings}" }
                button {
                    class: "ta-settings-panel__close",
                    aria_label: "Close settings",
                    r#type: "button",
                    onclick: move |_| on_close.call(()),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line {
                            x1: "18",
                            y1: "6",
                            x2: "6",
                            y2: "18",
                        }
                        line {
                            x1: "6",
                            y1: "6",
                            x2: "18",
                            y2: "18",
                        }
                    }
                }
            }

            div { class: "ta-settings-panel__body",
                // Theme
                div { class: "ta-settings-group",
                    span { class: "ta-settings-group__label", "{tr.theme}" }
                    div { class: "ta-settings-options",
                        div {
                            class: "ta-settings-opt",
                            role: "button",
                            tabindex: "0",
                            aria_selected: matches!(current_theme, Theme::Dark),
                            onclick: move |_| theme_service.set(Theme::Dark),
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                            }
                            "{tr.dark}"
                        }
                        div {
                            class: "ta-settings-opt",
                            role: "button",
                            tabindex: "0",
                            aria_selected: matches!(current_theme, Theme::Light),
                            onclick: move |_| theme_service.set(Theme::Light),
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
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
                            "{tr.light_mode}"
                        }
                        div {
                            class: "ta-settings-opt",
                            role: "button",
                            tabindex: "0",
                            aria_selected: matches!(current_theme, Theme::System),
                            onclick: move |_| theme_service.set(Theme::System),
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                rect {
                                    x: "2",
                                    y: "3",
                                    width: "20",
                                    height: "14",
                                    rx: "2",
                                }
                                line {
                                    x1: "8",
                                    y1: "21",
                                    x2: "16",
                                    y2: "21",
                                }
                                line {
                                    x1: "12",
                                    y1: "17",
                                    x2: "12",
                                    y2: "21",
                                }
                            }
                            "{tr.system}"
                        }
                    }
                }

                // Language
                div { class: "ta-settings-group",
                    span { class: "ta-settings-group__label", "{tr.language}" }
                    div { class: "ta-settings-options",
                        div {
                            class: "ta-settings-opt",
                            role: "button",
                            tabindex: "0",
                            aria_selected: matches!(lang(), Language::En),
                            onclick: move |_| set_language(Language::En),
                            span { class: "ta-settings-opt__icon", "EN" }
                            "{tr.english}"
                        }
                        div {
                            class: "ta-settings-opt",
                            role: "button",
                            tabindex: "0",
                            aria_selected: matches!(lang(), Language::Ko),
                            onclick: move |_| set_language(Language::Ko),
                            span { class: "ta-settings-opt__icon", "KO" }
                            "{tr.korean}"
                        }
                    }
                }

                // Actions (footer stays at bottom thanks to margin-top:auto)
                div { class: "ta-settings-footer",
                    if can_edit {
                        Link {
                            to: Route::TeamSetting {
                                username: username.clone(),
                            },
                            class: "ta-settings-action",
                            onclick: move |_| on_close.call(()),
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                                circle { cx: "12", cy: "12", r: "3" }
                            }
                            "{tr.team_settings_link}"
                        }
                    }

                    if is_logged_in {
                        button {
                            class: "ta-settings-action ta-settings-action--logout",
                            r#type: "button",
                            onclick: {
                                let username = username.clone();
                                move |_| {
                                    let username = username.clone();
                                    async move {
                                        // Centralized sign-out: flushes server session,
                                        // clears UserContext + cached refresh token,
                                        // reloads on web. Mobile doesn't reload, so we
                                        // navigate to TeamHome explicitly afterward
                                        // — staying on an admin-only sub-page would
                                        // render ViewerPage under the now-logged-out
                                        // state.
                                        crate::features::auth::services::sign_out(user_ctx).await;
                                        nav.replace(Route::TeamHome { username });
                                    }
                                }
                            },
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" }
                                polyline { points: "16 17 21 12 16 7" }
                                line {
                                    x1: "21",
                                    y1: "12",
                                    x2: "9",
                                    y2: "12",
                                }
                            }
                            "{tr.logout}"
                        }
                    } else {
                        button {
                            class: "ta-settings-action ta-settings-action--login",
                            r#type: "button",
                            onclick: move |_| {
                                on_close.call(());
                                popup.open(rsx! {
                                    LoginModal {}
                                });
                            },
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" }
                                polyline { points: "10 17 15 12 10 7" }
                                line {
                                    x1: "15",
                                    y1: "12",
                                    x2: "3",
                                    y2: "12",
                                }
                            }
                            "{tr.login}"
                        }
                        button {
                            class: "ta-settings-action",
                            r#type: "button",
                            onclick: move |_| {
                                on_close.call(());
                                popup.open(rsx! {
                                    SignupModal {}
                                });
                            },
                            "{tr.sign_up}"
                        }
                    }
                }
            }
        }
    }
}
