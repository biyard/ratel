use super::*;
use crate::features::spaces::space_common::controllers::SpaceResponse;

pub fn get_nav_item(
    space: &SpaceResponse,
    _role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    Some((
        icon(),
        SpacePage::Leaderboard,
        Route::SpaceLeaderboardPage {
            space_id: space.id.clone(),
        }
        .into(),
    ))
}

#[component]
pub fn icon() -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "20",
            view_box: "0 0 20 20",
            width: "20",
            xmlns: "http://www.w3.org/2000/svg",
            // Trophy icon
            path {
                d: "M6.66667 2.5H13.3333V10C13.3333 11.8409 11.841 13.3333 10 13.3333C8.15905 13.3333 6.66667 11.8409 6.66667 10V2.5Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M13.3333 4.16667H15.4167C16.1071 4.16667 16.6667 4.72631 16.6667 5.41667V5.83333C16.6667 7.67428 15.1743 9.16667 13.3333 9.16667",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M6.66667 4.16667H4.58333C3.89298 4.16667 3.33333 4.72631 3.33333 5.41667V5.83333C3.33333 7.67428 4.82572 9.16667 6.66667 9.16667",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M10 13.3333V15.8333",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M6.66667 17.5H13.3333",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.5",
            }
        }
    }
}
