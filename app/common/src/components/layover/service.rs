use dioxus::prelude::*;

#[derive(Clone)]
pub struct Config {
    pub id: String,
    pub title: String,
    pub content: Element,
    pub container_class: Option<String>,
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

    pub fn open(
        &mut self,
        id: String,
        title: String,
        content: Element,
        container_class: Option<String>,
    ) -> &mut Self {
        let mut need_update = true;
        if let Some(config) = self.state.read().as_ref() {
            if config.id == id {
                need_update = false;
            }
        }
        if need_update {
            self.state.set(Some(Config {
                id,
                title,
                content,
                container_class,
            }));
        }
        self
    }

    pub fn close(&mut self) {
        self.state.set(None);
    }

    pub fn state(&self) -> Option<Config> {
        self.state.read().clone()
    }
}

pub fn use_layover() -> LayoverService {
    use_context::<LayoverService>()
}
