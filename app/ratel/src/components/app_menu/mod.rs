mod menus;

use crate::common::components::sidebar::*;
use crate::features::auth::LoginModal;
use crate::*;
use menus as i;

translate! {
    AppMenuTranslate;

    home: {
        en: "Home",
        ko: "홈",
    },

    membership: {
        en: "Membership",
        ko: "멤버십",
    },

    rewards: {
        en: "Rewards",
        ko: "보상",
    },

    sign_in: {
        en: "Sign In",
        ko: "로그인",
    },

    join_the_movement: {
        en: "Join the Movement",
        ko: "참여하기",
    },

    logout: {
        en: "Log Out",
        ko: "로그아웃",
    },
}

#[component]
pub fn AppMenu() -> Element {
    let lang = use_language();
    let tr: AppMenuTranslate = use_translate();
    let mut popup = use_popup();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let ctx = use_sidebar();
    let collapsed = (ctx.state)() == SidebarState::Collapsed;

    let logged_in = user_ctx().is_logged_in();

    rsx! {
        SidebarHeader { class: "flex justify-between items-center p-4",
            Link { to: "/",
                icons::ratel::Logo { class: "size-10" }
            }
        }

        SidebarContent {
            SidebarGroup {
                SidebarMenu {
                    NavMenuItem {
                        href: "/",
                        label: tr.home,
                        collapsed,
                        icon: rsx! {
                            i::HomeIcon {}
                        },
                    }
                    NavMenuItem {
                        href: "/membership",
                        label: tr.membership,
                        collapsed,
                        icon: rsx! {
                            i::MembershipIcon {}
                        },
                    }

                    if let Some(user) = user_ctx().user.as_ref() {
                        SidebarMenuItem {
                            Link {
                                to: format!("/{}/rewards", user.username),
                                class: "flex gap-2 items-center py-1.5 w-full text-sm rounded-md aria-extended:px-2 sidebar-menu-button hover:bg-accent/10",
                                "aria-extended": collapsed,
                                i::RewardsIcon {}
                                if !collapsed {
                                    span { "{tr.rewards}" }
                                }
                            }
                        }
                    }
                }
            }
        }

        SidebarFooter { class: "px-2 pb-4",
            SidebarMenu {
                // Language toggle
                SidebarMenuItem {
                    SidebarMenuButton {
                        r#as: Callback::new(move |attrs: Vec<Attribute>| {
                            rsx! {
                                button {
                                    onclick: move |_| {
                                        lang().switch();
                                    },
                                    ..attrs,
                                    LanguageIcon {}
                                    if !collapsed {
                                        span { class: "uppercase", {lang.to_string()} }
                                    }
                                }
                            }
                        }),
                    }
                }

                // Theme toggle
                SidebarMenuItem {
                    ThemeToggleButton { collapsed }
                }

                // Profile or Sign In
                if logged_in {
                    SidebarMenuItem {
                        ProfileButton { collapsed }
                    }
                } else {
                    SidebarMenuItem {
                        SidebarMenuButton {
                            r#as: Callback::new(move |attrs: Vec<Attribute>| {
                                rsx! {
                                    button {
                                        onclick: move |_| {
                                            popup.open(rsx! {
                                                LoginModal {}
                                            }).with_title(tr.join_the_movement);
                                        },
                                        ..attrs,
                                        i::SignInIcon {}
                                        if !collapsed {
                                            span { "{tr.sign_in}" }
                                        }
                                    }
                                }
                            }),
                        }
                    }
                }

                // Expand/Collapse toggle
                SidebarMenuItem {
                    SidebarMenuButton {
                        r#as: Callback::new(move |attrs: Vec<Attribute>| {
                            rsx! {
                                button {
                                    onclick: move |_| {
                                        ctx.toggle();
                                    },
                                    ..attrs,
                                    if collapsed {
                                        lucide_dioxus::PanelLeftOpen {
                                            size: 20,
                                            class: "[&>path]:stroke-icon-primary [&>rect]:stroke-icon-primary",
                                        }
                                    } else {
                                        lucide_dioxus::PanelLeftClose {
                                            size: 20,
                                            class: "[&>path]:stroke-icon-primary [&>rect]:stroke-icon-primary",
                                        }
                                        span { "Collapse" }
                                    }
                                }
                            }
                        }),
                    }
                }
            }
        }
    }
}

#[component]
fn NavMenuItem(href: &'static str, label: &'static str, collapsed: bool, icon: Element) -> Element {
    rsx! {
        SidebarMenuItem {
            Link {
                to: href,
                class: "flex gap-2 items-center py-1.5 w-full text-sm rounded-md aria-extended:px-2 sidebar-menu-button hover:bg-accent/10",
                "aria-extended": collapsed,
                {icon}
                if !collapsed {
                    span { "{label}" }
                }
            }
        }
    }
}

