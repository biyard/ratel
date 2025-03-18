#![allow(non_snake_case)]
use bdk::prelude::{by_components::responsive::ResponsiveService, *};
use by_components::meta::MetaPage;

use super::components::*;

#[component]
pub fn HomePage(lang: Language, class: String) -> Element {
    let tr: TopTranslate = translate(&lang);
    let image = asset!("/public/logos/logo.png");
    let responsive_service: ResponsiveService = use_context();

    rsx! {
        MetaPage { title: "Ratel", description: tr.description, image: "{image}" }
        div { class: "hidden md:!block",
            Top { lang }
            div { class: "flex flex-col w-full justify-start items-center",
                About { lang }
                PoliticianStance { lang }
                Community { lang }
                Support { lang }
            }
        }
        div { class: "block md:!hidden",
            MobileTop { lang, class }
        }
    }
}
