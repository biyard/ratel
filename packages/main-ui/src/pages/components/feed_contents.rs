use bdk::prelude::*;

#[derive(Clone, PartialEq, Translate)]
pub enum Tab {
    #[translate(ko = "For you", en = "For you")]
    Me,
    #[translate(ko = "Following", en = "Following")]
    Following,
}

#[component]
pub fn FeedContents(lang: Language) -> Element {
    let mut selected_tab = use_signal(|| Tab::Me);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start text-white",
            FeedTab {
                lang,
                selected_tab: selected_tab(),
                onchange: move |tab| {
                    selected_tab.set(tab);
                },
            }

            if selected_tab() == Tab::Me {
                MyFeedList { lang }
            } else {
                FollowingFeedList { lang }
            }
        }
    }
}

#[component]
pub fn MyFeedList(lang: Language) -> Element {
    rsx! {
        div { class: "text-white", "my feed list" }
    }
}

#[component]
pub fn FollowingFeedList(lang: Language) -> Element {
    rsx! {
        div { class: "text-white", "following feed list" }
    }
}

#[component]
pub fn FeedTab(lang: Language, selected_tab: Tab, onchange: EventHandler<Tab>) -> Element {
    let tabs = [Tab::Me, Tab::Following];

    rsx! {
        div { class: "flex flex-row w-full",
            for tab in tabs {
                div {
                    class: "flex flex-col flex-1 items-center cursor-pointer py-4",
                    onclick: {
                        let tab = tab.clone();
                        move |_| {
                            onchange.call(tab.clone());
                        }
                    },

                    div {
                        class: "font-bold text-sm/20 aria-active:text-white text-neutral-400 h-25",
                        "aria-active": selected_tab == tab,
                        {tab.translate(&lang)}
                    }
                    if selected_tab == tab {
                        div { class: "w-29 h-2 mt-1 rounded-full bg-yellow-400" }
                    } else {
                        div { class: "h-2 mt-1" }
                    }
                }
            }
        }
    }
}
