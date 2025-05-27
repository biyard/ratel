#![allow(unused)]

use bdk::prelude::{by_components::icons::edit::Edit1, *};

use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, window};

use crate::pages::my_network::{
    components::{FollowerContent, FollowingContent},

    controller::FollowerPerson,
};

#[derive(Clone, PartialEq, Translate)]
pub enum Tab {
    #[translate(ko = "Following", en = "Following")]
    Following,
    #[translate(ko = "Followers", en = "Followers")]
    Followers,
}

#[component]
pub fn FollowContents(
    lang: Language,

    my_followers: Vec<FollowerPerson>,

    is_write: bool,
    onwrite: EventHandler<MouseEvent>,
    onclick: EventHandler<i64>,
) -> Element {
    let mut selected_tab = use_signal(|| Tab::Following);

    rsx! {
        div { class: "flex flex-col w-full h-full justify-start items-start text-white",
            FollowTab {
                lang,
                selected_tab: selected_tab(),
                onchange: move |tab| {
                    selected_tab.set(tab);
                },
            }

            div { class: "flex flex-col w-full h-[calc(100vh-250px)] max-tablet:!h-full overflow-y-scroll",

                if selected_tab() == Tab::Following {
                    MyFollowingList { lang, my_followers, onclick }
                } else {
                    MyFollowersList { lang, my_followers, onclick }
                }
            }
        
        
        }
    }

}

#[component]
pub fn MyFollowingList(
    lang: Language,
    mut my_followers: Vec<FollowerPerson>,
    onclick: EventHandler<i64>,
) -> Element {

    let mut visible_count = use_signal(|| 10);
    let mut listener = use_signal(|| None as Option<EventListener>);

    let container_id = "following-scroll-container";

    use_effect({
        move || {
            if let Some(container) = window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(container_id))
                .and_then(|el| el.dyn_into::<HtmlElement>().ok())
            {
                let new_listener = EventListener::new(&container, "scroll", {
                    let container = container.clone();
                    move |_event| {
                        let scroll_top = container.scroll_top();
                        let scroll_height = container.scroll_height();
                        let client_height = container.client_height();

                        if scroll_top + client_height as i32 >= scroll_height as i32 - 5 {
                            visible_count.set(visible_count() + 5);
                            tracing::debug!("visible count: {}", visible_count());
                        }
                    }
                });

                listener.set(Some(new_listener));
            }
        }
    });

    

    rsx! {
        div {
            id: container_id,
            class: "flex flex-col w-full overflow-y-scroll p-[30px] bg-[#191919] gap-[.5rem] rounded-[12px]",

            span {
                class: "text-[24px]",
                "{my_followers.len()} following"
            }

            for person in my_followers.clone(){
                FollowingContent { lang, person, onclick }

            }
            
        }
    }
}

#[component]
pub fn MyFollowersList(
    lang: Language,
    mut my_followers: Vec<FollowerPerson>,
    onclick: EventHandler<i64>,
) -> Element {

    let mut visible_count = use_signal(|| 10);
    let mut listener = use_signal(|| None as Option<EventListener>);

    let container_id = "followers-scroll-container";

    use_effect({
        move || {
            if let Some(container) = window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id(container_id))
                .and_then(|el| el.dyn_into::<HtmlElement>().ok())
            {
                let new_listener = EventListener::new(&container, "scroll", {
                    let container = container.clone();
                    move |_event| {
                        let scroll_top = container.scroll_top();
                        let scroll_height = container.scroll_height();
                        let client_height = container.client_height();

                        if scroll_top + client_height as i32 >= scroll_height as i32 - 5 {
                            visible_count.set(visible_count() + 5);
                            tracing::debug!("visible count: {}", visible_count());
                        }
                    }
                });

                listener.set(Some(new_listener));
            }
        }
    });


    rsx! {
        div {
            id: container_id,
            class: "flex flex-col w-full overflow-y-scroll p-[30px] bg-[#191919] gap-[.5rem] rounded-[12px]",

            span {
                class: "text-[24px]",
                "{my_followers.len()} followers"
            }

            for person in my_followers.clone() {
                FollowerContent { lang, person, onclick }
            }


        }
    }
}



#[component]
pub fn FollowTab(lang: Language, selected_tab: Tab, onchange: EventHandler<Tab>) -> Element {
    let tabs = [Tab::Following, Tab::Followers];

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
                        class: "font-bold text-sm/20 aria-selected:text-white text-neutral-400 h-25",
                        "aria-selected": selected_tab == tab,
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