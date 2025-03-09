#![allow(non_snake_case)]
use crate::components::icons;
use dioxus::prelude::*;

#[component]
pub fn Socials(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        div {..attributes,
            a {
                href: "https://x.com/theangryratel",
                target: "_blank",
                alt: "X",
                icons::X {}
            }
            a {
                href: "https://bsky.app/profile/angry-ratel.bsky.social",
                target: "_blank",
                alt: "BlueSky",
                icons::Bsky {}
            }
                // a { href: "", icons::Telegram {} }
        }
    }
}
