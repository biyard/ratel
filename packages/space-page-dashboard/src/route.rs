use crate::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/dashboard")]
        #[route("/")]
        Main { space_id: SpacePartition },
        #[route("/sub-page")]
        Sub { space_id: SpacePartition },
}

fn load_extensions_from_json(json_str: &str) -> Vec<DashboardExtension> {
    serde_json::from_str(json_str).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON: {}", e);
        vec![]
    })
}

pub fn get_creator_extensions() -> Vec<DashboardExtension> {
    let json_data = include_str!("../assets/mock/creator_extensions.json");
    load_extensions_from_json(json_data)
}

pub fn get_viewer_extensions() -> Vec<DashboardExtension> {
    let json_data = include_str!("../assets/mock/viewer_extensions.json");
    load_extensions_from_json(json_data)
}

pub fn get_candidate_extensions() -> Vec<DashboardExtension> {
    let json_data = include_str!("../assets/mock/candidate_extensions.json");
    load_extensions_from_json(json_data)
}

pub fn get_participant_extensions() -> Vec<DashboardExtension> {
    let json_data = include_str!("../assets/mock/participant_extensions.json");
    load_extensions_from_json(json_data)
}

#[component]
pub fn Main(space_id: ReadSignal<SpacePartition>) -> Element {
    async fn future(n: SpacePartition) -> SpacePartition {
        n
    }
    let space_id = use_resource(move || future(space_id()));

    match space_id() {
        Some(id) => {
            rsx! {
                views::HomePage {
                    space_id: id,
                }
            }
        },
        None => rsx! {},
    }
}

#[component]
pub fn Sub(space_id: SpacePartition) -> Element {
    rsx! {
        div {
            class: "p-4",
            h1 { class: "text-2xl font-bold mb-4", "Dashboard Sub Page" }
            p { "Space ID: { space_id.to_string() }" }

            Link {
                to: Route::Main { space_id: space_id.clone() },
                class: "text-blue-500 hover:underline",
                "Back to Main"
            }
        }
    }
}
