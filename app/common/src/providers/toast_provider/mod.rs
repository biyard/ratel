mod toast_card;
use dioxus::prelude::*;
use dioxus_translate::{Language, Translate, use_language};

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
    lang: Signal<Language>,
}

impl ToastService {
    pub fn init() {
        let svc = Self {
            toasts: Signal::new(Vec::new()),
            next_id: Signal::new(0),
            lang: use_language(),
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

    pub fn error(&mut self, err: crate::Error) -> &mut Self {
        let l = (self.lang)();
        self.push(ToastLevel::Error, err.translate(&l))
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
pub fn ToastProvider() -> Element {
    let toasts = use_toast().list();

    if toasts.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "flex fixed top-4 right-4 flex-col gap-3 z-[102] max-w-[380px]",
            for toast in toasts {
                toast_card::ToastCard { key: "{toast.id}", toast }
            }
        }
    }
}
