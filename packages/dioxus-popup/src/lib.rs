#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct PopupService {
    pub id: Signal<Option<String>>,
    pub title: Signal<Option<String>>,
    pub data: Signal<Option<Element>>,
}

impl PopupService {
    pub fn init() {
        let srv = Self {
            data: Signal::new(None),
            id: Signal::new(None),
            title: Signal::new(None),
        };
        use_context_provider(|| srv);
    }

    pub fn render(&self) -> Element {
        (self.data)().clone().unwrap_or(default())
    }

    pub fn is_opened(&self) -> bool {
        (self.data)().is_some()
    }

    pub fn get_id(&self) -> String {
        (self.id)().clone().unwrap_or("popup-zone".to_string())
    }

    pub fn get_title(&self) -> Option<String> {
        (self.title)().clone()
    }

    pub fn open(&mut self, popup: Element) -> &mut Self {
        (self.data).set(Some(popup));

        self
    }

    pub fn with_id(&mut self, id: &str) -> &mut Self {
        (self.id).set(Some(id.to_string()));

        self
    }

    pub fn with_title(&mut self, title: &str) -> &mut Self {
        (self.title).set(Some(title.to_string()));

        self
    }

    pub fn close(&mut self) {
        (self.data).set(None);
        (self.id).set(None);
        (self.title).set(None);
    }

    pub fn use_popup_service() -> PopupService {
        use_context()
    }
}

#[component]
pub fn default() -> Element {
    rsx! {}
}

#[component]
pub fn PopupZone() -> Element {
    let mut popup: PopupService = use_context();

    rsx! {
        div {
            class: format!("{}", match popup.is_opened() {
                true => "fixed top-0 left-0 w-screen h-screen bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-[10px] bg-[#21344C]/30 z-[101]",
                false => "hidden"
            }),
            onclick: move |_| {
                popup.close();
            },
            if popup.is_opened() {
                div {
                    class: "relative bg-white rounded-lg p-[30px] min-w-[350px]",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    div {
                        class: "absolute top-0 right-0 m-4 cursor-pointer",
                        onclick: move |_| {
                            popup.close();
                        },
                        CancelIcon {}
                    }
                    {popup.render()}
                }
            }
        }

    }
}

#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    #[props(default = "black".to_string())]
    stroke: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24px".to_string())]
    width: String,
    #[props(default = "24px".to_string())]
    height: String,
    class: Option<String>,
}

pub fn CancelIcon(props: IconProps) -> Element {
    rsx! {
        svg {
            width: "{props.width}",
            height: "{props.height}",
            "stroke-linecap": "square",
            "stroke-width": "1",
            "viewBox": "0 0 24 24",
            fill: "{props.fill}",
            "xmlns": "http://www.w3.org/2000/svg",
            "stroke-linejoin": "miter",
            path {
                "d": "M4.92893219,19.0710678 C1.02368927,15.1658249 1.02368927,8.83417511 4.92893219,4.92893219 C8.83417511,1.02368927 15.1658249,1.02368927 19.0710678,4.92893219 C22.9763107,8.83417511 22.9763107,15.1658249 19.0710678,19.0710678 C15.1658249,22.9763107 8.83417511,22.9763107 4.92893219,19.0710678 Z",
                stroke: "black"
            }
            path {
                "d": "M15.5355339 15.5355339L8.46446609 8.46446609M15.5355339 8.46446609L8.46446609 15.5355339",
                stroke: props.stroke
            }
        }
    }
}
