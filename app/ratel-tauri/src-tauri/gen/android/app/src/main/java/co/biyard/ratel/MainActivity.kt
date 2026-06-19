package co.biyard.ratel

import android.Manifest
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.provider.Settings
import android.webkit.CookieManager
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import com.google.firebase.messaging.FirebaseMessaging

class MainActivity : TauriActivity() {
  private var webView: WebView? = null
  private var pendingUrl: String? = null
  private var fcmToken: String? = null
  private var deviceId: String = "android"
  private val handler = Handler(Looper.getMainLooper())

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    createNotificationChannel()
    requestNotificationPermission()
    // Cold start from a notification tap carries the route in the intent.
    pendingUrl = intent?.getStringExtra("url")
  }

  // Tapped while the app is already running (singleTask) → page is live, so a
  // single inject reaches the SPA listeners immediately.
  override fun onNewIntent(intent: Intent) {
    super.onNewIntent(intent)
    val url = intent.getStringExtra("url")
    if (!url.isNullOrEmpty()) {
      pendingUrl = url
      injectNow()
    }
  }

  // CookieManager on Android System WebView rejects third-party cookies by
  // default; the backend (`*.ratel.foundation`) is cross-site relative to the
  // WebView origin so without this every Set-Cookie is blocked. This is also
  // where we capture the FCM token + stable device id for backend registration.
  override fun onWebViewCreate(webView: WebView) {
    super.onWebViewCreate(webView)
    this.webView = webView

    val cm = CookieManager.getInstance()
    cm.setAcceptCookie(true)
    cm.setAcceptThirdPartyCookies(webView, true)

    deviceId = Settings.Secure.getString(contentResolver, Settings.Secure.ANDROID_ID) ?: "android"
    FirebaseMessaging.getInstance().token.addOnCompleteListener { task ->
      if (task.isSuccessful) {
        fcmToken = task.result
      }
    }

    // `onWebViewCreate` fires BEFORE the page/WASM is loaded, so a single inject
    // here lands on a window that the SPA boot then replaces (token + deep link
    // lost; the web listeners aren't attached yet either). Re-inject on a few
    // delays so at least one lands after the SPA is ready and its listeners are
    // up. Re-firing is idempotent (token upsert; same-route nav).
    scheduleInjects()
  }

  private fun scheduleInjects() {
    for (delay in longArrayOf(800, 2000, 4000, 7000)) {
      handler.postDelayed({ injectNow() }, delay)
    }
  }

  private fun injectNow() {
    val wv = webView ?: return
    fcmToken?.let { token ->
      wv.evaluateJavascript(
        "window.__RATEL_FCM__={token:'$token',deviceId:'$deviceId',platform:'android'};" +
          "window.dispatchEvent(new Event('ratel-fcm-ready'));",
        null
      )
    }
    pendingUrl?.let { url ->
      wv.evaluateJavascript(deepLinkJs(url), null)
    }
  }

  private fun deepLinkJs(url: String): String =
    "window.__RATEL_PENDING_URL__='$url';" +
      "window.dispatchEvent(new CustomEvent('ratel-deeplink',{detail:'$url'}));"

  private fun createNotificationChannel() {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
      val nm = getSystemService(NotificationManager::class.java)
      nm.createNotificationChannel(
        NotificationChannel(
          RatelMessagingService.CHANNEL_ID, "Ratel", NotificationManager.IMPORTANCE_HIGH
        )
      )
    }
  }

  private fun requestNotificationPermission() {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU &&
      ContextCompat.checkSelfPermission(this, Manifest.permission.POST_NOTIFICATIONS) !=
      PackageManager.PERMISSION_GRANTED
    ) {
      ActivityCompat.requestPermissions(this, arrayOf(Manifest.permission.POST_NOTIFICATIONS), 1001)
    }
  }
}
