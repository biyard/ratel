use bdk::prelude::*;

#[component]
pub fn Grade(
    #[props(default = "".to_string())] class: String,
    #[props(default = "none".to_string())] fill: String,
    #[props(default = "24".to_string())] width: String,
    #[props(default = "24".to_string())] height: String,
) -> Element {
    rsx! {
        svg {
            class,

            fill,
            height,
            view_box: "0 0 24 24",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M21.1924 6.69238V17.3066L12 22.6143L2.80762 17.3066V6.69238L12 1.38477L21.1924 6.69238Z",
                fill: "url(#paint0_linear_1458_7152)",
                stroke: "url(#paint1_linear_1458_7152)",
                stroke_width: "2.4",
            }
            path {
                d: "M12.0001 4.07935L18.859 8.03935V15.9593L12.0001 19.9193L5.14116 15.9593V8.03935L12.0001 4.07935Z",
                fill: "url(#paint2_linear_1458_7152)",
            }
            mask {
                fill: "black",
                height: "8",
                id: "path-3-outside-1_1458_7152",
                mask_units: "userSpaceOnUse",
                width: "5",
                x: "9.2002",
                y: "7.20044",
                rect {
                    fill: "white",
                    height: "8",
                    width: "5",
                    x: "9.2002",
                    y: "7.20044",
                }
                path { d: "M13.1668 8.61498V14.2004H11.6504V10.0113H11.6177L10.3959 10.7423V9.45498L11.7704 8.61498H13.1668Z" }
            }
            path {
                d: "M13.1668 8.61498V14.2004H11.6504V10.0113H11.6177L10.3959 10.7423V9.45498L11.7704 8.61498H13.1668Z",
                fill: "#FFD8BD",
            }
            path {
                d: "M13.1668 8.61498H14.1268V7.65498H13.1668V8.61498ZM13.1668 14.2004V15.1604H14.1268V14.2004H13.1668ZM11.6504 14.2004H10.6904V15.1604H11.6504V14.2004ZM11.6504 10.0113H12.6104V9.05135H11.6504V10.0113ZM11.6177 10.0113V9.05135H11.3525L11.1249 9.18751L11.6177 10.0113ZM10.3959 10.7423H9.43588V12.4352L10.8887 11.5661L10.3959 10.7423ZM10.3959 9.45498L9.89528 8.63583L9.43588 8.91658V9.45498H10.3959ZM11.7704 8.61498V7.65498H11.5003L11.2698 7.79583L11.7704 8.61498ZM13.1668 8.61498H12.2068V14.2004H13.1668H14.1268V8.61498H13.1668ZM13.1668 14.2004V13.2404H11.6504V14.2004V15.1604H13.1668V14.2004ZM11.6504 14.2004H12.6104V10.0113H11.6504H10.6904V14.2004H11.6504ZM11.6504 10.0113V9.05135H11.6177V10.0113V10.9713H11.6504V10.0113ZM11.6177 10.0113L11.1249 9.18751L9.90304 9.91842L10.3959 10.7423L10.8887 11.5661L12.1105 10.8352L11.6177 10.0113ZM10.3959 10.7423H11.3559V9.45498H10.3959H9.43588V10.7423H10.3959ZM10.3959 9.45498L10.8965 10.2741L12.271 9.43414L11.7704 8.61498L11.2698 7.79583L9.89528 8.63583L10.3959 9.45498ZM11.7704 8.61498V9.57498H13.1668V8.61498V7.65498H11.7704V8.61498Z",
                fill: "#291551",
                mask: "url(#path-3-outside-1_1458_7152)",
            }
            defs {
                linearGradient {
                    gradient_units: "userSpaceOnUse",
                    id: "paint0_linear_1458_7152",
                    x1: "5.88",
                    x2: "17.64",
                    y1: "3.72",
                    y2: "21.12",
                    stop { stop_color: "#4A4A78" }
                    stop { offset: "1", stop_color: "#F1F1F1" }
                }
                linearGradient {
                    gradient_units: "userSpaceOnUse",
                    id: "paint1_linear_1458_7152",
                    x1: "6.72",
                    x2: "16.32",
                    y1: "3.6",
                    y2: "21.12",
                    stop { stop_color: "#D8D8D8" }
                    stop { offset: "1", stop_color: "#7769A3" }
                }
                linearGradient {
                    gradient_units: "userSpaceOnUse",
                    id: "paint2_linear_1458_7152",
                    x1: "12.0001",
                    x2: "12.0001",
                    y1: "4.07935",
                    y2: "19.9193",
                    stop { stop_color: "#2F195C" }
                    stop { offset: "1", stop_color: "#4D3483" }
                }
            }
        }
    }
}
