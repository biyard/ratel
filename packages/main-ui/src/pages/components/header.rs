use bdk::prelude::{
    by_components::icons::{
        arrows::ShapeArrowDown, chat::RoundBubble, home::Home1, internet_script::Internet,
        notification::Bell, user::UserGroup,
    },
    dioxus_popup::PopupService,
    *,
};

use super::*;
use crate::{
    components::{icons::RatelSymbolWithText, popup::SignupPopup},
    route::Route,
    services::user_service::UserService,
};

#[component]
pub fn SocialHeader(lang: Language, onsearch: EventHandler<String>) -> Element {
    let tr: SocialHeaderTranslate = translate(&lang);

    let user_service: UserService = use_context();
    let mut popup: PopupService = use_context();
    let user = user_service.user_info();

    let is_login = user.email.is_some();
    let (nickname, profile_url) = if is_login {
        (
            user.nickname.unwrap_or_default(),
            user.profile_url.unwrap_or_default(),
        )
    } else {
        (String::new(), String::new())
    };

    rsx! {
        nav { class: "w-full max-w-desktop m-10 flex flex-row justify-between items-center",
            div { class: "flex flex-row gap-20 items-center",
                RatelSymbolWithText {}
                SearchBox { lang, onsearch }
            }

            div { class: "flex flex-row gap-10 items-center",
                Menu {
                    icon: rsx! {
                        Home1 { class: "[&>path]:stroke-[#737373]", width: "24", height: "24" }
                    },
                    link: Route::IndexPage {},
                    text: tr.home,
                }
                Menu {
                    icon: rsx! {
                        Internet {
                            class: "[&>path]:stroke-[#737373] [&>path]:fill-[#737373] [&>circle]:stroke-[#737373]",
                            width: "24",
                            height: "24",
                        }
                    },
                    link: Route::ExplorePage {},
                    text: tr.explore,
                }
                Menu {
                    icon: rsx! {
                        UserGroup { class: "[&>path]:stroke-[#737373]", width: "24", height: "24" }
                    },
                    link: Route::MyNetworkPage {},
                    text: tr.my_network,
                }
                Menu {
                    icon: rsx! {
                        RoundBubble {
                            class: "[&>path]:stroke-[#737373] [&>line]:stroke-[#737373]",
                            width: "24",
                            height: "24",
                            fill: "none",
                        }
                    },
                    link: Route::MessagesPage {},
                    text: tr.message,
                }
                Menu {
                    icon: rsx! {
                        Bell {
                            class: "[&>path]:stroke-[#737373] [&>path]:fill-[#737373]",
                            width: "24",
                            height: "24",
                        }
                    },
                    link: Route::NotificationsPage {},
                    text: tr.notification,
                }

                if is_login {
                    Profile { url: profile_url, name: nickname }
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

#[component]
pub fn Menu(text: String, icon: Element, link: Route) -> Element {
    rsx! {
        Link {
            class: "flex flex-col w-fit justify-center items-center p-10",
            to: link,
            {icon}
            div { class: "font-medium text-neutral-500 text-[15px]/18", {text} }
        }
    }
}

translate! {
    SocialHeaderTranslate;

    home: {
        ko: "Home",
        en: "Home"
    },
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
