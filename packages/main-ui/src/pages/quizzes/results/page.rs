use crate::components::icons::{Facebook, X};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn ResultsPage(
    #[props(default = Language::En)] lang: Language,
    id: ReadOnlySignal<String>,
) -> Element {
    let mut ctrl = Controller::new(lang, id)?;
    let _tr: ResultsTranslate = translate(&lang);
    let (result, candidate) = ctrl.result()?;
    let (_, _name, percent) = &result
        .percentage_of_each_candidate()
        .get(0)
        .map(|v| v.clone())
        .unwrap_or((0, "".to_string(), 0.0));
    let description = format!(
        "{}<p>🎖️ 정책 성향 일치율: <b>{:.1}%</b></p>",
        candidate.description, percent
    );

    rsx! {
        by_components::meta::MetaPage { title: "{candidate.name}", image: "{candidate.image}" }

        div {
            id: "results",
            class: "flex flex-col max-w-500 w-full items-center h-screen py-70 justify-between text-center gap-30",
            div { class: "flex flex-col max-w-500 w-full items-center text-center gap-30",
                img {
                    src: candidate.image,
                    alt: candidate.name,
                    class: "w-full max-w-200",
                }
                p {
                    class: "text-xl/40 px-10 text-left flex flex-col gap-10",
                    dangerous_inner_html: "{description}",

                }
                        // div { class: "flex flex-col gap-10 w-full px-10",
            //     h2 { class: "heading2", "Your support for each candidate" }
            //     for (_ , name , percent) in result.percentage_of_each_candidate().iter() {
            //         div { class: "flex flex-col gap-5 w-full",
            //             p { class: "w-100 text-left", "{name}" }
            //             div { class: "w-[{percent}%] h-20",
            //                 div { class: "animate-grow bg-primary h-full flex flex-row items-center justify-end text-right rounded-sm py-auto",
            //                     span { class: "px-10", "{percent:.1}%" }
            //                 }
            //             }
            //         }
            //     }
            // }

            }

            div { class: "flex flex-row justify-around items-center w-full",
                button {
                    class: "btn primary !hidden aria-show:!flex",
                    "aria-show": ctrl.is_mine(),
                    onclick: move |_| ctrl.sign_up(),
                    "Sign up and Save"
                }
                div { class: "flex flex-row gap-20 items-center",
                    a {
                        href: "https://www.facebook.com/sharer/sharer.php?u={ctrl.location()}",
                        target: "_blank",
                        class: "btn",
                        Facebook { size: 40 }
                    }

                    a {
                        href: "https://x.com/intent/tweet?text=%5B%F0%9F%93%A2%20%EB%82%98%EC%9D%98%20%EA%B3%B5%EC%95%BD%20%EC%84%B1%ED%96%A5%20%EA%B3%B5%EC%9C%A0%ED%95%98%EA%B8%B0%5D&url={ctrl.location()}&hashtags={candidate.tags}",
                        target: "_blank",
                        class: "btn rounded-[2px] bg-black w-35 h-35",
                        X { size: 20 }
                    }
                }
            }
        }
    }
}
