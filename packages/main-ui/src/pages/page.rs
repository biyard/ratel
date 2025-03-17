#![allow(non_snake_case)]
use bdk::prelude::{
    by_components::responsive::{Responsive, ResponsiveService},
    *,
};
use by_components::meta::MetaPage;

use super::components::*;

#[component]
pub fn HomePage(lang: Language, class: String) -> Element {
    let tr: TopTranslate = translate(&lang);
    let image = asset!("/public/logos/logo.png");
    let responsive_service: ResponsiveService = use_context();

    rsx! {
        Responsive {
            if responsive_service.width() > 1200.0 {
                //Desktop page
                div { class: "desktop-layout",
                    MetaPage {
                        title: "Ratel",
                        description: tr.description,
                        image: "{image}",
                    }
                    div { class: "flex flex-col w-full justify-start items-center",
                        Top { lang }
                        About { lang }
                        PoliticianStance { lang }
                        Community { lang }
                        Support { lang }
                    }
                }
            } else {
                //mobile page
                div { class: "mobile-layout",
                    MetaPage {
                        title: "Ratel",
                        description: tr.description,
                        image: "{image}",
                    }
                    div { class: "flex flex-col w-full justify-start items-center",
                        MobileTop { lang, class }
                    }
                }
            }
        }
    }
}
