use crate::common::assets::TEAM_BANNER_DEFAULT;
use crate::common::*;
use dioxus::prelude::*;

#[component]
pub fn TeamHeader(
    display_name: String,
    profile_url: String,
    description: String,
    thumbnail_url: String,
    is_creator: bool,
    settings_route: String,
    is_following: bool,
    processing: bool,
    on_follow: EventHandler<()>,
    on_unfollow: EventHandler<()>,
    logged_in: bool,
    /// Optional chip / badge rendered on the right side of the avatar+title row.
    /// Used by user profile views to display the public Character Level chip.
    #[props(default)]
    right_slot: Option<Element>,
) -> Element {
    rsx! {
        div { class: "w-full isolate",
            // Banner
            div {
                class: "overflow-hidden relative z-0 w-full rounded-[10px] bg-card-bg",
                style: "height: 180px; transform: translateZ(0);",
                img {
                    src: if !thumbnail_url.is_empty() { thumbnail_url.clone() } else { TEAM_BANNER_DEFAULT.to_string() },
                    alt: "banner",
                    class: "object-cover w-full h-full",
                }
                if is_creator {
                    Link {
                        to: "{settings_route}",
                        class: "flex absolute top-3 right-3 justify-center items-center w-8 h-8 rounded-lg transition-colors bg-black/40 hover:bg-black/60",
                        lucide_dioxus::Settings { class: "w-[18px] h-[18px] [&>path]:stroke-white [&>line]:stroke-white [&>polyline]:stroke-white [&>circle]:stroke-white" }
                    }
                }
            }

            // Avatar + Team info
            div { class: "flex relative z-10 gap-4 items-start px-4 -mt-7",
                if !profile_url.is_empty() {
                    img {
                        src: "{profile_url}",
                        alt: "{display_name}",
                        class: "object-cover object-top w-20 h-20 shrink-0 rounded-[16px]",
                    }
                } else {
                    div { class: "w-20 h-20 shrink-0 rounded-[16px] bg-neutral-600" }
                }

                div { class: "flex flex-col flex-1 gap-1 pt-9 min-w-0",
                    div { class: "flex gap-3 items-center",
                        h1 { class: "text-xl font-bold text-text-primary", "{display_name}" }
                        if logged_in {
                            if is_following {
                                button {
                                    class: "py-1 px-4 text-sm font-semibold rounded-full border transition-colors disabled:opacity-50 border-border text-foreground-muted hover:bg-hover",
                                    disabled: processing,
                                    onclick: move |_| on_unfollow.call(()),
                                    "Unfollow"
                                }
                            } else {
                                button {
                                    class: "py-1 px-4 text-sm font-semibold rounded-full border transition-colors disabled:opacity-50 border-border text-text-primary hover:bg-hover",
                                    disabled: processing,
                                    onclick: move |_| on_follow.call(()),
                                    "Follow"
                                }
                            }
                        }
                    }
                    if !description.is_empty() {
                        p {
                            class: "text-sm leading-5 text-foreground-muted line-clamp-2",
                            dangerous_inner_html: description,
                        }
                    }
                }

                if let Some(slot) = right_slot {
                    div { class: "self-start pt-9 shrink-0", {slot} }
                }
            }
        }
    }
}
