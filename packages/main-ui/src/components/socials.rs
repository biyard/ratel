#![allow(non_snake_case)]
use crate::components::icons;
use bdk::prelude::*;

#[component]
pub fn Socials(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default = 29)] size: i32,
    onclick: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        div {..attributes,
            a {
                href: "https://x.com/theangryratel",
                target: "_blank",
                onclick: move |_| {
                    if let Some(onclick) = onclick {
                        onclick(());
                    }
                },
                alt: "X",
                icons::X { class: "hover:[&>path]:fill-primary", size }
            }
            a {
                href: "https://bsky.app/profile/angry-ratel.bsky.social",
                target: "_blank",
                alt: "BlueSky",
                onclick: move |_| {
                    if let Some(onclick) = onclick {
                        onclick(());
                    }
                },
                icons::Bsky { class: "hover:[&>g>path]:fill-primary", size }
            }
            a {
                href: "#",
                target: "_blank",
                alt: "Telegram",
                onclick: move |_| {
                    if let Some(onclick) = onclick {
                        onclick(());
                    }
                },
                icons::Telegram {
                    class: "[&>g>path]:fill-btn-p-disabled cursor-not-allowed",
                    size,
                }
            }
        }
    }
}
