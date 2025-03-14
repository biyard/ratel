use bdk::prelude::*;
use by_components::icons::arrows::ChevronDown;

#[component]
pub fn Dropdown(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    items: Vec<String>,
    #[props(default = 0)] selected: usize,
    onselect: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "relative inline-block text-left",
            "x-data": "{{ open: false }}",
            div {
                button {
                    "@click": "open = !open",
                    aria_expanded: "false",
                    aria_haspopup: "true",
                    class: "inline-flex w-full justify-center gap-x-1.5 rounded-md px-20 py-12 text-sm font-semibold text-white ring-1 shadow-xs ring-c-wg-70 ring-inset items-center cursor-pointer",
                    id: "menu-button",
                    r#type: "button",
                    span { class: "w-100 text-left", {items[selected].clone()} }
                    ChevronDown {
                        class: "[&>path]:stroke-white",
                        width: "15",
                        height: "15",
                    }
                }
            }
            div {
                "x-show": "open",
                "@click.away": "open = false",
                aria_labelledby: "menu-button",
                aria_orientation: "vertical",
                class: "absolute right-0 z-10 w-full mt-2 w-56 origin-top-right rounded-md bg-white ring-1 shadow-lg ring-black/5 focus:outline-hidden",
                role: "menu",
                tabindex: "-1",
                div { class: "py-1", role: "none",
                    for (i , item) in items.into_iter().enumerate() {
                        a {
                            class: "block px-4 py-2 text-sm text-gray-700",
                            "@click": "open = false",
                            href: "#",
                            onclick: move |_| {
                                onselect(item.clone());
                            },
                            id: "menu-item-{i}",
                            role: "menuitem",
                            tabindex: "-1",
                            {item.clone()}
                        }
                    }
                }
            }
        }
    }
}
