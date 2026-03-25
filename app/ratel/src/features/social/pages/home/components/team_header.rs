use crate::common::*;
use crate::common::assets::TEAM_BANNER_DEFAULT;
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
) -> Element {
    rsx! {
        div { class: "w-full isolate",
            // Banner
            div {
                class: "relative z-0 w-full rounded-[10px] bg-card-bg overflow-hidden",
                style: "height: 180px; transform: translateZ(0);",
                img {
                    src: if !thumbnail_url.is_empty() { thumbnail_url.clone() } else { TEAM_BANNER_DEFAULT.to_string() },
                    alt: "banner",
                    class: "w-full h-full object-cover",
                }
                if is_creator {
                    Link {
                        to: "{settings_route}",
                        class: "absolute top-3 right-3 flex items-center justify-center w-8 h-8 rounded-lg bg-black/40 hover:bg-black/60 transition-colors",
                        lucide_dioxus::Settings {
                            class: "w-[18px] h-[18px] [&>path]:stroke-white [&>line]:stroke-white [&>polyline]:stroke-white [&>circle]:stroke-white",
                        }
                    }
                }
            }

            // Avatar + Team info
            div { class: "relative z-10 flex items-start gap-4 px-4 -mt-7",
                if !profile_url.is_empty() {
                    img {
                        src: "{profile_url}",
                        alt: "{display_name}",
                        class: "shrink-0 w-20 h-20 rounded-[16px] object-cover object-top border-4 border-background",
                    }
                } else {
                    div { class: "shrink-0 w-20 h-20 rounded-[16px] bg-neutral-600 border-4 border-background" }
                }

                div { class: "flex flex-col gap-1 pt-9 flex-1 min-w-0",
                    div { class: "flex items-center gap-3",
                        h1 { class: "text-xl font-bold text-text-primary", "{display_name}" }
                        if logged_in {
                            if is_following {
                                button {
                                    class: "px-4 py-1 rounded-full border border-border text-sm font-semibold text-foreground-muted hover:bg-hover transition-colors disabled:opacity-50",
                                    disabled: processing,
                                    onclick: move |_| on_unfollow.call(()),
                                    "Unfollow"
                                }
                            } else {
                                button {
                                    class: "px-4 py-1 rounded-full border border-border text-sm font-semibold text-text-primary hover:bg-hover transition-colors disabled:opacity-50",
                                    disabled: processing,
                                    onclick: move |_| on_follow.call(()),
                                    "Follow"
                                }
                            }
                        }
                    }
                    if !description.is_empty() {
                        p {
                            class: "text-sm text-foreground-muted leading-5 line-clamp-2",
                            dangerous_inner_html: description,
                        }
                    }
                }
            }
        }
    }
}
