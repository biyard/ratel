mod interop_theme;
mod moon;
mod sun;
mod sun_moon;
mod theme;
mod theme_service;

pub use interop_theme::*;
pub use moon::*;
pub use sun::*;
pub use sun_moon::*;
pub use theme::*;
pub use theme_service::*;

use crate::*;
use dioxus::prelude::*;

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut theme_service = use_theme();
    let current = theme_service.current();
    let mut open = use_signal(|| false);

    let options = [Theme::Light, Theme::Dark, Theme::System];

    rsx! {
        div { class: "relative",
            button {
                class: "flex flex-col items-center py-2 px-3 rounded-lg transition-colors cursor-pointer hover:bg-accent-hover",
                onclick: move |_| {
                    let v = *open.read();
                    open.set(!v);
                },

                span { class: "text-sm", {current.icon()} }
                span { class: "text-sm font-medium", "{current.label()}" }
            }

            if *open.read() {
                div { class: "overflow-hidden absolute right-0 top-full z-50 mt-1 rounded-lg border shadow-lg min-w-[140px] bg-card border-border",
                    for option in options {
                        button {
                            class: {
                                let selected = if option == current { "bg-accent-hover" } else { "" };
                                format!(
                                    "flex gap-2 items-center w-full px-3 py-2 text-sm hover:bg-accent-hover cursor-pointer transition-colors {selected}",
                                )
                            },
                            onclick: move |_| {
                                theme_service.set(option);
                                open.set(false);
                            },

                            span { {option.icon()} }
                            span { "{option.label()}" }
                        }
                    }
                }
            }
        }
    }
}
