#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn BasicIncome(#[props(default = "181".to_string())] size: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "{size}",
            view_box: "0 0 181 181",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                cx: "90.4141",
                cy: "90.2695",
                fill: "#68D2C0",
                r: "90",
            }
            g { clip_path: "url(#clip0_93_4752)",
                path {
                    d: "M81.9418 68.6238H78.8584V88.2172H81.9418V68.6238Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M75.9817 73.8185C75.9817 73.1912 76.0014 72.8187 76.0014 72.0248C76.0014 71.2309 76.0802 70.1821 75.4301 69.545C74.7503 68.8785 74.0509 68.9079 73.0953 68.9079L63.4707 68.9471L64.6824 71.77H72.7308C72.7308 71.77 72.9377 74.8085 72.5437 75.4358C71.0167 77.847 63.6776 85.5314 63.6776 85.5314L66.0222 87.3251C66.0222 87.3251 73.45 79.9739 75.4695 76.6708C75.8537 76.0337 75.962 75.2005 75.9817 73.7989V73.8185Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M95.6247 79.131V77.5334H101.053C102.166 77.5334 102.727 76.7002 102.727 75.9847V68.9766H99.644V70.9565L89.0441 70.9957L90.1869 73.4167H99.6539V75.0536H88.591V68.9766H85.5075V75.9063C85.5075 76.7198 86.0888 77.5334 87.1823 77.5334H92.5709V79.131H84.4141V81.6206H103.841V79.131H95.6346H95.6247H95.6346Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M88.5122 82.4929H85.2021V86.6096C85.2021 87.3153 85.9213 88.1092 86.8769 88.1092H103.013V85.2178H88.5122V82.4929Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M85.291 103.655V106.438H99.8314V110.271H102.994L103.013 105.605C103.013 103.772 101.831 103.655 101.033 103.655H85.291Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M103.821 99.783H84.3945V102.273H103.821V99.783Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M86.7884 98.401L102.925 98.3618V95.7448L88.2956 95.8036V93.4806L101.585 93.461L102.737 90.8538H86.7884C85.9116 90.8538 85.291 91.4125 85.291 92.2064V97.0582C85.291 97.8521 85.9018 98.401 86.7884 98.401Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M118.421 98.6067H111.545C108.264 98.6067 106.067 100.283 106.067 102.772C106.067 105.262 108.116 106.938 111.545 106.938H118.421C121.839 106.938 123.878 105.38 123.878 102.772C123.878 100.165 121.682 98.6067 118.421 98.6067ZM120.421 102.821C120.421 103.694 119.721 104.145 118.125 104.145H111.515C110.254 104.145 109.525 103.625 109.525 102.821C109.525 101.929 110.116 101.498 111.515 101.498H118.125C119.711 101.498 120.421 101.939 120.421 102.821Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M108.668 94.3235V90.5009L117.387 90.4813L118.657 87.7957H107.151C106.393 87.7957 105.486 88.0897 105.486 89.4913V95.4899C105.486 96.2054 106.196 96.9994 107.141 97.0092C107.979 97.0092 109.496 97.0092 111.141 97.0092C114.421 97.0092 118.234 97.0092 118.283 97.0092H118.549V94.3137L108.668 94.3333V94.3235Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M123.11 91.0203L123.13 87.3545L120.007 87.3447L119.978 97.6756H123.081L123.091 94.098H125.612V91.0203H123.11Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M74.3169 103.753V97.4795H71.0857V103.753H62.9092V106.703H82.4835V103.753H74.3169Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M80.5226 100.322L82.483 97.4893L75.2227 88.9031L73.4199 91.922L80.5226 100.322Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M64.9089 100.44L62.9189 97.6756L70.9477 88.2171C71.4796 87.5996 72.0412 87.3447 72.78 87.3447C73.4597 87.3447 73.8538 87.5408 74.4448 88.0504L74.9473 88.5503L64.9187 100.44H64.9089Z",
                    fill: "#091E3A",
                }
                path {
                    d: "M90.4041 25.8987C56.0825 25.8987 28.2627 53.5784 28.2627 87.7272C28.2627 121.876 56.0825 149.556 90.4041 149.556C92.9457 149.556 95.4577 149.399 97.9205 149.105L90.1578 170.178C90.1578 170.178 152.516 145.174 152.565 87.7762C152.565 53.6274 124.726 25.8987 90.4041 25.8987ZM134.419 88.3055C134.419 128.972 99.5755 144.43 99.5755 144.43L105.299 128.992L105.338 128.933C100.679 130.609 95.6449 131.521 90.4041 131.521C66.0913 131.521 46.3791 111.908 46.3791 87.7174C46.3791 63.527 66.0913 43.9238 90.4041 43.9238C114.717 43.9238 134.419 63.429 134.419 88.3055C134.419 88.6878 134.419 87.9232 134.419 88.3055Z",
                    fill: "#091E3A",
                }
            }
            defs {
                clipPath { id: "clip0_93_4752",
                    rect {
                        fill: "white",
                        height: "144.28",
                        transform: "translate(28.2627 25.8987)",
                        width: "124.302",
                    }
                }
            }
        }
    }
}
