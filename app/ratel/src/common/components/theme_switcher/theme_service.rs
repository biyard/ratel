use super::*;
use crate::common::*;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct ThemeService {
    theme: Signal<Theme>,
}

impl ThemeService {
    pub fn init() {
        let theme = use_signal(Theme::default);
        let svc = Self { theme };

        use_context_provider(move || svc);

        // `load_theme` is async because on mobile it round-trips through
        // the WebView via `document::eval`. We kick it off from an effect
        // so the first render uses the default theme, then we update the
        // signal (and re-apply the attribute) once storage has answered.
        let mut theme = theme;
        use_future(move || async move {
            let saved = load_theme()
                .await
                .and_then(|s| s.parse::<Theme>().ok())
                .unwrap_or_default();
            theme.set(saved);
            apply_theme(&saved.to_string());
        });
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
