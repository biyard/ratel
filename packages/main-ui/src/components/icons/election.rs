use bdk::prelude::*;

#[component]
pub fn PresidentialElectionIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
    #[props(default = 24)] size: i32,
) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "{size}",
            view_box: "0 0 24 24",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path {
                d: "M20 17H4V18.6C4 19.9255 5.07452 21 6.4 21H7H17H17.6C18.9255 21 20 19.9255 20 18.6V17Z",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M7 13.0137H6L4 17.0137H20L18 13.0137H17",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            rect {
                height: "11.2287",
                mask: "url(#path-3-inside-1_1733_19542)",
                rx: "1.2",
                stroke: "#737373",
                stroke_width: "1.5",
                transform: "rotate(44.541 12.8574 1)",
                width: "9.62457",
                x: "12.8574",
                y: "1",
            }
        }
    }
}
