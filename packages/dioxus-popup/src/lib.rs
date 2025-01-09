#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct PopupService {
    pub id: Signal<Option<String>>,
    pub title: Signal<Option<String>>,
    pub data: Signal<Option<Element>>,
    pub close: Signal<bool>,
}

impl PopupService {
    pub fn init() {
        let srv = Self {
            data: Signal::new(None),
            id: Signal::new(None),
            title: Signal::new(None),
            close: Signal::new(true),
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

    pub fn without_close(&mut self) -> &mut Self {
        (self.close).set(false);

        self
    }

    pub fn close(&mut self) {
        (self.data).set(None);
        (self.id).set(None);
        (self.title).set(None);
        (self.close).set(true);
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
    let mut hover_close = use_signal(|| false);

    rsx! {
        div {
            class: format!(
                "{}",
                match popup.is_opened() {
                    true => {
                        "fixed top-0 left-0 w-screen h-screen bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-[4px] bg-black/25 z-[101]"
                    }
                    false => "hidden",
                },
            ),
            onclick: move |_| {
                popup.close();
            },
            if popup.is_opened() {
                div {
                    class: "relative bg-[#424563] rounded-[12px] border-[#292B3C] border-[1px] p-[25px] min-w-[350px]",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    if (popup.close)() {
                        div {
                            class: format!(
                                "absolute top-[25px] right-[25px] rounded-[4px] cursor-pointer {}",
                                if hover_close() { "bg-[{theme.background}]" } else { "" },
                            ),
                            onclick: move |_| {
                                popup.close();
                            },
                            onmouseenter: move |_| {
                                hover_close.set(true);
                            },
                            onmouseleave: move |_| {
                                hover_close.set(false);
                            },
                            Close { color: if hover_close() { "#74789E" } else { "white" } }
                        }
                    }
                    div {
                        id: popup.get_id(),
                        class: "flex flex-col items-center justify-center gap-[25px]",
                        match popup.get_title() {
                            Some(title) => {
                                rsx! {
                                    div { class: "text-[20px] font-bold text-white", "{title}" }
                                }
                            }
                            None => rsx! {},
                        }
                        {popup.render()}
                    }
                }
            }
        }
    }
}

#[component]
pub fn Close(#[props(default = "white".to_string())] color: String) -> Element {
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M16.9498 7.05029L7.05029 16.9498M7.05029 7.05029L16.9498 16.9498",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}
