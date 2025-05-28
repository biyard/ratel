#![allow(unused)]
use super::*;
use crate::{
    pages::threads::_id::spaces::_id::pages::deliberation::controller::Controller, route::Route,
};
use bdk::prelude::*;
use by_components::loaders::cube_loader::CubeLoader;
use dto::by_components::icons::{
    arrows::ShapeArrowDown, chat::Discuss, edit::Edit1, email::Vote, file::File,
    validations::CheckCircle,
};

#[component]
pub fn EditButton(isedit: bool, onedit: EventHandler<bool>) -> Element {
    rsx! {
        div {
            class: "cursor-pointer flex flex-row w-194 h-46 justify-start items-center px-16 py-12 bg-white rounded-l-[100px] rounded-r-[4px] gap-4",
            onclick: move |_| {
                onedit.call(!isedit);
            },
            Edit1 {
                class: "[&>path]:stroke-neutral-500 w-16 h-16",
                width: "16",
                height: "16",
                fill: "none",
            }

            div { class: "font-bold text-neutral-900 text-base/22",
                {if isedit { "Save" } else { "Edit" }}
            }
        }
    }
}

#[component]
pub fn MoreButton() -> Element {
    rsx! {
        div { class: "flex flex-row w-48 h-46 justify-center items-center bg-neutral-500 rounded-l-[4px] rounded-r-[100px]",
            ShapeArrowDown { size: 16 }
        }
    }
}

#[component]
pub fn DeliberationSettingLayout(children: Element) -> Element {
    let route = use_route::<Route>();
    let (lang, feed_id, id) = match route {
        Route::Summary { feed_id, id }
        | Route::Deliberation { feed_id, id }
        | Route::FinalConsensus { feed_id, id }
        | Route::Poll { feed_id, id } => (Language::En, feed_id, id),
        _ => (Language::En, 0, 0),
    };

    let mut ctrl = Controller::new(lang, feed_id, id)?;

    rsx! {
        div {
            id: "deliberation-setting-layout",
            class: "flex flex-row w-full gap-20",

            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "w-full h-full flex flex-col justify-center items-center", CubeLoader {} }
                },
                div { class: "w-full h-full", Outlet::<Route> {} }
            }

            div { class: "w-250 max-tablet:!hidden flex flex-col gap-10",
                div { class: "flex flex-row w-full justify-between items-center gap-8",
                    div { class: "cursor-pointer w-fit h-fit",
                        EditButton {
                            isedit: ctrl.is_edit(),
                            onedit: move |e| {
                                ctrl.change_edit(e);
                            },
                        }
                    }
                    MoreButton {}
                }
                div { class: "flex-col py-20 px-12 rounded-[10px] bg-[#191919]",
                    for n in DeliberationSettingStep::VARIANTS {
                        div {
                            class: "cursor-pointer flex flex-row w-full justify-start items-center px-4 py-8 rounded-[4px] gap-4 aria-selected:!bg-neutral-800",
                            aria_selected: ctrl.current_step() == *n,
                            onclick: move |_| {
                                ctrl.change_current_step(*n);
                            },
                            {
                                match n {
                                    DeliberationSettingStep::Summary => rsx! {
                                        File { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                                    },
                                    DeliberationSettingStep::Deliberation => rsx! {
                                        Discuss {
                                            class: "[&>path]:stroke-neutral-500",
                                            width: "20",
                                            height: "20",
                                            fill: "none",
                                        }
                                    },
                                    DeliberationSettingStep::Poll => rsx! {
                                        Vote {
                                            class: "[&>path]:stroke-neutral-500 [&>rect]:stroke-neutral-500",
                                            width: "20",
                                            height: "20",
                                        }
                                    },
                                    DeliberationSettingStep::FinalConsensus => rsx! {
                                        CheckCircle {
                                            class: "[&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500",
                                            width: "20",
                                            height: "20",
                                        }
                                    },
                                }
                            }

                            {n.translate(&lang)}
                        }
                    }
                }
            }

        } // end of this page
    }
}
