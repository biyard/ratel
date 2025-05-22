use bdk::prelude::{
    by_components::icons::{
        chat::RoundBubble, home::Home1, internet_script::Internet, user::UserGroup,
    },
    *,
};

use crate::{pages::RouteTab, route::Route};

#[component]
pub fn BottomNavigationBar(
    lang: Language,
    current_page: RouteTab,
    onroute: EventHandler<RouteTab>,
) -> Element {
    let tr: BottomSheetTranslate = translate(&lang);

    rsx! {
        div { class: "hidden max-tablet:flex max-tablet:flex-col max-tablet:justify-start max-tablet:items-start max-tablet:bg-neutral-800 max-tablet:h-fit",
            div { class: "flex flex-row w-full h-1 gap-16 mx-16 px-16 bg-neutral-700" }
            div { class: "flex flex-row w-full justify-between items-center px-16 pt-12 pb-22",
                BottomIcon {
                    icon: rsx! {
                        Internet {
                            class: format!(
                                "{}",
                                if current_page == RouteTab::Explore {
                                    "[&>path]:stroke-white [&>path]:fill-white [&>circle]:stroke-white"
                                } else {
                                    "[&>path]:stroke-neutral-500 [&>path]:fill-neutral-500 [&>circle]:stroke-neutral-500"
                                },
                            ),
                            width: "32",
                            height: "32",
                        }
                    },
                    link: Route::ExplorePage {},
                    text: tr.explore,
                    selected: current_page == RouteTab::Explore,
                    onselected: move |_| {
                        onroute.call(RouteTab::Explore);
                    },
                }
                BottomIcon {
                    icon: rsx! {
                        Home1 {
                            class: format!(
                                "{}",
                                if current_page == RouteTab::Home {
                                    "[&>path]:stroke-white"
                                } else {
                                    "[&>path]:stroke-neutral-500"
                                },
                            ),
                            width: "32",
                            height: "32",
                        }
                    },
                    link: Route::IndexPage {},
                    text: tr.home,
                    selected: current_page == RouteTab::Home,
                    onselected: move |_| {
                        onroute.call(RouteTab::Home);
                    },
                }
                BottomIcon {
                    icon: rsx! {
                        UserGroup {
                            class: format!(
                                "{}",
                                if current_page == RouteTab::MyNetwork {
                                    "[&>path]:stroke-white"
                                } else {
                                    "[&>path]:stroke-neutral-500"
                                },
                            ),
                            width: "32",
                            height: "32",
                        }
                    },
                    link: Route::MyNetworkPage {},
                    text: tr.my_network,
                    selected: current_page == RouteTab::MyNetwork,
                    onselected: move |_| {
                        onroute.call(RouteTab::MyNetwork);
                    },
                }
                BottomIcon {
                    icon: rsx! {
                        RoundBubble {
                            class: format!(
                                "{}",
                                if current_page == RouteTab::Message {
                                    "[&>path]:stroke-white [&>line]:stroke-white"
                                } else {
                                    "[&>path]:stroke-neutral-500 [&>line]:stroke-neutral-500"
                                },
                            ),
                            width: "32",
                            height: "32",
                            fill: "none",
                        }
                    },
                    link: Route::MessagesPage {},
                    text: tr.message,
                    selected: current_page == RouteTab::Message,
                    onselected: move |_| {
                        onroute.call(RouteTab::Message);
                    },
                }
            }
        }
    }
}

#[component]
pub fn BottomIcon(
    icon: Element,
    text: String,
    link: Route,
    selected: bool,
    onselected: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        Link {
            class: "flex flex-col w-fit justify-center items-center gap-4",
            onclick: move |e| {
                onselected.call(e);
            },
            to: link,
            {icon}
            div {
                class: "font-medium text-sm/14 text-neutral-500 aria-selected:text-white",
                aria_selected: selected,
                {text}
            }
        }
    }
}

translate! {
    BottomSheetTranslate;

    explore: {
        ko: "Explore",
        en: "Explore"
    },
    home: {
        ko: "Home",
        en: "Home"
    },
    my_network: {
        ko: "My network",
        en: "My network"
    },
    message: {
        ko: "Message",
        en: "Message"
    }
}
