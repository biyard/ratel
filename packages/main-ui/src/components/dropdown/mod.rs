use dioxus::prelude::*;

#[component]
pub fn Dropdown(
    #[props(into)] inner_class: Option<String>,
    items: Vec<String>,
    placeholder: String,
    value: Option<String>,
    onclick: EventHandler<String>,
    #[props(default = "#2C2E42".to_string())] bg_color: String,
    #[props(default = "white".to_string())] text_color: String,
    #[props(default = "#404761".to_string())] placeholder_color: String,
    #[props(default = "#1B1D31".to_string())] hover_color: String,
) -> Element {
    let mut is_open: Signal<bool> = use_signal(|| false);
    let items = items.clone();

    rsx! {
        div {
            class: if let Some(class) = inner_class { class } 
                else { "relative w-full h-[59px] bg-[{bg_color}] rounded-[8px]" },
            input {
                class: "w-full px-[24px] py-[17.5px] text-[18px] font-bold bg-[{bg_color}] placeholder-[{placeholder_color}] leading-[24px] rounded-[8px] cursor-pointer",
                placeholder: "{placeholder}",
                value: value.unwrap_or_default(),
                readonly: true,
                onmousedown: move |_| {
                    tracing::debug!("Dropdown input clicked");
                    is_open.set(!is_open());
                },
            }
            if (is_open)() {
                div {
                    class: "absolute w-full mt-[10px] bg-[{bg_color}] \
                            rounded-[8px] shadow-lg overflow-hidden \
                            transition-all duration-200 z-10 ",
                    // Options list
                    div {
                        class: "bg-[{bg_color}] overflow-y-auto max-h-[200px]",
                        for item in items {
                            div {
                                class: "w-full h-[43px] px-[24px] py-[12px] text-left font-bold text-[15px] \
                                        leading-[22.5px] hover:bg-[{hover_color}] transition-colors",
                                onclick: move |_| {
                                    onclick(item.clone());
                                    is_open.set(false);
                                },
                                "{item}"
                            }
                        }
                    }
                }
            }
        }
        
    }
}