use dioxus::prelude::*;

#[component]
pub fn MetaMask() -> Element {
    rsx! {
        svg {
            width: "50",
            height: "51",
            view_box: "0 0 50 51",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M43.0156 5.64844L27.4004 17.2461L30.288 10.4036L43.0156 5.64844Z",
                fill: "#E2761B",
                stroke: "#E2761B",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M6.96875 5.64844L22.4584 17.3559L19.712 10.4036L6.96875 5.64844Z",
                fill: "#E4761B",
                stroke: "#E4761B",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M37.3991 32.5312L33.2402 38.9029L42.1385 41.3511L44.6966 32.6725L37.3991 32.5312Z",
                fill: "#E4761B",
                stroke: "#E4761B",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M5.32031 32.6725L7.86268 41.3511L16.761 38.9029L12.6022 32.5312L5.32031 32.6725Z",
                fill: "#E4761B",
                stroke: "#E4761B",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
