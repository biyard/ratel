#![allow(non_snake_case)]
use super::i18n::*;
use crate::route::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn Menus(
    #[props(default ="menus".to_string())] id: String,
    #[props(default ="".to_string())] class: String,
    lang: Language,
) -> Element {
    let tr: MenusTranslate = translate(&lang);

    rsx! {
        div { id, class,
            div { class: "flex flex-row rounded-full bg-[#323342]",
                MenuItem { to: Route::HomePage { lang }, "{tr.home}" }
                MenuItem { to: Route::TopicsPage { lang }, "{tr.topics}" }
                MenuItem {
                    to: Route::PoliticianStatusPage {
                        lang,
                    },
                    "{tr.politicians}"
                }
                MenuItem { to: Route::PatronsPage { lang }, "{tr.patrons}" }
            }
        }
    }
}

#[component]
pub fn MenuItem(#[props(into)] to: NavigationTarget, children: Element) -> Element {
    rsx! {
        Link {
            class: "px-[10px] py-[5px] text-[15px] font-light leading-[22.5px] text-[#adbcd7] hover:text-white hover:bg-[#424563] rounded-full",
            to,
            {children}
        }
    }
}
