use crate::*;

// Dashboard Route enum - 완전한 경로로 정의 (독립 실행 가능)
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/dashboard")]
        #[route("/")]
        Main { space_id: SpacePartition },
        #[route("/sub-page")]
        Sub { space_id: SpacePartition },
}

#[component]
pub fn Main(space_id: ReadSignal<SpacePartition>) -> Element {
    async fn future(n: SpacePartition) -> SpacePartition {
        n
    }
    let space_id = use_resource(move || future(space_id()));

    match space_id() {
        Some(id) => {
            // let extensions = use_resource(|| fetch_extensions_from_db(id));
            let extensions = vec![];  // 임시: 빈 배열
            
            rsx! {
                views::HomePage {
                    space_id: id,
                    extensions: extensions
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
