#![allow(unused)]
use crate::{
    pages::{
        SpaceEditButton, SpaceMoreButton,
        threads::_id::spaces::_id::pages::legislation::controller::Controller,
    },
    route::Route,
};

use bdk::prelude::*;
use dto::by_components::loaders::cube_loader::CubeLoader;

#[component]
pub fn LegislationSettingLayout(children: Element) -> Element {
    let route = use_route::<Route>();
    let (lang, feed_id, id) = match route {
        Route::LegislationSummary { feed_id, id } => (Language::En, feed_id, id),
        _ => (Language::En, 0, 0),
    };

    let mut ctrl = Controller::new(lang, feed_id, id)?;

    rsx! {
        div {
            id: "legislation-setting-layout",
            class: "flex flex-row w-full gap-20",

            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "w-full h-full flex flex-col justify-center items-center", CubeLoader {} }
                },
                div { class: "w-full h-full", Outlet::<Route> {} }
            }

        // div { class: "w-250 max-tablet:!hidden flex flex-col gap-10",
        //     div { class: "flex flex-row w-full justify-between items-center gap-8",
        //         div { class: "cursor-pointer w-fit h-fit",
        //             SpaceEditButton {
        //                 isedit: ctrl.is_edit(),
        //                 onedit: move |e| {
        //                     ctrl.change_edit(e);
        //                 },
        //             }
        //         }
        //         SpaceMoreButton {}
        //     }
        // }
        }
    }
}
