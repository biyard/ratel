use bdk::prelude::*;

use crate::{
    pages::components::{BottomNavigationBar, SocialHeader},
    route::Route,
};
use dioxus_popup::PopupZone;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum RouteTab {
    Notification,
    Explore,
    Home,
    MyNetwork,
    Message,
}

#[component]
pub fn SocialLayout(#[props(default = Language::En)] lang: Language) -> Element {
    let nav = use_navigator();
    let mut selected = use_signal(|| RouteTab::Home);
    let mut prev_selected = use_signal(|| RouteTab::Home);

    rsx! {
        div { class: "flex flex-col h-screen w-full justify-between items-center overflow-hidden",
            div { class: "flex-shrink-0 w-full",
                SocialHeader {
                    lang,
                    onsearch: |_| {},
                    current_page: selected(),
                    onroute: move |_| {
                        let value = selected();
                        if value == RouteTab::Notification {
                            selected.set(prev_selected());
                        } else {
                            prev_selected.set(value);
                            selected.set(RouteTab::Notification);
                        }
                        match selected() {
                            RouteTab::Notification => {
                                nav.replace(Route::NotificationsPage {});
                            }
                            RouteTab::Explore => {
                                nav.replace(Route::ExplorePage {});
                            }
                            RouteTab::Home => {
                                nav.replace(Route::IndexPage {});
                            }
                            RouteTab::MyNetwork => {
                                nav.replace(Route::MyNetworkPage {});
                            }
                            RouteTab::Message => {
                                nav.replace(Route::MessagesPage {});
                            }
                        }
                    },
                }
            }

            div { class: "w-full max-w-desktop flex-1 overflow-y-auto", Outlet::<Route> {} }


            div {
                class: "flex-shrink-0 w-full aria-hidden:!hidden",
                aria_hidden: selected() == RouteTab::Notification,
                BottomNavigationBar {
                    lang,
                    current_page: selected(),
                    onroute: move |route: RouteTab| {
                        selected.set(route);
                    },
                }
            }
        }
        PopupZone {}
    }
}
