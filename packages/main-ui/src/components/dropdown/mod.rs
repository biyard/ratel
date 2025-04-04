use bdk::prelude::*;
use by_components::icons::{arrows::ChevronDown, validations::Check};

#[component]
pub fn Dropdown(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    items: Vec<String>,
    #[props(default = 0)] selected: usize,
    onselect: EventHandler<String>,
) -> Element {
    let mut selected_item = use_signal(|| 0);
    let mut opened = use_signal(|| false);

    rsx! {
        div {..attributes,
            div { class: "relative w-full h-full inline-block text-left",
                div { class: "w-full h-full min-w-150 max-mobile:!min-w-100",
                    button {
                        aria_expanded: "false",
                        aria_haspopup: "true",
                        class: "inline-flex w-full h-full flex-row justify-between gap-x-1.5 rounded-md px-20 py-12 text-sm font-semibold text-white ring-1 shadow-xs ring-c-wg-70 ring-inset items-center cursor-pointer",
                        id: "menu-button",
                        r#type: "button",
                        onclick: move |_| opened.set(!opened()),
                        span { class: "text-left", {items[selected_item()].clone()} }
                        ChevronDown {
                            class: "[&>path]:stroke-white",
                            width: "15",
                            height: "15",
                        }
                    }
                }
                div {
                    aria_labelledby: "menu-button",
                    aria_orientation: "vertical",
                    visibility: if !opened() { "hidden" },
                    class: "absolute right-0 z-10 w-full mt-10 origin-top-right rounded-[10px] ring-1 ring-primary focus:outline-hidden bg-black overflow-hidden",
                    role: "menu",
                    tabindex: "-1",
                    div { class: "py-1", role: "none",
                        for (i , item) in items.into_iter().enumerate() {
                            div {
                                class: "text-sm text-gray-700 text-c-wg-50 font-semibold py-15 px-20 flex flex-row w-full justify-between hover:text-white items-center cursor-pointer",
                                color: if i == selected_item() { "white" },
                                onclick: move |_| {
                                    opened.set(false);
                                    selected_item.set(i);
                                    onselect(item.clone());
                                },
                                id: "menu-item-{i}",
                                role: "menuitem",
                                tabindex: "-1",
                                {item.clone()}
                                if i == selected_item() {
                                    Check { class: "[&>path]:stroke-white" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
