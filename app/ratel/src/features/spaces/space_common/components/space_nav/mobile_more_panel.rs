use crate::features::spaces::space_common::*;

translate! {
    MobileMorePanelTranslate;

    more: {
        en: "More",
        ko: "더보기",
    },

    sign_in: {
        en: "Sign In",
        ko: "로그인",
    },

    theme: {
        en: "Theme",
        ko: "테마",
    },

    language: {
        en: "Language",
        ko: "언어",
    },

    privacy_policy: {
        en: "Privacy Policy",
        ko: "개인정보처리방침",
    },

    terms_of_service: {
        en: "Terms of Service",
        ko: "서비스 이용약관",
    },

    dark: {
        en: "Dark",
        ko: "다크",
    },

    light: {
        en: "Light",
        ko: "라이트",
    },
}

/// "More" tab button shown only on mobile in the bottom navigation bar.
#[component]
pub fn MobileMoreTab(is_open: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let tr: MobileMorePanelTranslate = use_translate();

    rsx! {
        button {
            class: "hidden max-tablet:flex flex-1 flex-col gap-0.5 items-center py-2 px-1 text-sm font-medium rounded-sm cursor-pointer text-text hover:bg-space-nav-item-hover",
            "aria-selected": is_open,
            onclick: move |e| onclick.call(e),
            div { class: "h-6 w-6 flex items-center justify-center",
                lucide_dioxus::Ellipsis {
                    class: "w-5 h-5 [&>circle]:fill-current text-icon-primary",
                }
            }
            "{tr.more}"
        }
    }
}

/// Settings panel shown when the "More" tab is tapped on mobile.
#[component]
pub fn MobileMorePanel(
    is_logged_in: bool,
    on_close: EventHandler<MouseEvent>,
    on_login: EventHandler<()>,
) -> Element {
    let tr: MobileMorePanelTranslate = use_translate();
    let mut theme_service = use_theme();
    let is_dark = match theme_service.current() {
        Theme::Dark | Theme::System => true,
        Theme::Light => false,
    };
    let lang = use_language();
    let current_lang = lang();

    rsx! {
        BottomSheet {
            open: true,
            on_close: move |e| on_close.call(e),
            // Theme row
            Row {
                class: "w-full",
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Center,
                span { class: "text-sm font-medium text-foreground", "{tr.theme}" }
                Switch {
                    active: is_dark,
                    on_toggle: move |_| {
                        if is_dark {
                            theme_service.set(Theme::Light);
                        } else {
                            theme_service.set(Theme::Dark);
                        }
                    },
                }
            }

            Separator {}

            // Language row
            Row {
                class: "w-full",
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Center,
                span { class: "text-sm font-medium text-foreground", "{tr.language}" }
                Row {
                    class: "gap-1",
                    cross_axis_align: CrossAxisAlign::Center,
                    button {
                        class: "py-1 px-3 text-xs font-medium rounded-md cursor-pointer transition-colors bg-card-bg text-foreground-muted aria-selected:bg-primary aria-selected:text-btn-primary-text",
                        "aria-selected": matches!(current_lang, Language::En),
                        onclick: move |_| {
                            switch_language_to(Language::En);
                        },
                        "English"
                    }
                    button {
                        class: "py-1 px-3 text-xs font-medium rounded-md cursor-pointer transition-colors bg-card-bg text-foreground-muted aria-selected:bg-primary aria-selected:text-btn-primary-text",
                        "aria-selected": matches!(current_lang, Language::Ko),
                        onclick: move |_| {
                            switch_language_to(Language::Ko);
                        },
                        "한국어"
                    }
                }
            }

            if !is_logged_in {
                Separator {}

                // Sign In button
                Button {
                    class: "w-full",
                    size: ButtonSize::Small,
                    style: ButtonStyle::Primary,
                    onclick: move |_| {
                        on_login.call(());
                    },
                    "{tr.sign_in}"
                }
            }

            Separator {}

            // Privacy & Terms links
            Col {
                class: "gap-2",
                a {
                    class: "text-xs text-foreground-muted hover:text-foreground cursor-pointer",
                    href: "https://ratel.foundation/privacy",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "{tr.privacy_policy}"
                }
                a {
                    class: "text-xs text-foreground-muted hover:text-foreground cursor-pointer",
                    href: "https://ratel.foundation/terms",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "{tr.terms_of_service}"
                }
            }
        }
    }
}

/// Sets the language to a specific value and persists it to localStorage and cookie.
fn switch_language_to(target: Language) {
    set_language(target);

    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::wasm_bindgen::JsCast;

        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item(dioxus_translate::STORAGE_KEY, &target.to_string());
            }

            if let Some(doc) = window.document() {
                let html_document = doc.dyn_into::<web_sys::HtmlDocument>().unwrap();
                let _ =
                    html_document.set_cookie(&format!("language={}; path=/;", target));
            }
        }
    }
}
