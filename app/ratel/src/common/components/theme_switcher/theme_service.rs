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

        #[allow(unused_mut)]
        let mut svc = Self {
            theme: use_signal(move || saved),
        };
        #[cfg(not(feature = "server"))]
        apply_theme(saved.to_string().as_str());

        // Re-sync the theme signal from localStorage after hydration to fix
        // SSR/client mismatch where the server defaults to System theme but the
        // user previously selected a different theme (e.g., Light).
        #[cfg(not(feature = "server"))]
        use_effect(move || {
            let stored: Theme = load_theme()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default();
            if svc.current() != stored {
                svc.theme.set(stored);
                apply_theme(stored.to_string().as_str());
            }
        });

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
