#![allow(non_snake_case)]
use crate::pages::landing::components::FooterWithSocial;

use super::*;
use bdk::prelude::*;
use components::*;
use controller::*;
use i18n::*;

#[component]
pub fn PoliticiansByIdPage(
    id: ReadOnlySignal<i64>,
    #[props(default = Language::En)] lang: Language,
) -> Element {
    let ctrl = Controller::new(lang, id)?;
    let tr: PoliticiansByIdTranslate = translate(&lang);
    let p = ctrl.politician()?;

    rsx! {
        by_components::meta::MetaPage {
            title: p.name(&lang),
            description: tr.description,
            image: p.image_url.clone(),
        }

        div { class: "w-full grow flex flex-col items-center",
            div {
                id: "politicians-by-id",
                class: "w-full grow max-w-1177 mt-160 flex flex-col justify-start px-10",
                PoliticianHeader {
                    lang,
                    image: p.image_url.clone(),
                    name: p.name(&lang),
                    party: p.party_enum(),
                    stance: p.stance,
                    email: p.email.clone().unwrap_or_default(),
                    description: tr.description,
                }
                PoliticianActivities { lang, name: p.name(&lang), bills: p.bills }
            } // end of this page
            FooterWithSocial { lang }
        }
    }
}
