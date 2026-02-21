use dioxus::prelude::*;

#[derive(Clone)]
pub struct PopupConfig {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Element,
    pub closable: bool,
    pub backdrop_closable: bool,
    pub overflow: bool,
}

#[derive(Clone, Copy)]
pub struct PopupService {
    state: Signal<Option<PopupConfig>>,
}

impl PopupService {
    pub fn new() -> Self {
        Self {
            state: Signal::new(None),
        }
    }

    pub fn is_open(&self) -> bool {
        self.state.read().is_some()
    }

    pub fn config(&self) -> Option<PopupConfig> {
        self.state.read().clone()
    }

    pub fn open(&mut self, content: Element) -> &mut Self {
        self.state.set(Some(PopupConfig {
            id: "popup-zone-1".to_string(),
            title: None,
            description: None,
            content,
            closable: true,
            backdrop_closable: true,
            overflow: false,
        }));
        self
    }

    pub fn close(&mut self) {
        self.state.set(None);
    }

    pub fn with_id(&mut self, id: impl Into<String>) -> &mut Self {
        {
            let mut current = self.state.write();
            if let Some(ref mut cfg) = *current {
                cfg.id = id.into();
            }
        }
        self
    }

    pub fn with_title(&mut self, title: impl Into<String>) -> &mut Self {
        {
            let mut current = self.state.write();
            if let Some(ref mut cfg) = *current {
                cfg.title = Some(title.into());
            }
        }
        self
    }

    pub fn with_description(&mut self, desc: impl Into<String>) -> &mut Self {
        {
            let mut current = self.state.write();
            if let Some(ref mut cfg) = *current {
                cfg.description = Some(desc.into());
            }
        }
        self
    }

    pub fn without_close(&mut self) -> &mut Self {
        {
            let mut current = self.state.write();
            if let Some(ref mut cfg) = *current {
                cfg.closable = false;
            }
        }
        self
    }

    pub fn without_backdrop_close(&mut self) -> &mut Self {
        {
            let mut current = self.state.write();
            if let Some(ref mut cfg) = *current {
                cfg.backdrop_closable = false;
            }
        }
        self
    }

    pub fn with_overflow(&mut self, overflow: bool) -> &mut Self {
        {
            let mut current = self.state.write();
            if let Some(ref mut cfg) = *current {
                cfg.overflow = overflow;
            }
        }
        self
    }
}

pub fn use_popup() -> PopupService {
    use_context::<PopupService>()
}
