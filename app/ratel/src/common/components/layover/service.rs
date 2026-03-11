use crate::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum LayoverSize {
    #[default]
    #[strum(serialize = "w-full")]
    Full,
    #[strum(serialize = "w-[50%] !bg-[#1A1A1A] max-tablet:w-full")]
    Half,
    #[strum(serialize = "max-w-[800px] !bg-[#1A1A1A] max-tablet:!max-w-full")]
    Medium,
    #[strum(serialize = "w-[337px] max-tablet:w-full")]
    Small,
    Fit,
}

#[derive(Clone)]
pub struct Config {
    pub id: String,
    pub title: String,
    pub content: Element,
    pub container_class: Option<LayoverSize>,
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

    pub fn open(&mut self, id: String, title: String, content: Element) -> &mut Self {
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
                container_class: None,
            }));
        }
        self
    }

    pub fn set_size(&mut self, size: LayoverSize) -> &mut Self {
        self.state.with_mut(|state| {
            if let Some(config) = state.as_mut() {
                config.container_class = Some(size);
            }
        });

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
