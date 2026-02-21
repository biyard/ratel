use dioxus::prelude::*;

#[component]
pub fn AdminPage(teamname: String) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center w-full h-full",
            h1 { class: "text-2xl font-bold", "ratel-team-group (admin)" }
            p { class: "mt-2 text-gray-500", "Coming soon..." }
            p { class: "mt-2 text-gray-400", "team: {teamname}" }
        }
    }
}
