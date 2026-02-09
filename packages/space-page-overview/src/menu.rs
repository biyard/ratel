use crate::*;

pub fn get_nav_item(
    space_id: SpacePartition,
    _role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    Some((
        icon(),
        SpacePage::Overview,
        Route::HomePage { space_id }.into(),
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
                d: "M11.2498 2.5H9.99984H7.1665C5.50965 2.5 4.1665 3.84315 4.1665 5.5V14.5C4.1665 16.1569 5.50965 17.5 7.16651 17.5H12.8332C14.49 17.5 15.8332 16.1569 15.8332 14.5V7.1875M11.2498 2.5L15.8332 7.1875M11.2498 2.5V6.1875C11.2498 6.73978 11.6976 7.1875 12.2498 7.1875H15.8332",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M7.5 10.8334H12.5",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M7.5 14.1666H12.5",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
        }
    }
}