/// Theme toggle rendered as a SidebarMenuButton — icon only when collapsed.
#[component]
fn ThemeToggleButton(collapsed: bool) -> Element {
    let mut theme_service = use_theme();
    let current = theme_service.current();

    let next = match current {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::System,
        Theme::System => Theme::Light,
    };

    rsx! {
        SidebarMenuButton {
            r#as: Callback::new(move |attrs: Vec<Attribute>| {
                rsx! {
                    button {
                        onclick: move |_| {
                            theme_service.set(next);
                        },
                        ..attrs,
                        {current.icon()}
                        if !collapsed {
                            span { "{current.label()}" }
                        }
                    }
                }
            }),
        }
    }
}

/// Profile button rendered as a SidebarMenuButton — avatar only when collapsed, with dropdown.
#[component]
fn ProfileButton(collapsed: bool) -> Element {
    let tr: AppMenuTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let team_ctx = use_team_context();
    let mut open = use_signal(|| false);
    let mut popup = use_popup();
    let nav = use_navigator();

    let user = user_ctx().user.clone();
    let Some(user) = user else {
        return rsx! {};
    };

    let profile_url = user.profile_url.clone();
    let display_name = user.display_name.clone();
    let teams = team_ctx.teams.read().clone();

    rsx! {
        div { class: "relative",
            SidebarMenuButton {
                r#as: Callback::new(move |attrs: Vec<Attribute>| {
                    let profile_url = profile_url.clone();
                    let display_name = display_name.clone();
                    let avatar_class = if collapsed {
                        "w-8 aspect-square rounded-full object-cover"
                    } else {
                        "w-5 aspect-square rounded-full object-cover"
                    };
                    let placeholder_class = if collapsed {
                        "w-8 h-8 bg-neutral-500 rounded-full"
                    } else {
                        "w-5 h-5 bg-neutral-500 rounded-full"
                    };
                    rsx! {
                        button {
                            onclick: move |_| {
                                open.set(!open());
                            },
                            ..attrs,
                            if !profile_url.is_empty() {
                                img { src: "{profile_url}", alt: "Profile", class: "{avatar_class}" }
                            } else {
                                div { class: "{placeholder_class}" }
                            }
                            if !collapsed {
                                span { class: "truncate max-w-24", "{display_name}" }
                            }
                        }
                    }
                }),
            }

            if open() {
                div {
                    class: "fixed inset-0 z-998",
                    onclick: move |_| open.set(false),
                }

                div { class: "absolute bottom-0 left-full p-2 ml-2 rounded-lg border w-[220px] border-divider bg-bg z-999",
                    // User
                    Link {
                        class: "flex gap-2 items-center py-1.5 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                        to: "/",
                        onclick: move |_| open.set(false),
                        if !user.profile_url.is_empty() {
                            img {
                                src: "{user.profile_url}",
                                alt: "{user.display_name}",
                                class: "object-cover w-5 h-5 rounded-full",
                            }
                        } else {
                            div { class: "w-5 h-5 rounded-full bg-neutral-600" }
                        }
                        span { class: "text-sm truncate", "{user.display_name}" }
                    }

                    // Teams
                    for team in teams.iter() {
                        Link {
                            class: "flex gap-2 items-center py-1.5 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                            to: format!("/{}/home", team.username),
                            onclick: move |_| open.set(false),
                            if !team.profile_url.is_empty() {
                                img {
                                    src: "{team.profile_url}",
                                    alt: "{team.nickname}",
                                    class: "object-cover w-5 h-5 rounded-full",
                                }
                            } else {
                                div { class: "w-5 h-5 rounded-full bg-neutral-600" }
                            }
                            span { class: "text-sm truncate", "{team.nickname}" }
                        }
                    }

                    div { class: "my-1.5 h-px bg-divider" }

                    // Create Team
                    button {
                        class: "py-1.5 px-2 w-full text-sm text-left rounded-md cursor-pointer hover:bg-hover",
                        onclick: move |_| {
                            open.set(false);
                            popup.open(rsx! {
                                TeamCreationPopup {}
                            });
                        },
                        "Create Team"
                    }

                    // Logout
                    button {
                        class: "py-1.5 px-2 w-full text-sm text-left rounded-md cursor-pointer hover:bg-hover",
                        onclick: move |_| {
                            open.set(false);
                            spawn(async move {
                                let _ = crate::features::auth::controllers::logout_handler().await;
                                nav.push("/");
                                #[cfg(target_arch = "wasm32")]
                                {
                                    if let Some(window) = web_sys::window() {
                                        let _ = window.location().reload();
                                    }
                                }
                            });
                        },
                        "{tr.logout}"
                    }
                }
            }
        }
    }
}

#[component]
fn LanguageIcon() -> Element {
    let lang = use_language();

    if lang() == Language::Ko {
        rsx! {
            i::KrIcon {
                width: "16",
                height: "16",
                class: "object-cover rounded-full",
            }
        }
    } else {
        rsx! {
            i::EnIcon {
                width: "16",
                height: "16",
                class: "object-cover rounded-full",
            }
        }
    }
}
