use super::*;
use controller::*;

use bdk::prelude::*;
use dto::ContentType;

use crate::{
    pages::components::{
        BottomNavigationBar, BottomSheet, CreateFeedBox, LeftSidebar, RightSidebar, SocialHeader,
    },
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

#[component]
pub fn MyPageLayout(#[props(default = Language::En)] lang: Language) -> Element {
    let mut is_write = use_signal(|| false);
    let mut ctrl = Controller::new(lang)?;

    let landing_data = ctrl.landing_data()?;
    let hot_promotions = ctrl.hot_promotions()?;
    let news = ctrl.news()?;
    let profile = ctrl.profile()?;
    let communities = ctrl.communities()?;
    let accounts = ctrl.accounts()?;

    let my_spaces = landing_data.my_spaces;

    let followers = landing_data.follower_list;
    let profile_data = landing_data.profile_data;

    let recent_feeds: Vec<String> = my_spaces
        .iter()
        .map(|v| v.title.clone().unwrap_or_default())
        .take(3)
        .collect();
    let recent_spaces: Vec<String> = my_spaces
        .iter()
        .map(|v| v.title.clone().unwrap_or_default())
        .take(3)
        .collect();
    let recent_communities: Vec<String> = communities
        .iter()
        .map(|v| v.title.clone().unwrap_or_default())
        .take(3)
        .collect();

    rsx! {
        div { class: "flex flex-col w-full h-screen justify-start items-start",
            div { class: "flex flex-row w-full h-[calc(100vh)] justify-start items-start py-20 px-10 gap-20",
                LeftSidebar {
                    lang,
                    profile: profile_data.clone(),
                    // recent_feeds: recent_feeds.clone(),
                    // recent_spaces: recent_spaces.clone(),
                    // recent_communities: recent_communities.clone(),
                    accounts: accounts.clone(),

                    onwrite: move |_| {
                        is_write.set(true);
                    },
                    add_account: move |_| async move {
                        ctrl.add_account().await;
                    },
                    sign_out: move |_| async move {
                        ctrl.signout().await;
                    },
                }

                div { class: "w-full", Outlet::<Route> {} }

                RightSidebar {
                    lang,
                    promotion: hot_promotions,
                    news,
                    followers,

                    follow: move |id: i64| async move {
                        ctrl.follow(id).await;
                    },
                }
            }

            div {
                class: "fixed bottom-85 left-0 w-full hidden max-tablet:flex aria-hidden:!hidden",
                aria_hidden: is_write(),
                BottomSheet {
                    lang,
                    profile: profile.clone(),
                    recent_feeds,
                    recent_spaces,
                    recent_communities,
                    accounts,

                    add_account: move |_| async move {
                        ctrl.add_account().await;
                    },
                    sign_out: move |_| async move {
                        ctrl.signout().await;
                    },
                }
            }

            div {
                class: "flex flex-row w-full justify-start items-start aria-hidden:!hidden",
                aria_hidden: !is_write(),
                CreateFeedBox {
                    lang,
                    nickname: profile.nickname.clone(),
                    profile: profile.profile.clone(),
                    onclose: move |_| {
                        is_write.set(false);
                    },
                    onsend: move |(content_type, description): (ContentType, String)| async move {
                        ctrl.create_feed(content_type, description).await;
                    },
                }
            }
        }
    }
}
