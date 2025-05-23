use crate::components::icons::{ThumbsDown, ThumbsUp};

use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn QuizzesPage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: QuizzesTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div {
            id: "quizzes",
            class: "absolute transition-all duration-500 left-0 top-0 h-screen w-screen flex flex-col items-center py-50 px-20 justify-between aria-started:left-[-100%]",
            "aria-started": ctrl.started(),
            h1 { class: "heading1 whitespace-pre-line text-center py-30", {tr.title} }
            dotlottie-player {
                src: asset!("/public/animations/ani_logo.json"),
                class: "w-193 h-250",
                "autoplay": true,
            }

            button {
                class: "btn primary w-full !hidden aria-show:!flex",
                onclick: move |_| ctrl.start(),
                "aria-show": !ctrl.already_done(),
                {tr.btn_start}
            }

            button {
                onclick: move |_| ctrl.go_to_result(),
                class: "btn w-full !hidden aria-show:!flex",
                "aria-show": ctrl.already_done(),
                {tr.btn_go_to_result}
            }
        } // end of this page

        div {
            class: "absolute relative left-0 top-0 py-70 px-20 w-screen h-screen hidden aria-started:!flex flex-col overflow-hidden",
            "aria-started": ctrl.started(),
            div { class: "w-full flex flex-row gap-10 items-center lg:py-10",
                div { class: "w-full rounded-full h-5 bg-component-bg",
                    div { class: "transition-all duration-500 w-[{ctrl.progress()}%] rounded-full h-full bg-primary" }
                }
                span { {format!("{}/{}", ctrl.current_step(), ctrl.quizzes()?.len())} }

            }

            for (i , quiz) in ctrl.quizzes()?.into_iter().enumerate() {
                div {
                    id: "quiz-{i}",
                    class: "absolute transition-all duration-500 left-[{ctrl.left(i)}%] top-0 h-screen w-screen flex flex-col items-center py-50 px-20 justify-between",

                    div { class: "grow-1 h-full flex flex-col items-center justify-center gap-10",
                        h2 { class: "heading1 !text-primary font-black", "Q{i+1}." }
                        QuizItem { quiz: quiz.clone() }
                    }
                    div { class: "w-full flex flex-row justify-around",
                        button {
                            class: "group btn !flex-col items-center",
                            onclick: move |_| async move {
                                ctrl.like(i).await;
                            },
                            ThumbsUp {
                                class: "group-hover:[&>path]:fill-primary group-hover:[&>path]:stroke-primary",
                                size: 50,
                            }
                            {tr.btn_like}
                        }
                        button {
                            class: "group btn !flex-col items-center",
                            onclick: move |_| async move {
                                ctrl.dislike(i).await;
                            },
                            ThumbsDown {
                                class: "group-hover:[&>path]:fill-primary group-hover:[&>path]:stroke-primary",
                                size: 50,
                            }
                            {tr.btn_dislike}
                        }
                    }
                }
            }
        }
    }
}
