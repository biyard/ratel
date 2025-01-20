#![allow(non_snake_case)]
use super::write_topic::WriteTopic;

use super::controller::*;
use super::legislation_selector::*;
use by_macros::EnumProp;
use by_types::QueryParam;
use dioxus::prelude::*;
use dioxus_translate::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, EnumProp)]
pub enum NewTopicStep {
    #[default]
    SelectLegislation,
    WriteTopic,
}

#[component]
pub fn NewTopicPage(
    lang: Language,
    step: NewTopicStep,
    legislation_id: QueryParam<String>,
) -> Element {
    let ctrl = Controller::new()?;

    let step = match step {
        NewTopicStep::WriteTopic => rsx! {
            WriteTopic {
                lang,
                onclick: move |req| async move { ctrl.handle_create_topic(req).await },
            }
        },
        NewTopicStep::SelectLegislation => rsx! {
            LegislationSelector {
                lang,
                onclick: move |id| ctrl.navigate_to_write_topic(&lang, id),
            }
        },
    };

    rsx! {
        div { id: "creation", {step} }
    }
}
