use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn MyNetworkPage(#[props(default = Language::En)] lang: Language) -> Element {
    let ctrl = Controller::new(lang)?;
    let tr: MyNetworkTranslate = translate(&lang);
    let connections = ctrl.get_connections();

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div {
            class: "p-6 max-w-3xl mx-auto",
            h1 {
                class: "text-2xl font-bold mb-4",
                "{tr.title}"
            }

            div {
                class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                for user in connections {
                    div {
                        class: "bg-white shadow-md rounded-lg p-4 border border-gray-200 hover:shadow-lg transition",
                        h2 { class: "text-lg font-semibold", "{user}" }
                        p { class: "text-sm text-gray-600", "You are connected with {user}." }
                    }
                }
            }
        }
    }
}
