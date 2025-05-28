use super::*;
use crate::{
    pages::{
        SpaceEditButton, SpaceMoreButton,
        threads::_id::spaces::_id::pages::nft::controller::Controller,
    },
    route::Route,
};
use bdk::prelude::*;
use dto::by_components::{
    icons::{file::File, shopping::Cube},
    loaders::cube_loader::CubeLoader,
};

#[component]
pub fn NftSettingLayout(
    #[props(default = Language::En)] lang: Language,
    feed_id: ReadOnlySignal<i64>,
    id: ReadOnlySignal<i64>,
) -> Element {
    let mut ctrl = Controller::new(lang, feed_id(), id())?;
    rsx! {
        div { id: "nft-setting-layout", class: "flex flex-row w-full gap-20",

            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "w-full h-full flex flex-col justify-center items-center", CubeLoader {} }
                },
                div { class: "w-full h-full", Outlet::<Route> {} }
            }

            div { class: "w-250 max-tablet:!hidden flex flex-col gap-10",
                div { class: "flex flex-row w-full justify-between items-center gap-8",
                    div { class: "cursor-pointer w-fit h-fit",
                        SpaceEditButton {
                            isedit: ctrl.is_edit(),
                            onedit: move |e| {
                                ctrl.change_edit(e);
                            },
                        }
                    }
                    SpaceMoreButton {}
                }
                div { class: "flex-col py-20 px-12 rounded-[10px] bg-[#191919]",
                    for n in NftSettingStep::VARIANTS {
                        div {
                            class: "cursor-pointer flex flex-row w-full justify-start items-center px-4 py-8 rounded-[4px] gap-4 aria-selected:!bg-neutral-800",
                            aria_selected: ctrl.current_step() == *n,
                            onclick: move |_| {
                                ctrl.change_current_step(*n);
                            },
                            {
                                match n {
                                    NftSettingStep::Summary => rsx! {
                                        File { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                                    },
                                    NftSettingStep::Nft => rsx! {
                                        Cube {
                                            class: "[&>path]:stroke-neutral-500",
                                            width: "20",
                                            height: "20",
                                            fill: "none",
                                        }
                                    },
                                }
                            }

                            {n.translate(&lang)}
                        }
                    }
                }
            }
        }
    }
}
