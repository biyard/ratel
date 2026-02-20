mod some_component;
use some_component::SomeComponent;

use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    debug!("Rendering Draft Home for user: {}", username);
    rsx! {
        div { class: "flex flex-col items-center justify-center w-full h-full",
            h1 { class: "text-2xl font-bold", "ratel-user-draft" }
            p { class: "mt-2 text-gray-500", "Coming soon..." }
            SomeComponent {}
        }
    }
}
