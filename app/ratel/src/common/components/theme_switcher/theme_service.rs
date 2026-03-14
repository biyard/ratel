use super::*;
use crate::common::*;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct ThemeService {
    theme: Signal<Theme>,
}

impl ThemeService {
    pub fn init() {
        #[cfg(not(feature = "server"))]
        let saved = load_theme().unwrap_or_default().parse().unwrap_or_default();
        #[cfg(feature = "server")]
        let saved = Theme::default();

        let svc = Self {
            theme: use_signal(move || saved),
        };
        #[cfg(not(feature = "server"))]
        apply_theme(saved.to_string().as_str());

        use_context_provider(move || svc);
    }

    pub fn current(&self) -> Theme {
        *self.theme.read()
    }

    pub fn set(&mut self, theme: Theme) {
        self.theme.set(theme);
        let theme = theme.to_string();
        save_theme(&theme);
        apply_theme(&theme);
    }
}

pub fn use_theme() -> ThemeService {
    use_context::<ThemeService>()
}
