use crate::common::*;

#[component]
pub fn Moon(#[props(extends = SvgAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-moon-icon lucide-moon",
            fill: "none",
            height: "24",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "24",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401" }
        }
    }
}
