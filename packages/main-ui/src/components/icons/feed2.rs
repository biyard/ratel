use bdk::prelude::*;

#[component]
pub fn Feed2(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "20".to_string())] width: String,
    #[props(default = "20".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            class,
            fill,
            height,
            view_box: "0 0 20 20",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M14.9991 7.40724C14.0446 5.48649 12.0625 4.1665 9.77218 4.1665C7.48178 4.1665 5.54437 5.48649 4.58984 7.40724",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.86667",
            }
            path {
                d: "M4.58984 4.81494V7.40753",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.86667",
            }
            path {
                d: "M6.78712 7.40723H4.58984",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.86667",
            }
            path {
                d: "M4.58984 12.5928C5.54437 14.5135 7.52644 15.8335 9.81683 15.8335C12.1072 15.8335 14.0446 14.5135 14.9991 12.5928",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.86667",
            }
            path {
                d: "M15 15.1854V12.5928",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.86667",
            }
            path {
                d: "M12.8027 12.5928H15",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_width: "1.86667",
            }
        }
    }
}
