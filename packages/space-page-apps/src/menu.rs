use crate::*;

pub fn get_nav_item(
    space_id: SpacePartition,
    role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget, i64)> {
    if role != SpaceUserRole::Creator {
        return None;
    }
    Some((icon(), SpacePage::Apps, Route::HomePage { space_id }.into(), 3))
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
                d: "M3.33325 4.16666C3.33325 3.70642 3.70635 3.33333 4.16659 3.33333H7.49992C7.96016 3.33333 8.33325 3.70642 8.33325 4.16666V7.49999C8.33325 7.96023 7.96016 8.33333 7.49992 8.33333H4.16659C3.70635 8.33333 3.33325 7.96023 3.33325 7.49999V4.16666Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M3.33325 12.5C3.33325 12.0398 3.70635 11.6667 4.16659 11.6667H7.49992C7.96016 11.6667 8.33325 12.0398 8.33325 12.5V15.8333C8.33325 16.2936 7.96016 16.6667 7.49992 16.6667H4.16659C3.70635 16.6667 3.33325 16.2936 3.33325 15.8333V12.5Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M11.6666 4.16666C11.6666 3.70642 12.0397 3.33333 12.4999 3.33333H15.8333C16.2935 3.33333 16.6666 3.70642 16.6666 4.16666V7.49999C16.6666 7.96023 16.2935 8.33333 15.8333 8.33333H12.4999C12.0397 8.33333 11.6666 7.96023 11.6666 7.49999V4.16666Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M11.6666 12.5C11.6666 12.0398 12.0397 11.6667 12.4999 11.6667H15.8333C16.2935 11.6667 16.6666 12.0398 16.6666 12.5V15.8333C16.6666 16.2936 16.2935 16.6667 15.8333 16.6667H12.4999C12.0397 16.6667 11.6666 16.2936 11.6666 15.8333V12.5Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
        }
    }
}
