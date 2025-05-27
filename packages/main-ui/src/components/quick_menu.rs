use bdk::prelude::*;

use crate::{
    components::icons::{PresidentialElectionIcon, quiz::QuizIcon, sign::SignIcon},
    route::Route,
};

#[component]
pub fn QuickMenu(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut show = use_signal(|| false);
    rsx! {
        button {
            id: "quick-menu",
            class: "translation-all duration-500 fixed top-1/2 translate-y-[-50%] right-[-50px] bg-white z-999 w-70 h-200 rounded-l-sm text-black font-semibold flex flex-row items-center justify-start p-2 gap-3 aria-show:right-0 cursor-pointer",
            "aria-show": show(),
            onclick: move |_e| {
                show.set(!show());
            },
            div { class: "group flex flex-row items-center gap-3 pr-5 border-r h-full border-c-wg-30",
                div { class: "transition-all mt-3 w-1 rounded-full h-50 bg-c-wg-50 group-hover:w-2 group-hover:h-55" }
                div { class: "peer whitespace-nowrap text-[8px]/8 flex flex-col items-center ",
                    span { "Q" }
                    span { "U" }
                    span { "I" }
                    span { "C" }
                    span { "K" }
                    span { class: "h-8" }
                    span { "M" }
                    span { "E" }
                    span { "N" }
                    span { "U" }
                }
            }

            div { class: "flex flex-col gap-2 w-full items-center",
                Link {
                    class: "w-full h-50 flex items-center justify-center text-c-wg-50 !text-[7px] whitespace-pre flex-col group hover:text-primary",
                    to: Route::PresidentialElectionPageForLanding {
                    },
                    PresidentialElectionIcon {
                        class: "group-hover:[&>circle]:stroke-primary group-hover:[&>path]:stroke-primary group-hover:[&>rect]:stroke-primary",
                        width: "30",
                        height: "30",
                    }
                    "한국대선"
                }

                Link {
                    class: "w-full h-50 flex items-center justify-center text-c-wg-50 !text-[7px] whitespace-pre flex-col group hover:text-primary",
                    to: Route::AdvocacyCampaignsByIdPage {
                        id: 1,
                    },
                    SignIcon { class: "group-hover:[&>g]:stroke-primary", size: 25 }
                    "발의지지"
                }

                Link {
                    class: "w-full h-50 flex items-center justify-center text-c-wg-50 !text-[7px] whitespace-pre flex-col group hover:text-primary",
                    to: Route::QuizzesPage {},
                    QuizIcon { class: "group-hover:[&>path]:fill-primary", size: 30 }

                    "대선공약퀴즈"
                }
            }
        }
    }
}
