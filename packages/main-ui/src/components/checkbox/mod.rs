#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn Checkbox(
    #[props(default ="".to_string())] class: String,
    title: String,
    onchange: EventHandler<bool>,
) -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        div {
            class,
            onclick: move |_| {
                checked.set(!checked());
                onchange(checked());
            },
            div { class: "flex flex-row items-start justify-start gap-[6px] cursor-pointer",
                CheckboxIcon { checked: checked() }
                span { class: "w-full text-[16px] font-normal leading-[24px] text-white", "{title}" }
            }
        }
    }
}

#[component]
pub fn CheckboxIcon(checked: bool) -> Element {
    let theme_service: Theme = use_context();
    let theme = theme_service.get_data();

    rsx! {
        div { class: "w-[28px] h-[28px] flex items-center justify-center",
            svg {
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",

                path {
                    d: "M10.8333 22.5H13.1667C16.4336 22.5 18.0671 22.5 19.3149 21.8642C20.4126 21.3049 21.3049 20.4126 21.8642 19.3149C22.5 18.0671 22.5 16.4336 22.5 13.1667V10.8333C22.5 7.56636 22.5 5.93287 21.8642 4.68505C21.3049 3.58744 20.4126 2.69506 19.3149 2.13579C18.0671 1.5 16.4336 1.5 13.1667 1.5L10.8333 1.5C7.56636 1.5 5.93287 1.5 4.68505 2.13579C3.58744 2.69506 2.69506 3.58744 2.1358 4.68505C1.5 5.93287 1.5 7.56636 1.5 10.8333L1.5 13.1667C1.5 16.4336 1.5 18.0671 2.1358 19.3149C2.69506 20.4126 3.58744 21.3049 4.68505 21.8642C5.93287 22.5 7.56636 22.5 10.8333 22.5Z",
                    fill: "{theme.background}",
                }

                // Checkmark Path
                if checked {
                    path {
                        d: "M15.5 9.66667L10.8333 14.3333L8.5 12",
                        fill: "{theme.background}",
                    }
                }

                if checked {
                    path {
                        d: "M15.5 9.66667L10.8333 14.3333L8.5 12M13.1667 22.5H10.8333C7.56636 22.5 5.93287 22.5 4.68505 21.8642C3.58744 21.3049 2.69506 20.4126 2.1358 19.3149C1.5 18.0671 1.5 16.4336 1.5 13.1667L1.5 10.8333C1.5 7.56636 1.5 5.93287 2.1358 4.68505C2.69506 3.58744 3.58744 2.69506 4.68505 2.13579C5.93287 1.5 7.56636 1.5 10.8333 1.5L13.1667 1.5C16.4336 1.5 18.0671 1.5 19.3149 2.13579C20.4126 2.69506 21.3049 3.58744 21.8642 4.68505C22.5 5.93287 22.5 7.56636 22.5 10.8333V13.1667C22.5 16.4336 22.5 18.0671 21.8642 19.3149C21.3049 20.4126 20.4126 21.3049 19.3149 21.8642C18.0671 22.5 16.4336 22.5 13.1667 22.5Z",
                        stroke: "#C5BB7B",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
            }
        }
    }
}
