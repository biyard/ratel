use crate::*;
use crate::common::components::sidebar::*;
use crate::features::auth::LoginModal;
use crate::features::posts::controllers::create_post::create_post_handler;

translate! {
    MobileBottomNavTranslate;

    home: {
        en: "Home",
        ko: "홈",
    },

    more: {
        en: "More",
        ko: "더보기",
    },

    create_post: {
        en: "Create Post",
        ko: "글쓰기",
    },

    sign_in: {
        en: "Sign In",
        ko: "로그인",
    },

    join_the_movement: {
        en: "Join the Movement",
        ko: "참여하기",
    },

    theme: {
        en: "Theme",
        ko: "테마",
    },

    language: {
        en: "Language",
        ko: "언어",
    },

    close_menu: {
        en: "Close menu",
        ko: "메뉴 닫기",
    },

    mobile_navigation: {
        en: "Mobile navigation",
        ko: "모바일 내비게이션",
    },
}

/// Home icon for the bottom nav (inline SVG to avoid name collision with component `Home`).
#[component]
fn BottomNavHomeIcon() -> Element {
    rsx! {
        svg {
            width: "22",
            height: "22",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            class: "[&>path]:stroke-foreground-muted",
            path {
                d: "M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

/// Create Post button for the mobile bottom nav bar.
#[component]
fn MobileCreatePostButton(label: String) -> Element {
    let nav = use_navigator();
    let mut toast = use_toast();

    rsx! {
        button {
            class: "flex justify-center items-center w-12 h-12 rounded-full cursor-pointer bg-primary -mt-4 shadow-lg",
            "aria-label": "{label}",
            "data-testid": "mobile-create-post-btn",
            onclick: move |_| {
                let nav = nav.clone();
                async move {
                    match create_post_handler(None).await {
                        Ok(resp) => {
                            let post_pk: FeedPartition = resp.post_pk.into();
                            nav.push(format!("/posts/{post_pk}/edit"));
                        }
                        Err(e) => {
                            dioxus::logger::tracing::error!("Failed to create post: {:?}", e);
                            toast.error(e);
                        }
                    }
                }
            },
            lucide_dioxus::Plus {
                size: 24,
                class: "[&>path]:stroke-background",
            }
        }
    }
}

/// More menu panel for unauthenticated users (theme, language, sign in).
#[component]
fn MoreMenuPanel(show: Signal<bool>) -> Element {
    let tr: MobileBottomNavTranslate = use_translate();
    let mut popup = use_popup();
    let lang = use_language();
    let mut theme_service = use_theme();
    let current_theme = theme_service.current();

    let next_theme = match current_theme {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::System,
        Theme::System => Theme::Light,
    };

    rsx! {
        // Backdrop
        button {
            class: "fixed inset-0 z-[998] md:hidden",
            "data-testid": "mobile-more-backdrop",
            r#type: "button",
            "aria-label": "{tr.close_menu}",
            onclick: move |_| show.set(false),
        }

        // Menu panel
        div {
            class: "fixed left-0 z-[999] w-full border-t md:hidden border-separator bg-background bottom-[calc(theme(spacing.14)+env(safe-area-inset-bottom))] pb-[env(safe-area-inset-bottom)]",
            "data-testid": "mobile-more-panel",

            div { class: "flex flex-col py-2",
                // Theme toggle
                button {
                    class: "flex gap-3 items-center py-3 px-4 w-full cursor-pointer hover:bg-hover",
                    onclick: move |_| {
                        theme_service.set(next_theme);
                        show.set(false);
                    },
                    {current_theme.icon()}
                    span { class: "text-sm text-foreground", "{tr.theme}: {current_theme.label()}" }
                }

                // Language toggle
                button {
                    class: "flex gap-3 items-center py-3 px-4 w-full cursor-pointer hover:bg-hover",
                    onclick: move |_| {
                        lang().switch();
                        show.set(false);
                    },
                    lucide_dioxus::Globe {
                        size: 20,
                        class: "[&>path]:stroke-icon-primary [&>circle]:stroke-icon-primary [&>line]:stroke-icon-primary",
                    }
                    span { class: "text-sm text-foreground uppercase",
                        "{tr.language}: {lang()}"
                    }
                }

                // Divider
                div { class: "mx-4 my-1 h-px bg-separator" }

                // Sign In
                button {
                    class: "flex gap-3 items-center py-3 px-4 w-full cursor-pointer hover:bg-hover",
                    "data-testid": "mobile-sign-in-btn",
                    onclick: move |_| {
                        show.set(false);
                        popup.open(rsx! {
                            LoginModal {}
                        })
                        .with_title(tr.join_the_movement);
                    },
                    lucide_dioxus::LogIn {
                        size: 20,
                        class: "[&>path]:stroke-icon-primary [&>line]:stroke-icon-primary [&>polyline]:stroke-icon-primary",
                    }
                    span { class: "text-sm text-foreground", "{tr.sign_in}" }
                }
            }
        }
    }
}

/// A fixed bottom navigation bar shown only on mobile screens (< 768px).
///
/// - Not logged in: Home, More (opens a popup with theme, language, login)
/// - Logged in: Home, Create Post (+), More (opens sidebar sheet)
#[component]
pub fn MobileBottomNav() -> Element {
    let tr: MobileBottomNavTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let ctx = use_sidebar();
    let logged_in = user_ctx().is_logged_in();
    let mut show_more_menu = use_signal(|| false);

    rsx! {
        // Bottom navigation bar - only visible on mobile (< md breakpoint = 768px)
        nav {
            class: "fixed bottom-0 left-0 z-50 w-full border-t md:hidden border-separator bg-background pb-[env(safe-area-inset-bottom)]",
            "data-testid": "mobile-bottom-nav",
            "aria-label": "{tr.mobile_navigation}",
            div { class: "flex justify-around items-center h-14",
                // Home button
                Link {
                    to: "/",
                    class: "flex flex-col gap-0.5 items-center py-1.5 px-3",
                    "aria-label": "{tr.home}",
                    onclick: move |_| {
                        show_more_menu.set(false);
                    },
                    BottomNavHomeIcon {}
                    span { class: "text-[10px] text-foreground-muted", "{tr.home}" }
                }

                // Create Post button (only for logged-in users)
                if logged_in {
                    MobileCreatePostButton { label: tr.create_post.to_string() }
                }

                // More button
                button {
                    class: "flex flex-col gap-0.5 items-center py-1.5 px-3 cursor-pointer",
                    "aria-label": "{tr.more}",
                    "data-testid": "mobile-more-btn",
                    onclick: move |_| {
                        if logged_in {
                            ctx.set_open_mobile(true);
                        } else {
                            show_more_menu.set(!show_more_menu());
                        }
                    },
                    lucide_dioxus::Menu {
                        size: 22,
                        class: "[&>path]:stroke-foreground-muted [&>line]:stroke-foreground-muted",
                    }
                    span { class: "text-[10px] text-foreground-muted", "{tr.more}" }
                }
            }
        }

        // More menu overlay for non-logged-in users
        if !logged_in && show_more_menu() {
            MoreMenuPanel { show: show_more_menu }
        }
    }
}
