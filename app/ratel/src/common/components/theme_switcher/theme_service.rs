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
            apply_resolved(saved);
        });
    }

    pub fn current(&self) -> Theme {
        *self.theme.read()
    }

    pub fn set(&mut self, theme: Theme) {
        self.theme.set(theme);
        save_theme(&theme.to_string());
        apply_resolved(theme);
    }
}

/// Push the selected theme into the DOM. Passes `"system"` through to the
/// JS helper, which resolves it via `matchMedia("(prefers-color-scheme:
/// dark)")` plus the `color-scheme: light dark` CSS hint in
/// `dx-components-theme.css`.
///
/// We intentionally do NOT call the Android native probe
/// (`system_is_dark`) here: invoking `SystemThemePlugin::new()` during
/// app bootstrap crashes with `ClassNotFoundException` because the
/// manganis JNI thread hasn't been attached to the app's classloader
/// yet. The native module is kept around for future use once we can
/// schedule the probe on a safer thread or configure the dx-generated
/// `MainActivity` to attach the classloader up front.
fn apply_resolved(theme: Theme) {
    apply_theme(&theme.to_string());
}

pub fn use_theme() -> ThemeService {
    use_context::<ThemeService>()
}
