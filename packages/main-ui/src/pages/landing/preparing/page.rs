#![allow(non_snake_case)]
use crate::{
    components::{
        button::{ButtonSize, primary_button::PrimaryLink, secondary_botton::SecondaryA},
        icons::ComingSoon,
        socials::Socials,
    },
    pages::landing::preparing::i18n::PreparingTranslate,
    route::Route,
};
use bdk::prelude::*;
use by_components::icons::arrows::ArrowLeft;

#[component]
pub fn PreparingPage(
    #[props(default = Language::En)] lang: Language,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let tr: PreparingTranslate = translate(&lang);

    rsx! {
        div {
            id: "preparing",
            class: "w-screen h-screen flex flex-col items-center justify-center gap-100 max-[900px]:!px-30 max-[380px]:!mt-100",
            ..attributes,
            div { class: "flex flex-col items-center justify-center gap-32",
                div { class: "max-[900px]:!scale-70",
                    ComingSoon { height: Some(190) }
                }
                h1 { class: "text-5xl/56 text-center font-bold text-white whitespace-pre-line max-[900px]:text-[28px]",
                    {tr.title}
                }
                p { class: "text-lg text-center font-normal text-c-wg-30 whitespace-pre-line max-[900px]: text-[15px]",
                    {tr.description}
                }

                Socials { class: "flex flex-row gap-50" }
            }

            div { class: "flex flex-row gap-10 max-[900px]:!flex-col",
                PrimaryLink { size: ButtonSize::Normal, to: Route::LandingPage {},
                    ArrowLeft {
                        class: "[&>path]:stroke-3",
                        width: "20",
                        height: "20",
                    }
                    {tr.go_back}
                }

                SecondaryA { href: "/public/documents/Ratel-Token-White-Paper.pdf", {tr.learn_more} }
            }
        }
    }
}
