#![allow(unused)]
use crate::pages::components::{LeftSidebar, RightSidebar};

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

        //FIXME: fix to connect api
        div { class: "flex flex-row w-full justify-start items-start py-20 gap-20",
            LeftSidebar { lang }
            div { class: "flex flex-col w-full justify-start items-start text-white",
                "feed section"
            }
            RightSidebar {
                lang,
                promotions: vec![
                    PromotionModel {
                        profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c"
                            .to_string(),
                        title: "Crypto promotion".to_string(),
                        description: "A new research on engagement of crypto currency".to_string(),
                    },
                ],
                news: vec![
                    NewsModel {
                        title: "Legislative social".to_string(),
                        description: "Ratel has launched legislative social to engage industry-friendly legislative blah blah ..."
                            .to_string(),
                    },
                    NewsModel {
                        title: "Presidential candidates ".to_string(),
                        description: "Ratel has started to like election pledges on crypto industry of each presidential blah blah ..."
                            .to_string(),
                    },
                    NewsModel {
                        title: "New Promotions ".to_string(),
                        description: "Ratel engages and supports marketing for industrial promotions which helps blah blah ..."
                            .to_string(),
                    },
                ],
                feeds: vec![
                    FeedModel {
                        image: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c"
                            .to_string(),
                        title: "Donald Trump".to_string(),
                        description: "President of the US".to_string(),
                    },
                    FeedModel {
                        image: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c"
                            .to_string(),
                        title: "Elon Musk".to_string(),
                        description: "CEO of Tesla and SpaceX".to_string(),
                    },
                    FeedModel {
                        image: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c"
                            .to_string(),
                        title: "Jongseok Park".to_string(),
                        description: "National Assembly of blah blah".to_string(),
                    },
                ],
            }
        }
    }
}
