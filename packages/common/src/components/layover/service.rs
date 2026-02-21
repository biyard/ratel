use dioxus::prelude::*;

#[derive(Clone)]
pub struct Config {
    pub id: String,
    pub title: String,
    pub content: Element,
}

#[derive(Clone, Copy)]
pub struct LayoverService {
    state: Signal<Option<Config>>,
}

impl LayoverService {
    pub fn new() -> Self {
        Self {
            state: Signal::new(None),
        }
    }

    pub fn is_open(&self) -> bool {
        self.state.read().is_some()
    }

    pub fn open(&mut self, id: Option<String>, title: String, content: Element) -> &mut Self {
        self.state.set(Some(Config {
            id: id.unwrap_or("layover-zone".to_string()),
            title,
            content,
        }));
        self
    }

    pub fn close(&mut self) {
        self.state.set(None);
    }
}

pub fn use_layover() -> LayoverService {
    use_context::<LayoverService>()
}
