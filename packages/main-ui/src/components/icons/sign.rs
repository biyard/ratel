use bdk::prelude::*;

#[component]
pub fn SignIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = 14)] size: i32,
) -> Element {
    rsx! {

        svg {
            height: "{size}",
            view_box: "0 0 14 14",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            g {
                fill: "none",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1",
                path { d: "M10.05 7a3 3 0 1 1-5.999 0a3 3 0 0 1 5.999 0" }
                path { d: "M10.05 7v1.3c0 3.49 5.47.2 2.6-4.54A6.59 6.59 0 0 0 7 .5A6.5 6.5 0 1 0 9.52 13" }
            }
        }
    }
}
