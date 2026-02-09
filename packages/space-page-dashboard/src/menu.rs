use crate::*;

pub fn get_nav_item(
    space_id: SpacePartition,
    _role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget, i64)> {
    Some((
        icon(),
        SpacePage::Dashboard,
        Route::HomePage { space_id }.into(),
        0,
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
            path {
                d: "M10.0002 10C10.0002 9.5398 10.3733 9.16671 10.8335 9.16671H15.8335C16.2937 9.16671 16.6668 9.5398 16.6668 10V15.8334C16.6668 16.2936 16.2937 16.6667 15.8335 16.6667H10.8335C10.3733 16.6667 10.0002 16.2936 10.0002 15.8334V10Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M3.3335 4.16671C3.3335 3.70647 3.70659 3.33337 4.16683 3.33337H6.66683C7.12707 3.33337 7.50016 3.70647 7.50016 4.16671V15.8334C7.50016 16.2936 7.12707 16.6667 6.66683 16.6667H4.16683C3.70659 16.6667 3.3335 16.2936 3.3335 15.8334V4.16671Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M10.0002 4.16671C10.0002 3.70647 10.3733 3.33337 10.8335 3.33337H15.8335C16.2937 3.33337 16.6668 3.70647 16.6668 4.16671V5.83337C16.6668 6.29361 16.2937 6.66671 15.8335 6.66671H10.8335C10.3733 6.66671 10.0002 6.29361 10.0002 5.83337V4.16671Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.5",
            }
        }
    }
}
