use crate::components::icons::{ThumbsUp, Tokenpost};

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

        div { class: "absolute bottom-0 left-0 w-screen py-10 items-center justify-center text-c-wg-50 text-xs flex flex-row gap-5 cursor-pointer z-100",
            span { "Jointly powered by Ratel and" }
            a { href: "https://tokenpost.com", target: "_blank",
                Tokenpost { height: 25 }
            }
        }

        div {
            id: "quizzes",
            class: "absolute transition-all duration-500 left-0 top-0 h-screen w-screen flex flex-col items-center py-50 px-20 justify-between aria-started:left-[-100%]",
            "aria-started": ctrl.started(),
            h1 { class: "heading1 whitespace-pre-line text-center py-30", {tr.title} }
            div { class: "w-full flex flex-col items-center gap-10",
                dotlottie-player {
                    src: asset!("/public/animations/ani_logo.json"),
                    class: "w-193 h-200",
                    "autoplay": true,
                }
                p {
                    class: "w-full flex flex-col items-center !hidden aria-show:!flex",
                    "aria-show": !ctrl.already_done(),
                    p { class: "text-center",
                        "각 주제에 대해서 "
                        span { class: "text-primary font-bold",
                            "상대적으로 필요성이 높다고 생각되는 것"
                        }
                        "에 "
                        span { class: "text-primary font-bold", "좋아요" }
                        "를 클릭해주세요."

                    }
                }
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
                    class: "absolute transition-all duration-500 left-[{ctrl.left(i)}%] top-0 h-screen w-screen flex flex-col items-center py-50 px-20 justify-center gap-100",

                    div { class: "flex flex-col items-center justify-center gap-10",
                        h2 { class: "text-xl !text-primary font-black", "Topic {i+1}." }
                        QuizItem {
                            class: "text-xl font-semibold",
                            quiz: quiz.clone(),
                        }
                    }
                    div { class: "w-full flex flex-col gap-10",
                        button {
                            class: "group btn w-full flex !flex-col py-30 px-20 w-full flex flex-row gap-20 items-center bg-component-bg/50 hover:bg-component-bg/100",
                            onclick: move |_| async move {
                                ctrl.like(i).await;
                            },
                            p { class: "grow text-center text-lg", {quiz.like_option} }
                            div { class: "h-1 w-full bg-white/10" }
                            div { class: "flex flex-row gap-10 items-center",
                                span { {tr.btn_like} }
                                ThumbsUp {
                                    class: "group-hover:[&>path]:fill-primary group-hover:[&>path]:stroke-primary",
                                    size: 20,
                                }
                            }
                        }

                        button {
                            class: "group btn w-full flex !flex-col py-30 px-20 w-full flex flex-row gap-20 items-center bg-component-bg/50 hover:bg-component-bg/100",
                            onclick: move |_| async move {
                                ctrl.dislike(i).await;
                            },
                            p { class: "grow text-center text-lg", {quiz.dislike_option} }
                            div { class: "h-1 w-full bg-white/10" }
                            div { class: "flex flex-row gap-10 items-center",
                                span { {tr.btn_like} }
                                ThumbsUp {
                                    class: "group-hover:[&>path]:fill-primary group-hover:[&>path]:stroke-primary",
                                    size: 20,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
