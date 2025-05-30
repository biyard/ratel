use bdk::prelude::*;

#[component]
pub fn Pentagon2(
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
            view_box: "0 0 20 21",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M10.3173 3.6416L17.2615 8.68687L14.7618 16.9749H5.87289L3.37305 8.68687L10.3173 3.6416Z",
                stroke: "#737373",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M14.7623 16.9747L12.54 14.4351",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M5.87305 16.9747L8.09527 14.4351",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M3.33301 8.7207L6.50761 9.67308",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M17.3016 8.7207L14.127 9.67308",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M10.3174 3.6416V6.8162",
                stroke: "#737373",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M10.3173 6.81592L12.2244 8.2669L14.1269 9.67306L12.5396 14.435H8.09511L6.50781 9.67306L8.41026 8.2669L10.3173 6.81592Z",
                stroke: "#737373",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M5.1091 7.42529L3.37305 8.68662L3.99801 10.7586",
                stroke: "#737373",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M5.24805 14.9028L5.87301 16.9749H8.09523",
                stroke: "#737373",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M12.54 16.9749H14.7623L15.3872 14.9028",
                stroke: "#737373",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M15.5254 7.42529L17.2614 8.68662L16.6365 10.7586",
                stroke: "#737373",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
            path {
                d: "M12.0532 4.90292L10.3171 3.6416L8.58105 4.90292",
                stroke: "#737373",
                stroke_linejoin: "round",
                stroke_width: "1.5",
            }
        }
    }
}
