#![allow(non_snake_case)]
use by_components::meta::MetaPage;
use dioxus::prelude::*;

use super::components::*;

use dioxus_translate::*;

#[component]
pub fn HomePage(lang: Language) -> Element {
    let tr: TopTranslate = translate(&lang);
    let image = asset!("/public/logos/logo.png");

    rsx! {
        MetaPage { title: "Ratel", description: tr.description, image: "{image}" }
        div { class: "flex flex-col w-full justify-start items-center",
            Top { lang }
            About { lang }
            PoliticianStance { lang }
            Community { lang }
            Support { lang }
        }
    }
}
