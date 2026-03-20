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
            class: "hidden max-tablet:flex flex-col gap-0.5 items-center py-2 px-1 text-sm font-medium rounded-sm cursor-pointer text-text hover:bg-space-nav-item-hover",
            "aria-selected": is_open,
            onclick: move |e| onclick.call(e),
            div { class: "h-6 w-6 flex items-center justify-center",
                lucide_dioxus::Ellipsis {
                    class: "w-5 h-5 [&>circle]:fill-current text-[#737373]",
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
        // Backdrop
        div {
            class: "hidden max-tablet:block fixed inset-0 z-30 bg-black/50",
            onclick: move |e| on_close.call(e),
        }
        // Panel slides up from bottom
        div { class: "hidden max-tablet:flex fixed bottom-16 left-0 right-0 z-40 flex-col gap-4 p-5 bg-background rounded-t-xl border-t border-border",
            // Theme row
            Row {
                class: "w-full",
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Center,
                span { class: "text-sm font-medium text-foreground", "{tr.theme}" }
                Row {
                    class: "gap-1",
                    cross_axis_align: CrossAxisAlign::Center,
                    button {
                        class: "py-1 px-3 text-xs font-medium rounded-md cursor-pointer transition-colors",
                        class: if is_dark { "bg-primary text-btn-primary-text" } else { "bg-card-bg text-foreground-muted" },
                        onclick: move |_| {
                            theme_service.set(Theme::Dark);
                        },
                        "{tr.dark}"
                    }
                    button {
                        class: "py-1 px-3 text-xs font-medium rounded-md cursor-pointer transition-colors",
                        class: if !is_dark { "bg-primary text-btn-primary-text" } else { "bg-card-bg text-foreground-muted" },
                        onclick: move |_| {
                            theme_service.set(Theme::Light);
                        },
                        "{tr.light}"
                    }
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
                        class: "py-1 px-3 text-xs font-medium rounded-md cursor-pointer transition-colors",
                        class: if matches!(current_lang, Language::En) { "bg-primary text-btn-primary-text" } else { "bg-card-bg text-foreground-muted" },
                        onclick: move |_| {
                            set_language(Language::En);
                        },
                        "English"
                    }
                    button {
                        class: "py-1 px-3 text-xs font-medium rounded-md cursor-pointer transition-colors",
                        class: if matches!(current_lang, Language::Ko) { "bg-primary text-btn-primary-text" } else { "bg-card-bg text-foreground-muted" },
                        onclick: move |_| {
                            set_language(Language::Ko);
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
                    "{tr.privacy_policy}"
                }
                a {
                    class: "text-xs text-foreground-muted hover:text-foreground cursor-pointer",
                    href: "https://ratel.foundation/terms",
                    target: "_blank",
                    "{tr.terms_of_service}"
                }
            }
        }
    }
}
