mod menus;

use crate::*;
use common::hooks::use_scroll_lock;
use menus as i;
use ratel_auth::LoginModal;

translate! {
    AppMenuTranslate;

    home: {
        en: "Home",
        ko: "홈",
    },

    my_network: {
        en: "My Network",
        ko: "네트워크",
    },

    notification: {
        en: "Notification",
        ko: "알림",
    },

    membership: {
        en: "Membership",
        ko: "멤버십",
    },

    sign_in: {
        en: "Sign In",
        ko: "로그인",
    },

    join_the_movement: {
        en: "Join the Movement",
        ko: "참여하기",
    },
}

#[component]
pub fn AppMenu() -> Element {
    let tr: AppMenuTranslate = use_translate();
    let mut popup = use_popup();
    let user_ctx = ratel_auth::hooks::use_user_context();
    let mut mobile_menu_open = use_signal(|| false);
    use_scroll_lock(mobile_menu_open());

    let logged_in = user_ctx().is_logged_in();

    rsx! {
        header { class: "flex justify-center items-center py-2.5 px-2.5 border-b border-divider bg-bg! h-(--header-height) z-999 text-menu-text",
            nav { class: "flex justify-between items-center mx-2.5 w-full gap-12.5 max-w-desktop [&>path]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80",
                // Logo
                div { class: "flex gap-5 items-center",
                    Link { to: "/",
                        icons::ratel::Logo { class: "mobile:size-12 size-13.5" }
                    }
                }

                // Desktop nav items
                div { class: "flex gap-2.5 justify-center items-center max-tablet:hidden",
                    // Home
                    NavItem {
                        href: "/",
                        label: tr.home,
                        icon: rsx! {
                            i::HomeIcon {
                            }
                        },
                    }

                    // My Network (authorized only)
                    // if logged_in {
                    //     NavItem {
                    //         href: "/my-network",
                    //         label: tr.my_network,
                    //         icon: rsx! {
                    //             icons::user::UserGroup {}
                    //         },
                    //     }
                    // }

                    // Notification (authorized only)
                    // if logged_in {
                    //     NavItem {
                    //         href: "/notifications",
                    //         label: tr.notification,
                    //         icon: rsx! {
                    //             icons::notification::Bell { class: "transition-all [&>path]:stroke-menu-text group-hover:[&>path]:stroke-menu-text/80" }
                    //         },
                    //     }
                    // }

                    // Membership
                    NavItem {
                        href: "/membership",
                        label: tr.membership,
                        icon: rsx! {
                            i::MembershipIcon {}
                        },
                    }

                    ThemeSwitcher {}
                    // Language toggle
                    LanguageToggle {}

                    // Sign In / Profile
                    if logged_in {
                        ProfileDropdown {}
                    } else {
                        button {
                            class: "flex flex-col justify-center items-center p-2.5 font-bold cursor-pointer group text-menu-text text-[15px]",
                            onclick: move |_| {
                                popup.open(rsx! {
                                    LoginModal {}
                                }).with_title(tr.join_the_movement);
                            },
                            i::SignInIcon {}
                            span { class: "font-medium whitespace-nowrap transition-all text-menu-text text-[15px] group-hover:text-menu-text/80",
                                "{tr.sign_in}"
                            }
                        }
                    }
                }

                // Mobile hamburger / close toggle
                div {
                    class: "hidden cursor-pointer max-tablet:block",
                    onclick: move |_| {
                        mobile_menu_open.set(!mobile_menu_open());
                    },
                    if mobile_menu_open() {
                        icons::validations::Clear {
                            width: "32",
                            height: "32",
                            class: "transition-all [&>path]:stroke-menu-text",
                        }
                    } else {
                        icons::layouts::ThreeRow {
                            width: "32",
                            height: "32",
                            class: "transition-all [&>path]:stroke-menu-text",
                        }
                    }
                }
            }
        }

        // Mobile side menu
        MobileSideMenu { is_open: mobile_menu_open }
    }
}

#[component]
fn NavItem(href: &'static str, label: &'static str, icon: Element) -> Element {
    rsx! {
        Link {
            class: "flex flex-col justify-center items-center p-2.5 group",
            to: href,
            {icon}
            span { class: "font-medium whitespace-nowrap transition-all text-menu-text text-[15px] group-hover:text-menu-text/80",
                "{label}"
            }
        }
    }
}

#[component]
fn LanguageToggle() -> Element {
    let lang = use_language();

    let is_ko = lang().to_string() == "ko";
    let (flag, label) = if is_ko {
        (
            rsx! {
                i::KrIcon {
                    width: "16",
                    height: "16",
                    class: "object-cover rounded-full cursor-pointer",
                }
            },
            "KO",
        )
    } else {
        (
            rsx! {
                i::EnIcon {
                    width: "16",
                    height: "16",
                    class: "object-cover rounded-full cursor-pointer",
                }
            },
            "EN",
        )
    };

    rsx! {
        button {
            class: "flex flex-col justify-center items-center p-2.5 font-bold cursor-pointer group text-menu-text text-[15px]",
            onclick: move |_| {
                let next = lang().switch();
                debug!("Language switched to: {}", next);
            },
            div { class: "flex flex-col justify-center items-center h-6 w-fit", {flag} }
            span { class: "font-medium whitespace-nowrap transition-all text-menu-text text-[15px] group-hover:text-menu-text/80",
                "{label}"
            }
        }
    }
}
