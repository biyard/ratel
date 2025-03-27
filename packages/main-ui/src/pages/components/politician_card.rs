#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::{components::icons::*, route::Route};

#[component]
pub fn PoliticianCard(
    lang: Language,
    id: i64,
    name: String,
    party: String,
    image_url: String,
) -> Element {
    rsx! {
        Link {
            to: Route::PoliticiansByIdPage {
                lang,
                id,
            },
            class: "relative col-span-1 w-full h-full rounded-[10px] overflow-hidden",

            background_image: format!("url({})", image_url),
            background_size: "cover",
            background_position: "center",
            background_repeat: "no-repeat",

            div {
                class: "absolute bottom-0 left-0 w-full h-100",
                background: "linear-gradient(180deg, rgba(0, 2, 3, 0) 0%, rgba(0, 2, 3, 0.5) 40%, rgba(0, 2, 3, 0.8) 100%, rgba(0, 2, 3, 0.9) 100%)",
            }

            div { class: "absolute bottom-0 left-0 w-full flex flex-col justify-start gap-4 px-10 py-15",
                p { class: "text-white text-lg/22 font-bold", "{name}" }
                div { class: "flex flex-row items-center gap-8",
                    PPP { size: "18" }
                    p { class: "text-white text-[15px]/22 font-medium", "{party}" }
                }
            }
        }
    }
}
