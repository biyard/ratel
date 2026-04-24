use super::*;

#[component]
pub fn ActionEditTopbar(
    space_name: String,
    action_type_label: String,
    /// Kebab-case action type key used to color the type badge. One of:
    /// `"poll"`, `"quiz"`, `"discussion"`, `"follow"`. Falls back to the
    /// default gold accent if unspecified.
    #[props(default)]
    action_type_key: String,
    title: Signal<String>,
    on_title_change: EventHandler<String>,
    on_cancel: EventHandler<()>,
    #[props(default = true)] editable_title: bool,
    on_back: EventHandler<()>,
) -> Element {
    let tr: ActionEditTopbarTranslate = use_translate();

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }

        header { class: "arena-topbar", role: "banner",
            div { class: "arena-topbar__left",
                button {
                    class: "back-btn",
                    aria_label: "{tr.back}",
                    onclick: move |_| on_back.call(()),
                    svg {
                        width: "16",
                        height: "16",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M19 12H5M12 19l-7-7 7-7" }
                    }
                }
                nav { class: "breadcrumb",
                    span { class: "breadcrumb__item", "{space_name}" }
                    span { class: "breadcrumb__sep", "\u{203A}" }
                    span { class: "breadcrumb__item breadcrumb__current", "{tr.actions_breadcrumb}" }
                }
                span {
                    class: "type-badge",
                    "data-action-type": action_type_key,
                    "{action_type_label}"
                }
                if editable_title {
                    input {
                        class: "topbar-title-input",
                        r#type: "text",
                        value: "{title}",
                        aria_label: "{tr.title_aria}",
                        oninput: move |e| on_title_change.call(e.value()),
                    }
                } else {
                    span { class: "topbar-title-input topbar-title-input--readonly",
                        "{title}"
                    }
                }
            }
            div { class: "arena-topbar__right",
                button {
                    class: "btn btn--ghost",
                    onclick: move |_| on_cancel.call(()),
                    "{tr.cancel}"
                }
            }
        }
    }
}
