use dioxus::prelude::*;

#[component]
pub fn Dropdown(
    #[props(into)] inner_class: Option<String>,
    items: Vec<String>,
    value: Option<String>,
    placeholder: String,
    onselect: EventHandler<String>,
    #[props(default = "#2C2E42".to_string())] bg_color: String,
    #[props(default = "#FFFFFF".to_string())] text_color: String,
    #[props(default = "#404761".to_string())] placeholder_color: String,
    #[props(default = "#1B1D31".to_string())] hover_color: String,
) -> Element {
    let mut is_open = use_signal(|| false);
    // let mut selected_index = use_signal(|| 0);
    // let len = items.len();
    rsx! {
        div {
            role: "combobox",
            "aria-expanded": "{is_open()}",
            class: if let Some(class) = inner_class { class } 
                else { "relative w-full h-[59px] bg-[{bg_color}] rounded-[8px]" },
            input {
                class: "w-full px-[24px] py-[17.5px]  bg-[{bg_color}] rounded-[8px] cursor-pointer \
                        text-[18px] font-bold placeholder-[{placeholder_color}] leading-[24px] text-[{text_color}]",
                placeholder: "{placeholder}",
                value: value.unwrap_or_default(),
                readonly: true,
                onfocus: move |_| is_open.set(true),
                onblur : move |_| is_open.set(false),
                // TODO: Implement keyboard navigation
                // onkeydown: move |e: KeyboardEvent| {
                //     match e.key() {
                //         Key::ArrowDown => {
                //             let mut index = (selected_index)();
                //             index = (index + 1) % len;
                //             selected_index.set(index);
                //         }
                //         Key::ArrowUp => {
                //             let mut index = (selected_index)();
                //             if index == 0 {
                //                 index = len - 1;
                //             } else {
                //                 index -= 1;
                //             }
                //             selected_index.set(index);
                //         }
                //         Key::Enter => {
                //             // onselect.call(items[(selected_index)()].clone());
                //             is_open.set(false);
                //         }
                //         Key::Escape => {
                //             is_open.set(false);
                //         }
                //         _ => {}
                //     }
                // },
                "aria-label": "Dropdown",
            }
            div {
                class: "absolute w-full mt-[10px] bg-[{bg_color}] \
                        rounded-[8px] shadow-lg overflow-hidden \
                        transition-all duration-200 z-10 text-[{text_color}]",
                style: if (is_open)() {
                    "max-height: 200px; opacity: 1;"
                } else {
                    "max-height: 0; opacity: 0;"
                },
                // Options list
                div {
                    class: "bg-[{bg_color}] overflow-y-auto max-h-[200px]",
                    // for (index, item) in items.iter().enumerate() {  
                    for item in items {
                        div {
                            class: "w-full h-[43px] px-[24px] py-[12px] text-left font-bold text-[15px] \
                                    leading-[22.5px] hover:bg-[{hover_color}] transition-colors",
                            onclick: move |_| {
                                tracing::debug!("Dropdown item clicked");
                                onselect.call(item.clone());
                                is_open.set(false);
                            },
                            // TODO: Implement keyboard navigation
                            // "aria-selected": "{(selected_index)() == index}",
                            // style: if (selected_index)() == index {
                            //     "background-color: {hover_color};"
                            // } else {
                            //     "background-color: {bg_color};"
                            // },
                            "{item}"
                        }
                    }
                }
            }
        }
    }
}