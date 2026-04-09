use crate::common::*;
use crate::features::spaces::layout::use_space_layout_ui;

#[component]
pub fn FullActionLayover(
    #[props(default)] content_class: String,
    #[props(default)] bottom_class: String,
    #[props(default)] combo_chip: Option<Element>,
    #[props(default)] bottom_left: Option<Element>,
    #[props(default)] bottom_right: Option<Element>,
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
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

    let base_content_class = "mx-auto flex w-full max-w-desktop flex-1 flex-col gap-4 overflow-y-auto scrollbar-none pb-20 max-tablet:pb-36 bg-gradient-to-b from-background to-background/95";
    let content_class = if content_class.is_empty() {
        base_content_class.to_string()
    } else {
        format!("{base_content_class} {content_class}")
    };

    let base_bottom_class = "fixed bottom-0 left-0 right-0 z-30 flex items-center justify-between gap-3 border-t border-card-border bg-card-bg/80 px-5 py-3 backdrop-blur-md";
    let bottom_class = if bottom_class.is_empty() {
        base_bottom_class.to_string()
    } else {
        format!("{base_bottom_class} {bottom_class}")
    };

    rsx! {
        div { class: "flex flex-col flex-1 w-full min-h-0", ..attributes,
            div { class: "{content_class}", {children} }

            div { class: "{bottom_class}",
                div { class: "flex gap-3 items-center",
                    if let Some(chip) = combo_chip {
                        {chip}
                    }
                    if let Some(left) = bottom_left {
                        {left}
                    }
                }
                div { class: "flex gap-3 items-center",
                    if let Some(right) = bottom_right {
                        {right}
                    }
                }
            }
        }
    }
}
