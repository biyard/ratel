//! Native system-theme detection for Android.
//!
//! Android WebView does not reliably forward `prefers-color-scheme: dark` to
//! web content unless the host app uses a DayNight Android theme — dx's
//! scaffold does not. We therefore query `Configuration.uiMode` in Kotlin
//! via manganis FFI and drive the initial theme from Rust.
//!
//! On non-Android targets this module returns `None` so callers fall back
//! to the web `matchMedia` path.

// Manganis FFI signatures must mirror the Kotlin method names verbatim
// (camelCase). `-D warnings` makes non_snake_case a hard error otherwise.
#![allow(non_snake_case)]

#[cfg(target_os = "android")]
#[manganis::ffi("src/common/components/theme_switcher/android")]
extern "Kotlin" {
    pub type SystemThemePlugin;

    /// Kotlin signature: `fun isSystemDarkMode(): String`
    /// Returns `"dark"` or `"light"`. Using `String` instead of `Boolean`
    /// because manganis 0.7.5 miscompiles `-> bool` FFI returns.
    pub fn isSystemDarkMode(this: &SystemThemePlugin) -> String;
}

/// Returns `Some(true)` if the OS is in dark mode, `Some(false)` if light,
/// or `None` on platforms without a native probe (web, desktop) — callers
/// should fall back to `matchMedia` / CSS-driven detection there.
pub fn system_is_dark() -> Option<bool> {
    #[cfg(target_os = "android")]
    {
        let plugin = match SystemThemePlugin::new() {
            Ok(p) => p,
            Err(e) => {
                crate::error!("failed to create SystemThemePlugin: {e:?}");
                return None;
            }
        };
        match isSystemDarkMode(&plugin) {
            Ok(mode) => Some(mode == "dark"),
            Err(e) => {
                crate::error!("isSystemDarkMode ffi failed: {e:?}");
                None
            }
        }
    }

    #[cfg(not(target_os = "android"))]
    {
        None
    }
}
