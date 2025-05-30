use super::{controller::Controller, *};
use crate::{
    components::button::primary_button::PrimaryButton, pages::landing::components::SectionHeader,
};
use bdk::prelude::*;
use i18n::*;

#[component]
pub fn BecomeSponsorPage(#[props(default = Language::En)] lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: BecomeSponsorTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }

        div {
            id: "become-sponsor",
            class: "w-full max-w-desktop min-h-screen flex flex-col !justify-start gap-72 py-150 max-tablet:!px-30 max-tablet:!overflow-y-scroll max-tablet:!pt-40 px-10",
            SectionHeader {
                section_name: tr.section_name,
                title: tr.title,
                description: tr.description,
            }

            div { class: "flex flex-col gap-8 items-start",
                h2 { class: "text-2xl/22.5 font-bold text-text-primary", {tr.contact_us} }
                p { class: "text-[15px]/22.5 font-normal text-text-secondary", {tr.limit} }
            }

            div { class: "w-full flex flex-col gap-20 items-center justify-center",
                PrimaryButton {
                    width: "30%",
                    onclick: move |_| {
                        let ctrl = ctrl.clone();
                        async move {
                            tracing::debug!("Become Sponsor Clicked");
                            ctrl.notify_slack().await;
                        }
                    },
                    {tr.support_us}
                }
            }
        }
    }
}
