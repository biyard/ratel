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
                href: "https://t.me/AngryRatel",
                target: "_blank",
                alt: "Telegram",
                onclick: move |_| {
                    if let Some(onclick) = onclick {
                        onclick(());
                    }
                },
                icons::Telegram { class: "hover:[&>g>path]:fill-primary", size }
            }
            a {
                href: "https://discord.gg/MA6tqpHN4T",
                target: "_blank",
                alt: "Discord",
                onclick: move |_| {
                    if let Some(onclick) = onclick {
                        onclick(());
                    }
                },
                icons::Discord { class: "hover:[&>g>path]:fill-primary", size }
            }
            a {
                href: "https://www.youtube.com/@Angry_Ratel",
                target: "_blank",
                alt: "Youtube",
                onclick: move |_| {
                    if let Some(onclick) = onclick {
                        onclick(());
                    }
                },
                icons::Youtube { class: "hover:[&>g>path]:fill-primary", size }
            }
        }
    }
}
