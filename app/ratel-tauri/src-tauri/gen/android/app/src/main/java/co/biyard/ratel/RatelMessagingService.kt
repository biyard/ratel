package co.biyard.ratel

import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.os.Build
import androidx.core.app.NotificationCompat
import com.google.firebase.messaging.FirebaseMessagingService
import com.google.firebase.messaging.RemoteMessage

/**
 * FCM service. Backgrounded "notification" messages are shown by the system
 * automatically (and, on tap, relaunch MainActivity with the data payload as
 * intent extras). This service only needs to:
 *  - persist token refreshes (MainActivity reads the current token and registers
 *    it with the backend through the authenticated web layer), and
 *  - render a notification for messages that arrive while the app is foreground
 *    (the system does NOT auto-display those).
 */
class RatelMessagingService : FirebaseMessagingService() {
    override fun onNewToken(token: String) {
        getSharedPreferences("ratel_fcm", Context.MODE_PRIVATE)
            .edit().putString("token", token).apply()
    }

    override fun onMessageReceived(message: RemoteMessage) {
        val title = message.notification?.title ?: message.data["title"] ?: "Ratel"
        val body = message.notification?.body ?: message.data["body"] ?: ""
        val url = message.data["url"] ?: ""

        val nm = getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            nm.createNotificationChannel(
                NotificationChannel(CHANNEL_ID, "Ratel", NotificationManager.IMPORTANCE_HIGH)
            )
        }

        val launch = packageManager.getLaunchIntentForPackage(packageName)?.apply {
            if (url.isNotEmpty()) putExtra("url", url)
        }
        val pi = PendingIntent.getActivity(
            this, url.hashCode(), launch,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        val notif = NotificationCompat.Builder(this, CHANNEL_ID)
            .setSmallIcon(R.mipmap.ratel_icon)
            .setContentTitle(title)
            .setContentText(body)
            .setAutoCancel(true)
            .setContentIntent(pi)
            .build()
        nm.notify(System.currentTimeMillis().toInt(), notif)
    }

    companion object {
        const val CHANNEL_ID = "ratel_default"
    }
}
