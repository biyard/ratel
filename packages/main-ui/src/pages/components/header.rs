use bdk::prelude::{
    by_components::icons::{arrows::ShapeArrowDown, edit::Search, home::Home1},
    dioxus_popup::PopupService,
    *,
};

use super::*;
use crate::{
    components::{icons::RatelSymbolWithText, popup::SignupPopup},
    pages::RouteTab,
    route::Route,
    services::user_service::UserService,
};

#[component]
pub fn SocialHeader(
    lang: Language,
    onsearch: EventHandler<String>,
    current_page: RouteTab,
    onroute: EventHandler<MouseEvent>,
) -> Element {
    let mut search_extend = use_signal(|| false);
    let tr: SocialHeaderTranslate = translate(&lang);

    let user_service: UserService = use_context();
    let mut popup: PopupService = use_context();
    let (is_login, nickname, _email, profile_url) = match user_service.get_user_info() {
        Some(v) => (true, v.0, v.1, v.2),
        None => (false, "".to_string(), "".to_string(), "".to_string()),
    };
    rsx! {
        div { class: "w-screen h-80 flex items-center justify-center z-100 max-tablet:!h-48",
            div { class: "w-full max-w-desktop m-10 flex flex-row justify-between items-center",
                div { class: "hidden max-tablet:flex max-tablet:flex-row max-tablet:items-center max-tablet:justify-between max-tablet:w-full max-tablet:px-16 max-tablet:py-4",
                    div {
                        class: "flex flex-row w-fit justify-start items-center gap-10 aria-expanded:w-full",
                        aria_expanded: search_extend(),
                        // div {
                        //     class: "cursor-pointer flex flex-col w-fit justify-center items-center p-10",
                        //     onclick: move |e| {
                        //         onroute.call(e);
                        //     },
                        //     Bell {
                        //         class: "[&>path]:stroke-neutral-500 [&>path]:fill-neutral-500",
                        //         width: "32",
                        //         height: "32",
                        //     }
                        // }

                        div {
                            class: "flex flex-row w-fit justify-start items-center gap-10 aria-expanded:w-full transition-all duration-300 ease-in-out",
                            aria_expanded: search_extend(),

                            div {
                                class: format_args!(
                                    "transition-all duration-300 ease-in-out {}",
                                    if search_extend() {
                                        "opacity-0 scale-95 pointer-events-none"
                                    } else {
                                        "opacity-100 scale-100"
                                    },
                                ),
                                onclick: move |_| search_extend.set(true),
                                Search {
                                    class: "[&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500",
                                    width: "32",
                                    height: "32",
                                }
                            }

                            div {
                                class: format_args!(
                                    "transition-all duration-300 ease-in-out w-full {}",
                                    if search_extend() {
                                        "opacity-100 scale-100"
                                    } else {
                                        "opacity-0 scale-95 hidden"
                                    },
                                ),
                                MobileSearchBox {
                                    lang,
                                    onsearch,
                                    onextend: move |_| search_extend.set(false),
                                }
                            }
                        }
                    }

                    div {
                        class: format_args!(
                            "transition-all duration-300 ease-in-out {}",
                            if search_extend() {
                                "opacity-0 scale-95 hidden"
                            } else {
                                "opacity-100 scale-100"
                            },
                        ),
                        RatelSymbolWithText { size: 36 }
                    }

                    if profile_url.clone() == "" {
                        div { class: "w-80 h-80 rounded-full bg-neutral-400" }
                    } else {
                        img {
                            class: format_args!(
                                "w-30 h-30 rounded-full object-cover transition-all duration-300 ease-in-out {}",
                                if search_extend() {
                                    "opacity-0 scale-95 hidden"
                                } else {
                                    "opacity-100 scale-100"
                                },
                            ),
                            src: profile_url.clone(),
                        }
                    }

                }
                div { class: "flex flex-row gap-20 items-center max-tablet:z-100 max-tablet:!hidden",
                    RatelSymbolWithText {}
                    SearchBox { lang, onsearch }
                }

                div { class: "flex flex-row gap-10 items-center max-tablet:!hidden",
                    Link { class: "social-menu-item", to: Route::IndexPage {},
                        Home1 {
                            class: "[&>path]:stroke-[#737373]",
                            width: "24",
                            height: "24",
                        }
                        {tr.home}
                    }

                    Link {
                        class: "social-menu-item",
                        to: Route::PresidentialElectionPage {},
                        by_components::icons::internet_script::Internet {
                            class: "[&>path]:stroke-[#737373] [&>path]:fill-[#737373] [&>circle]:stroke-[#737373]",
                            width: "24",
                            height: "24",
                        }
                        {tr.election}
                    }
                    Link {
                        class: "social-menu-item",
                        to: Route::PoliticiansPage {},
                        by_components::icons::user::UserGroup {
                            class: "[&>path]:stroke-[#737373]",
                            width: "24",
                            height: "24",
                        }
                        {tr.politicians}
                    }

                    // Menu {
                    //     icon: rsx! {
                    //         Internet {
                    //             class: "[&>path]:stroke-[#737373] [&>path]:fill-[#737373] [&>circle]:stroke-[#737373]",
                    //             width: "24",
                    //             height: "24",
                    //         }
                    //     },
                    //     link: Route::ExplorePage {},
                    //     text: tr.explore,
                    // }
                    // Menu {
                    //     icon: rsx! {
                    //         UserGroup { class: "[&>path]:stroke-[#737373]", width: "24", height: "24" }
                    //     },
                    //     link: Route::MyNetworkPage {},
                    //     text: tr.my_network,
                    // }
                    // Menu {
                    //     icon: rsx! {
                    //         RoundBubble {
                    //             class: "[&>path]:stroke-[#737373] [&>line]:stroke-[#737373]",
                    //             width: "24",
                    //             height: "24",
                    //             fill: "none",
                    //         }
                    //     },
                    //     link: Route::MessagesPage {},
                    //     text: tr.message,
                    // }
                    // Menu {
                    //     icon: rsx! {
                    //         Bell {
                    //             class: "[&>path]:stroke-[#737373] [&>path]:fill-[#737373]",
                    //             width: "24",
                    //             height: "24",
                    //         }
                    //     },
                    //     link: Route::NotificationsPage {},
                    //     text: tr.notification,
                    // }

                    if is_login {
                        Profile { url: profile_url.clone(), name: nickname }
                    } else {
                        button {
                            class: "p-10 text-[15px] font-bold text-secondary hover:text-hover cursor-pointer max-tablet:!px-44 max-tablet:!py-15 order-1  max-tablet:!order-2",
                            onclick: move |_| {
                                tracing::debug!("Sign in clicked");
                                popup.open(rsx! {
                                    SignupPopup { lang }
                                }).with_id("signup_popup");
                            },
                            {tr.sign_in}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Profile(url: String, name: String) -> Element {
    rsx! {
        Link {
            class: "flex flex-col w-fit justify-center items-center p-10",
            to: Route::MyProfilePage {},
            img { class: "w-24 h-24 rounded-full object-cover", src: url }

            div { class: "flex flex-row w-fit h-fit justify-center items-center gap-1 py-3",
                div { class: "font-medium text-[15px]/18 text-neutral-500", {name} }
                ShapeArrowDown {
                    class: "[&>path]:stroke-[#737373] [&>path]:fill-[#737373]",
                    size: 12,
                    fill: "#737373",
                }
            }
        }
    }
}

translate! {
    SocialHeaderTranslate;

    home: {
        ko: "Home",
        en: "Home"
    },

    election: {
        ko: "Election",
        en: "Election"
    }

    politicians: {
        ko: "Politicians",
        en: "Politicians"
    }

    explore: {
        ko: "Explore",
        en: "Explore"
    },
    my_network: {
        ko: "My Network",
        en: "My Network"
    },
    message: {
        ko: "Message",
        en: "Message"
    },
    notification: {
        ko: "Notification",
        en: "Notification"
    },
    sign_in: {
        ko: "Sign in",
        en: "Sign in"
    }
}
