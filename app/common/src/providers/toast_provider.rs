use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastItem {
    pub id: u64,
    pub level: ToastLevel,
    pub message: String,
    pub link: Option<String>,
    pub dismissing: bool,
}

#[derive(Clone, Copy)]
pub struct ToastService {
    toasts: Signal<Vec<ToastItem>>,
    next_id: Signal<u64>,
}

impl ToastService {
    pub fn init() {
        let svc = Self {
            toasts: Signal::new(Vec::new()),
            next_id: Signal::new(0),
        };

        use_context_provider(move || svc);
    }

    fn push(&mut self, level: ToastLevel, msg: impl Into<String>) -> &mut Self {
        let id = *self.next_id.read();
        self.next_id.set(id + 1);
        self.toasts.write().push(ToastItem {
            id,
            level,
            message: msg.into(),
            link: None,
            dismissing: false,
        });
        self
    }

    pub fn info(&mut self, msg: impl Into<String>) -> &mut Self {
        self.push(ToastLevel::Info, msg)
    }

    pub fn warn(&mut self, msg: impl Into<String>) -> &mut Self {
        self.push(ToastLevel::Warn, msg)
    }

    pub fn error(&mut self, msg: impl Into<String>) -> &mut Self {
        self.push(ToastLevel::Error, msg)
    }

    pub fn with_link(&mut self, url: impl Into<String>) -> &mut Self {
        {
            let mut toasts = self.toasts.write();
            if let Some(last) = toasts.last_mut() {
                last.link = Some(url.into());
            }
        }
        self
    }

    pub fn dismiss(&mut self, id: u64) {
        let mut toasts = self.toasts.write();
        if let Some(toast) = toasts.iter_mut().find(|t| t.id == id) {
            toast.dismissing = true;
        }
    }

    pub fn remove(&mut self, id: u64) {
        self.toasts.write().retain(|t| t.id != id);
    }

    pub fn list(&self) -> Vec<ToastItem> {
        self.toasts.read().clone()
    }
}

pub fn use_toast() -> ToastService {
    use_context::<ToastService>()
}

#[component]
pub fn ToastTestButton() -> Element {
    let mut toast = use_toast();
    let mut count = use_signal(|| 0u32);

    rsx! {
        div { class: "flex fixed right-4 bottom-4 flex-col gap-2 z-[103]",
            button {
                class: "py-2 px-4 text-sm font-medium text-white bg-blue-600 rounded-lg shadow-lg transition-all hover:bg-blue-700 active:scale-95",
                onclick: move |_| {
                    let n = *count.read();
                    count.set(n + 1);
                    toast.info(format!("Info toast #{n}"));
                },
                "Info"
            }
            button {
                class: "py-2 px-4 text-sm font-medium text-white bg-yellow-600 rounded-lg shadow-lg transition-all hover:bg-yellow-700 active:scale-95",
                onclick: move |_| {
                    let n = *count.read();
                    count.set(n + 1);
                    toast.warn(format!("Warning toast #{n}"));
                },
                "Warn"
            }
            button {
                class: "py-2 px-4 text-sm font-medium text-white bg-red-600 rounded-lg shadow-lg transition-all hover:bg-red-700 active:scale-95",
                onclick: move |_| {
                    let n = *count.read();
                    count.set(n + 1);
                    toast.error(format!("Error toast #{n}"));
                },
                "Error"
            }
            button {
                class: "py-2 px-4 text-sm font-medium text-white bg-purple-600 rounded-lg shadow-lg transition-all hover:bg-purple-700 active:scale-95",
                onclick: move |_| {
                    let n = *count.read();
                    count.set(n + 1);
                    toast.info(format!("Toast with link #{n}")).with_link("https://example.com");
                },
                "With Link"
            }
        }
    }
}

#[component]
pub fn ToastZone() -> Element {
    let toasts = use_toast().list();

    if toasts.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "flex fixed top-4 right-4 flex-col gap-3 z-[102] max-w-[380px]",
            for toast in toasts {
                ToastCard { key: "{toast.id}", toast }
            }
        }
    }
}

#[component]
fn ToastCard(toast: ToastItem) -> Element {
    let mut toast_svc = use_toast();
    let id = toast.id;
    let dismissing = toast.dismissing;
    let mut drag_start_x: Signal<Option<f64>> = use_signal(|| None);
    let mut drag_offset: Signal<f64> = use_signal(|| 0.0);
    let mut mounted = use_signal(|| false);

    use_effect(move || {
        mounted.set(true);
    });

    // Auto-dismiss after 5 seconds
    let _auto_dismiss = use_future(move || {
        let mut svc = toast_svc;
        async move {
            #[cfg(feature = "web")]
            {
                gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
                svc.dismiss(id);
                gloo_timers::future::sleep(std::time::Duration::from_millis(300)).await;
                svc.remove(id);
            }
            #[cfg(feature = "server")]
            {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                svc.dismiss(id);
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                svc.remove(id);
            }
        }
    });

    let border_color = match toast.level {
        ToastLevel::Info => "border-l-blue-500",
        ToastLevel::Warn => "border-l-yellow-500",
        ToastLevel::Error => "border-l-red-500",
    };

    let slide_class = if dismissing {
        "translate-x-full opacity-0"
    } else if *mounted.read() {
        "translate-x-0 opacity-100"
    } else {
        "translate-x-full opacity-0"
    };

    let offset = *drag_offset.read();
    let transform_style = if offset < 0.0 {
        format!("transform: translateX({}px);", offset)
    } else {
        String::new()
    };

    let link = toast.link.clone();
    let has_link = link.is_some();

    rsx! {
        div {
            class: "flex gap-3 items-center p-4 rounded-lg border border-l-4 shadow-lg transition-all duration-300 cursor-pointer bg-[#1a1a2e] border-[#2a2a3e] {border_color} {slide_class}",
            style: "{transform_style}",
            onmousedown: move |e: MouseEvent| {
                drag_start_x.set(Some(e.client_coordinates().x));
            },
            onmousemove: move |e: MouseEvent| {
                if let Some(start) = *drag_start_x.read() {
                    let current = e.client_coordinates().x;
                    let diff = current - start;
                    if diff < 0.0 {
                        drag_offset.set(diff);
                    }
                }
            },
            onmouseup: move |_| {
                let off = *drag_offset.read();
                if off < -80.0 {
                    toast_svc.dismiss(id);
                    spawn(async move {
                        #[cfg(feature = "web")]
                        gloo_timers::future::sleep(std::time::Duration::from_millis(300)).await;
                        #[cfg(feature = "server")]
                        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                        toast_svc.remove(id);
                    });
                } else {
                    drag_offset.set(0.0);
                }
                drag_start_x.set(None);
            },
            onclick: move |_| {
                #[cfg(not(feature = "server"))]
                if let Some(ref url) = link {
                    let _ = web_sys::window().and_then(|w| w.open_with_url(url).ok());
                }
            },

            span { class: "flex-1 text-sm text-[#e0e0e0] select-none", "{toast.message}" }
            if has_link {
                span { class: "text-xs text-blue-400", "↗" }
            }
        }
    }
}
