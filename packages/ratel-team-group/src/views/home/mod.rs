mod some_component;
use some_component::SomeComponent;

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center w-full h-full",
            h1 { class: "text-2xl font-bold", "ratel-team-group" }
            p { class: "mt-2 text-gray-500", "Coming soon..." }
            SomeComponent {}
        }
    }
}
