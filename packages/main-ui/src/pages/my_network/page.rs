// use super::*;
// use bdk::prelude::*;
// use controller::*;
// use i18n::*;

// #[component]
// pub fn MyNetworkPage(#[props(default = Language::En)] lang: Language) -> Element {
//     let mut _ctrl = Controller::new(lang)?;
//     let tr: MyNetworkTranslate = translate(&lang);

//     rsx! {
//         by_components::meta::MetaPage { title: tr.title }

//         div { id: "my-network", "{tr.title} PAGE" } // end of this page
//     }
// }




use dioxus::prelude::*;

use super:: components::{left_sidebar::LeftSidebar, main_content::MainContent, right_sidebar::RightSidebar};


#[component]
pub fn MyNetworkPage() -> Element {
    rsx!(
        div { class: "bg-neutral-900 min-h-screen text-white font-sans text-sm",

            div { class: "flex justify-center border-t border-neutral-700 py-4 space-x-10 text-neutral-400",
                div { class: "cursor-pointer text-white border-b-2 border-yellow-400 pb-2", "Following" }
                div { class: "cursor-pointer hover:text-white", "Followers" }
            }

            div { class: "flex justify-center space-x-2 px-4 py-4",
                {
                    ["ALL", "POLITICIAN", "POLITIC", "CRYPTO", "CATEGORY", "CATEGORY"]
                        .iter()
                        .map(|label| rsx!(
                            button {
                                class: "bg-neutral-800 text-white px-4 py-4 rounded-md hover:border-yellow-400 border cursor-pointer m-4",
                                "{label}"
                            }
                        ))
                }
                div { class: "text-white", ">" }
            }

            div { class: "flex px-6",
                LeftSidebar {}
                MainContent {}
                RightSidebar {}
            }
        }
    )
}






