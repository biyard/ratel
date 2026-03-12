use crate::common::*;
use dioxus::prelude::*;

#[component]
pub fn TeamHeader(
    display_name: String,
    profile_url: String,
    description: String,
) -> Element {
    rsx! {
        div { class: "relative w-full mb-6",
            // Banner
            div {
                class: "w-full rounded-[10px] overflow-hidden",
                style: "height: 140px; background-color: #2a2a2a;",
            }

            // Avatar + Team info row (avatar overlaps banner by ~50px)
            div {
                class: "flex items-end gap-8 px-6",
                style: "margin-top: -52px;",

                // Avatar
                if !profile_url.is_empty() {
                    img {
                        src: "{profile_url}",
                        alt: "{display_name}",
                        class: "shrink-0 rounded-[24px] object-cover object-top",
                        style: "width: 100px; height: 100px; border: 10px solid var(--background, #1d1d1d); flex-shrink: 0;",
                    }
                } else {
                    div {
                        class: "shrink-0 rounded-[24px] bg-neutral-600",
                        style: "width: 100px; height: 100px; border: 10px solid var(--background, #1d1d1d); flex-shrink: 0;",
                    }
                }

                // Team name + follow + description
                div { class: "flex flex-col gap-2 pb-3 flex-1 min-w-0",
                    h1 {
                        class: "text-2xl font-bold text-text-primary whitespace-nowrap",
                        "{display_name}"
                    }
                    if !description.is_empty() {
                        p {
                            class: "text-sm text-foreground leading-5",
                            style: "overflow: hidden; display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical;",
                            dangerous_inner_html: description,
                        }
                    }
                }
            }
        }
    }
}
