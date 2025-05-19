use crate::pages::components::LeftSidebar;

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn IndexPage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut _ctrl = Controller::new(lang)?;
    let tr: IndexTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { class: "flex flex-row w-full justify-start items-start py-20 gap-20",
            LeftSidebar { lang }
            div { class: "flex flex-col w-full justify-start items-start text-white",
                "feed section"
            }
        }
    }
}
