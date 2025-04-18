#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn Progressive(#[props(default = "181".to_string())] size: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "{size}",
            view_box: "0 0 181 181",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                width: "180",
                height: "180",
                transform: "translate(0.414062 0.74707)",
                fill: "#282828",
            }
            circle {
                cx: "90.4141",
                cy: "90.7471",
                r: "90",
                fill: "white",
            }
            g { clip_path: "url(#clip0_0_1)",
                path {
                    d: "M54.3213 107.091H39.9595V100.078H29.709V115.863H64.5714V71.2476H54.3213V107.091Z",
                    fill: "#D6001C",
                }
                path {
                    d: "M81.2826 84.7882H99.6373V89.5687H81.2826V84.7882ZM95.585 98.5954H109.888V71.2478H99.6373V76.3087H81.2826V71.2478H71.0324V98.5954H85.3345V107.091H70.627V115.864H110.293V107.091H95.585V98.5954Z",
                    fill: "#D6001C",
                }
                path {
                    d: "M140.648 108.048C138.111 108.048 136.047 105.994 136.047 103.47C136.047 100.946 138.111 98.8917 140.648 98.8917C143.185 98.8917 145.249 100.946 145.249 103.47C145.249 105.994 143.185 108.048 140.648 108.048ZM145.249 71.2478V89.3225H126.321V80.0205H140.267V71.2478H116.071V98.5954H126.639C126.096 100.115 125.797 101.753 125.797 103.47C125.797 111.618 132.459 118.248 140.648 118.248C148.837 118.248 155.499 111.618 155.499 103.47V80.0205H161.914V71.2478H145.249Z",
                    fill: "#D6001C",
                }
                path {
                    d: "M42.6647 98.785L50.6939 93.7633L41.9167 80.0205H49.8755V71.2479H19.7325V80.0205H27.6912L18.9141 93.7633L26.9429 99.0298L34.804 87.4238L42.6647 98.785Z",
                    fill: "#D6001C",
                }
            }
            defs {
                clipPath { id: "clip0_0_1",
                    rect {
                        width: "143",
                        height: "47",
                        fill: "white",
                        transform: "translate(18.9141 71.2471)",
                    }
                }
            }
        }
    }
}
