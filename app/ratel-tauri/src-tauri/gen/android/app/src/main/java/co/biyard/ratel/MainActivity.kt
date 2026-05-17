package co.biyard.ratel

import android.os.Bundle
import android.webkit.CookieManager
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
  }

  // CookieManager on Android System WebView rejects third-party cookies
  // by default. The Tauri smoke test backend (`http://localhost:8080`)
  // is third-party relative to the WebView origin
  // (`http://tauri.localhost`), so without this hook every Set-Cookie
  // from the backend gets blocked with
  // `blockedReasons: ["UserPreferences"]` (confirmed via CDP
  // Network.responseReceivedExtraInfo). Production also benefits: the
  // dev backend at `*.ratel.foundation` is likewise cross-site relative
  // to `tauri.localhost`.
  override fun onWebViewCreate(webView: WebView) {
    super.onWebViewCreate(webView)
    val cm = CookieManager.getInstance()
    cm.setAcceptCookie(true)
    cm.setAcceptThirdPartyCookies(webView, true)
  }
}
