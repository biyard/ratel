package co.biyard.ratel.theme

import android.app.Activity
import android.content.res.Configuration
import android.webkit.WebView

/**
 * Reports the Android system's current dark/light preference.
 *
 * We query `Configuration.uiMode` because Android WebView won't reliably
 * forward `prefers-color-scheme: dark` to web content unless the host app
 * uses a DayNight theme — and the dx scaffold does not. So we bypass the
 * WebView handshake and let Rust drive the initial theme.
 */
class SystemThemePlugin(private val activity: Activity) {
    fun load(webView: WebView?) { /* no-op; manganis hook */ }

    /**
     * Returns `"dark"` or `"light"`. We use a `String` instead of `Boolean`
     * because the manganis FFI codegen currently miscompiles `-> bool`
     * returns (0.7.5). `String` is the well-tested wire type.
     */
    fun isSystemDarkMode(): String {
        val mode = activity.resources.configuration.uiMode and Configuration.UI_MODE_NIGHT_MASK
        return if (mode == Configuration.UI_MODE_NIGHT_YES) "dark" else "light"
    }
}
