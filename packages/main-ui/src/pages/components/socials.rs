#![allow(non_snake_case)]
use crate::components::icons;
use bdk::prelude::*;

#[component]
pub fn Socials(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = 29)] size: i32,
) -> Element {
    rsx! {
        div {..attributes,
            a {
                href: "https://x.com/theangryratel",
                target: "_blank",
                alt: "X",
                icons::X { class: "hover:[&>path]:fill-primary", size }
            }
            a {
                href: "https://bsky.app/profile/angry-ratel.bsky.social",
                target: "_blank",
                alt: "BlueSky",
                icons::Bsky { class: "hover:[&>g>path]:fill-primary", size }
            }
            a { href: "#", target: "_blank", alt: "Telegram",
                icons::Telegram {
                    class: "[&>g>path]:fill-btn-p-disabled cursor-not-allowed",
                    size,
                }
            }
        }
    }
}
