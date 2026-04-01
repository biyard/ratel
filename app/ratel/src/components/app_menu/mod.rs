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

    teams: {
        en: "Teams",
        ko: "팀",
    },

    admin: {
        en: "Admin",
        ko: "관리자",
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

    user_profile: {
        en: "User Profile",
        ko: "사용자 프로필",
    },
}

#[component]
pub fn AppMenu() -> Element {
    let lang = use_language();
    let tr: AppMenuTranslate = use_translate();
    let mut popup = use_popup();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let ctx = use_sidebar();
    // Use is_mobile_active() which also considers open_mobile, avoiding the
    // race where the Sheet opens before the async JS is_mobile eval resolves.
    let is_mobile = ctx.is_mobile_active();
    // On mobile, always show labels (expanded mode) since the Sheet has full width
    let collapsed = !is_mobile && (ctx.state)() == SidebarState::Collapsed;

    let logged_in = user_ctx().is_logged_in();
    let team_ctx = use_team_context();
    let teams = if is_mobile && logged_in {
        team_ctx.teams.read().clone()
    } else {
        Vec::new()
    };

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
                        href: Route::Index {},
                        label: tr.home,
                        collapsed,
                        icon: rsx! {
                            i::HomeIcon {}
                        },
                    }
                    MembershipMenuItem { label: tr.membership, collapsed }

                    if let Some(user) = user_ctx().user.as_ref() {
                        SidebarMenuItem {
                            Link {
                                to: format!("/{}/rewards", user.username),
                                class: "flex gap-2 items-center py-1.5 w-full text-sm rounded-md aria-extended:px-0 aria-extended:justify-center sidebar-menu-button hover:bg-accent/10 [&>svg]:w-5 [&>svg]:h-5",
                                "aria-extended": collapsed,
                                i::RewardsIcon {}
                                if !collapsed {
                                    span { "{tr.rewards}" }
                                }
                            }
                        }

                        if user.user_type == UserType::Admin {
                            SidebarMenuItem {
                                Link {
                                    to: Route::AdminMainPage {},
                                    class: "flex gap-2 items-center py-1.5 w-full text-sm rounded-md aria-extended:px-0 aria-extended:justify-center sidebar-menu-button hover:bg-accent/10 [&>svg]:w-5 [&>svg]:h-5",
                                    "aria-extended": collapsed,
                                    "data-testid": "admin-menu",
                                    i::AdminIcon {}
                                    if !collapsed {
                                        span { "{tr.admin}" }
                                    }
                                }
                            }
                        }

                        SidebarMenuItem {
                            Link {
                                to: format!("/{}/settings", user.username),
                                class: "flex gap-2 items-center py-1.5 w-full text-sm rounded-md aria-extended:px-0 aria-extended:justify-center sidebar-menu-button hover:bg-accent/10 [&>svg]:w-5 [&>svg]:h-5",
                                "aria-extended": collapsed,
                                lucide_dioxus::Settings {
                                    size: 20,
                                    class: "[&>path]:stroke-icon-primary [&>line]:stroke-icon-primary [&>circle]:stroke-icon-primary",
                                }
                                if !collapsed {
                                    span { "Settings" }
                                }
                            }
                        }
                    }
                }
            }

            // On mobile, show team list inline below nav items with separator
            if is_mobile && logged_in {
                SidebarSeparator {}
                SidebarGroup {
                    SidebarGroupLabel { class: "text-xs text-foreground-muted px-2",
                        "{tr.teams}"
                    }
                    SidebarMenu {
                        if let Some(user) = user_ctx().user.as_ref() {
                            SidebarMenuItem {
                                Link {
                                    to: format!("/{}", user.username),
                                    class: "flex gap-2 items-center py-1.5 w-full text-sm rounded-md sidebar-menu-button hover:bg-accent/10",
                                    "data-testid": "mobile-sidebar-user-link",
                                    if !user.profile_url.is_empty() {
                                        {
                                            let alt = if user.display_name.is_empty() { &user.username } else { &user.display_name };
                                            rsx! {
                                                img {
                                                    src: "{user.profile_url}",
                                                    alt: "{alt}",
                                                    class: "object-cover w-5 h-5 rounded-full",
                                                }
                                            }
                                        }
                                    } else {
                                        div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                                    }
                                    span { class: "truncate",
                                        if !user.display_name.is_empty() {
                                            "{user.display_name}"
                                        } else {
                                            "{user.username}"
                                        }
                                    }
                                }
                            }
                        }

                        for team in teams.iter() {
                            SidebarMenuItem {
                                key: "{team.username}",
                                Link {
                                    to: format!("/{}/home", team.username),
                                    class: "flex gap-2 items-center py-1.5 w-full text-sm rounded-md sidebar-menu-button hover:bg-accent/10",
                                    "data-testid": "mobile-sidebar-team-link",
                                    if !team.profile_url.is_empty() {
                                        {
                                            let alt = if team.nickname.is_empty() { &team.username } else { &team.nickname };
                                            rsx! {
                                                img {
                                                    src: "{team.profile_url}",
                                                    alt: "{alt}",
                                                    class: "object-cover w-5 h-5 rounded-full",
                                                }
                                            }
                                        }
                                    } else {
                                        div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                                    }
                                    span { class: "truncate",
                                        if !team.nickname.is_empty() {
                                            "{team.nickname}"
                                        } else {
                                            "{team.username}"
                                        }
                                    }
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
                        class: "app-menu-footer-button",
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
                        ProfileButton { collapsed, is_mobile }
                    }
                } else {
                    SidebarMenuItem {
                        SidebarMenuButton {
                            class: "app-menu-footer-button",
                            r#as: Callback::new(move |attrs: Vec<Attribute>| {
                                rsx! {
                                    button {
                                        "aria-label": "{tr.sign_in}",
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

                // Expand/Collapse toggle (hide on mobile since sidebar is a Sheet)
                if !is_mobile {
                    SidebarMenuItem {
                        SidebarMenuButton {
                            class: "app-menu-footer-button",
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
}

#[component]
fn NavMenuItem(href: Route, label: &'static str, collapsed: bool, icon: Element) -> Element {
    rsx! {
        SidebarMenuItem {
            Link {
                to: href,
                class: "flex gap-2 items-center py-1.5 w-full text-sm rounded-md aria-extended:px-0 aria-extended:justify-center sidebar-menu-button hover:bg-accent/10 [&>svg]:w-5 [&>svg]:h-5",
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
            class: "app-menu-footer-button",
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
/// On mobile, the team list is shown inline in the sidebar content instead of in the dropdown.
#[component]
fn ProfileButton(collapsed: bool, is_mobile: bool) -> Element {
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
    let teams = if is_mobile {
        Vec::new()
    } else {
        team_ctx.teams.read().clone()
    };

    // On mobile, position dropdown above the button; on desktop, to the right
    let dropdown_class = if is_mobile {
        "absolute bottom-full left-0 p-2 mb-2 rounded-lg border w-full border-divider bg-bg z-999"
    } else {
        "absolute bottom-0 left-full p-2 ml-2 rounded-lg border w-[220px] border-divider bg-bg z-999"
    };

    rsx! {
        div { class: "relative",
            SidebarMenuButton {
                class: "app-menu-footer-button",
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
                    let aria_label = if collapsed {
                        Some(tr.user_profile.to_string())
                    } else {
                        None
                    };
                    rsx! {
                        button {
                            "aria-label": aria_label,
                            "data-testid": "sidebar-profile-btn",
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

                div { class: "{dropdown_class}",
                    // User profile link
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
                            div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                        }
                        span { class: "text-sm truncate", "{user.display_name}" }
                    }

                    // Teams (only in desktop dropdown; on mobile they are inline in sidebar)
                    if !is_mobile {
                        for team in teams.iter() {
                            Link {
                                key: "{team.username}",
                                class: "flex gap-2 items-center py-1.5 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                                to: format!("/{}/home", team.username),
                                onclick: move |_| open.set(false),
                                if !team.profile_url.is_empty() {
                                    {
                                        let alt = if team.nickname.is_empty() { &team.username } else { &team.nickname };
                                        rsx! {
                                            img {
                                                src: "{team.profile_url}",
                                                alt: "{alt}",
                                                class: "object-cover w-5 h-5 rounded-full",
                                            }
                                        }
                                    }
                                } else {
                                    div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                                }
                                span { class: "text-sm truncate",
                                    if !team.nickname.is_empty() {
                                        "{team.nickname}"
                                    } else {
                                        "{team.username}"
                                    }
                                }
                            }
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
                        "data-testid": "sidebar-logout-btn",
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

#[cfg(feature = "membership")]
#[component]
fn MembershipMenuItem(label: &'static str, collapsed: bool) -> Element {
    rsx! {
        NavMenuItem {
            href: Route::MembershipHome {},
            label,
            collapsed,
            icon: rsx! {
                i::MembershipIcon {}
            },
        }
    }
}

#[cfg(not(feature = "membership"))]
#[component]
fn MembershipMenuItem(label: &'static str, collapsed: bool) -> Element {
    rsx! {}
}
