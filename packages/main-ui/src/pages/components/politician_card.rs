#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::route::Route;

#[component]
pub fn PoliticianCard(
    lang: Language,
    id: i64,
    name: String,
    party: String,
    image_url: String,
) -> Element {
    rsx! {
        Link {
            to: Route::PoliticiansByIdPage {
                lang,
                id,
            },
            class: "relative col-span-1 w-full h-full rounded-[10px] overflow-hidden max-[900px]:!min-w-[210px]",

            background_image: format!("url({})", image_url),
            background_size: "cover",
            background_position: "center",
            background_repeat: "no-repeat",

            div {
                class: "absolute bottom-0 left-0 w-full h-100",
                background: "linear-gradient(180deg, rgba(0, 2, 3, 0) 0%, rgba(0, 2, 3, 0.5) 40%, rgba(0, 2, 3, 0.8) 100%, rgba(0, 2, 3, 0.9) 100%)",
            }

            div { class: "absolute bottom-0 left-0 w-full flex flex-col justify-start gap-4 px-10 py-15",
                p { class: "text-white text-lg/22 font-bold", "{name}" }
                div { class: "flex flex-row items-center gap-8",
                    PPP { size: "18" }
                    p { class: "text-white text-[15px]/22 font-medium", "{party}" }
                }
            }
        }
    }
}

#[component]
pub fn PPP(#[props(default = "181".to_string())] size: String) -> Element {
    rsx! {

        svg {
            fill: "none",
            height: "{size}",
            view_box: "0 0 181 181",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                cx: "90.5508",
                cy: "90.9854",
                fill: "white",
                r: "90",
            }
            g { clip_path: "url(#clip0_93_4751)",
                path {
                    d: "M91.9893 134.419L92.3257 141.277L113.763 140.383L113.488 133.815L91.9893 134.419Z",
                    fill: "#241714",
                }
                path {
                    d: "M102.509 117.935C104.833 117.935 105.911 119.487 105.911 121.33C105.911 123.173 104.734 124.725 102.509 124.725C100.284 124.725 99.1069 123.173 99.1069 121.33C99.1069 119.487 100.215 117.935 102.509 117.935ZM102.509 131.414C109.099 131.414 112.593 126.819 112.593 121.338C112.593 115.856 109.405 111.261 102.509 111.261C95.6129 111.261 92.4248 115.856 92.4248 121.338C92.4248 126.819 95.8576 131.414 102.509 131.414Z",
                    fill: "#241714",
                }
                path {
                    d: "M67.5467 132.553H60.3906V143.899H60.6429H88.6787V137.546L67.5467 137.714V132.553Z",
                    fill: "#241714",
                }
                path {
                    d: "M65.207 124.564V118.432H70.834V123.952C69.2132 124.342 67.1566 124.511 65.207 124.564ZM70.834 130.466H77.455V112.079H58.5479V130.535C58.5479 130.535 64.97 130.459 70.834 129.557V130.474V130.466Z",
                    fill: "#241714",
                }
                path {
                    d: "M87.7391 110.787H80.583V134.909H87.7391V110.787Z",
                    fill: "#241714",
                }
                path {
                    d: "M121.944 110.795H114.788V143.907H121.944V110.795Z",
                    fill: "#241714",
                }
                path {
                    d: "M147.251 117.653V112.599H140.37V110.26H133.214V112.599H126.333V117.653C126.333 117.653 132.151 117.301 136.547 117.301C140.943 117.301 147.251 117.653 147.251 117.653Z",
                    fill: "#241714",
                }
                path {
                    d: "M148.366 138.663H135.331V137.233H148.366V138.663ZM128.175 132.049V144.381H135.331V143.907H148.366V144.381H155.522V132.049H128.175Z",
                    fill: "#241714",
                }
                path {
                    d: "M136.715 122.546C138.619 122.546 139.911 123.264 139.911 124.266C139.911 125.267 138.619 125.994 136.715 125.994C134.811 125.994 133.519 125.275 133.519 124.266C133.519 123.257 134.811 122.546 136.707 122.546M136.791 130.711C143.489 130.711 146.662 127.775 146.662 124.266C146.662 120.757 143.81 117.821 136.791 117.821C129.773 117.821 126.929 120.757 126.929 124.266C126.929 127.775 130.033 130.711 136.791 130.711Z",
                    fill: "#241714",
                }
                path {
                    d: "M155.522 110.787H148.366V130.987H155.522V110.787Z",
                    fill: "#241714",
                }
                path {
                    d: "M23.7998 130.268H35.6273V132.554H26.361V138.937L45.207 138.586V143.922H52.3631V132.554H42.7834V130.268H54.9855V124.09H23.7998V130.268Z",
                    fill: "#241714",
                }
                path {
                    d: "M44.4197 123.142L51.2777 123.112C52.8373 116.957 52.9214 111.651 52.9214 111.651H26.4912V117.982L45.6659 117.767C45.4289 119.656 45.0466 121.628 44.4274 123.142",
                    fill: "#241714",
                }
                path {
                    d: "M81.347 37.5898L62.9521 56V92.828H99.8031L118.19 74.4178V37.5898H81.347Z",
                    fill: "#E60024",
                }
            }
            defs {
                clipPath { id: "clip0_93_4751",
                    rect {
                        fill: "white",
                        height: "106.791",
                        transform: "translate(23.7998 37.5898)",
                        width: "131.723",
                    }
                }
            }
        }
    }
}
