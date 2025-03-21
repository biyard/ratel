#![allow(non_snake_case)]
use bdk::prelude::*;
use by_components::meta::MetaPage;
use subscription::Subscription;

use super::components::*;

#[component]
pub fn HomePage(lang: Language) -> Element {
    let tr: TopTranslate = translate(&lang);
    let image = asset!("/public/logos/logo.png");

    rsx! {
        MetaPage { title: "Ratel", description: tr.description, image: "{image}" }
        div { class: "hidden md:!block",
            div { class: "flex flex-col w-full justify-start items-center",
                Top { lang }
                About { lang }
                PoliticianStance { lang }
                Community { lang }
                Support { lang }
                Subscription { lang }
                Footer { lang }
            }
        }
        div { class: "block md:!hidden",
            div { class: "flex flex-col w-full justify-start items-center gap-[58px]",
                Top { lang }
                About { lang }
                PoliticianStance { lang }
                Community { lang }
                Footer { lang }
            }
        }
    }
}

// #[component]
// pub fn HomePage(lang: Language) -> Element {
//     let tr: TopTranslate = translate(&lang);
//     let image = asset!("/public/logos/logo.png");

//     rsx! {
//         MetaPage { title: "Ratel", description: tr.description, image: "{image}" }
//         div { class: "flex flex-col w-full justify-start items-center",
//             Top { lang }
//             About { lang }
//             PoliticianStance { lang }
//             Community { lang }
//             Support { lang }
//             Subscription { lang }
//             Footer { lang }
//         }
//     }
// }
