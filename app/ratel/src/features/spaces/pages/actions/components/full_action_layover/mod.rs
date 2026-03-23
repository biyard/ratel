use crate::common::*;
use crate::features::spaces::layout::use_space_layout_ui;

#[component]
pub fn FullActionLayover(
    #[props(default)] content_class: String,
    #[props(default)] bottom_class: String,
    #[props(default)] bottom_left: Option<Element>,
    #[props(default)] bottom_right: Option<Element>,
    #[props(extends=GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let layout_ui = use_space_layout_ui();
    let sidebar_visible = layout_ui.sidebar_visible;

    use_effect(move || {
        let mut sidebar_visible = sidebar_visible;
        sidebar_visible.set(false);
    });

    use_drop(move || {
        let mut sidebar_visible = sidebar_visible;
        sidebar_visible.set(true);
    });

    let base_content_class = "mx-auto flex w-full max-w-desktop flex-1 flex-col gap-4 overflow-y-auto pb-6";
    let content_class = if content_class.is_empty() {
        base_content_class.to_string()
    } else {
        format!("{base_content_class} {content_class}")
    };

    let base_bottom_class = "-mx-5 -mb-5 max-tablet:-mx-3 max-tablet:-mb-3 max-mobile:-mx-2 max-mobile:-mb-2 flex items-center justify-between gap-3 border-t border-card-border bg-card-bg px-5 py-3";
    let bottom_class = if bottom_class.is_empty() {
        base_bottom_class.to_string()
    } else {
        format!("{base_bottom_class} {bottom_class}")
    };

    rsx! {
        div { class: "flex min-h-0 w-full flex-1 flex-col gap-4",
            ..attributes,
            div { class: "{content_class}",
                {children}
            }

            div { class: "{bottom_class}",
                div {
                    if let Some(left) = bottom_left {
                        {left}
                    }
                }
                div { class: "flex items-center gap-3",
                    if let Some(right) = bottom_right {
                        {right}
                    }
                }
            }
        }
    }
}
