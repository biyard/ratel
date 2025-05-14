package com.ratel.ratelMobile

import android.app.NotificationChannel
import android.app.NotificationManager
import android.os.Build
import android.os.Handler
import android.os.Looper
import android.util.Log
import android.content.Context
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import com.google.firebase.messaging.FirebaseMessagingService
import com.google.firebase.messaging.RemoteMessage
import org.json.JSONObject
import android.webkit.WebView
import android.app.PendingIntent
import android.content.Intent
import android.media.RingtoneManager
import com.ratel.ratelMobile.MainActivity

class MyFirebaseService : FirebaseMessagingService() {
    override fun onMessageReceived(remoteMessage: RemoteMessage) {
        super.onMessageReceived(remoteMessage)

        if (remoteMessage.data.isNotEmpty()) {
            Log.d(TAG, "Message data payload: ${remoteMessage.data}")
            sendNotification(
                remoteMessage.data["title"].toString(),
                remoteMessage.data["body"].toString()
            )
        } else {
            remoteMessage.notification?.let {
                sendNotification(
                    remoteMessage.notification!!.title.toString(),
                    remoteMessage.notification!!.body.toString()
                )
            }
        }
    }

    override fun onNewToken(token: String) {
        super.onNewToken(token)
        sendRegistrationToServer(token)
    }
    
    private fun sendRegistrationToServer(token: String?) {
        Log.d(TAG, "sendRegistrationTokenToServer($token)")
    }

    private fun sendNotification(title: String, body: String) {
        val intent = Intent(this, MainActivity::class.java)
        intent.addFlags(Intent.FLAG_ACTIVITY_CLEAR_TOP)
        val pendingIntent = PendingIntent.getActivity(
            this, 0, intent,
            PendingIntent.FLAG_IMMUTABLE
        )

        val channelId = "fcm_default_channel"
        val defaultSoundUri = RingtoneManager.getDefaultUri(RingtoneManager.TYPE_NOTIFICATION)
        val notificationBuilder = NotificationCompat.Builder(this, channelId)
            .setSmallIcon(R.drawable.ic_launcher_foreground)
            .setContentTitle(title)
            .setContentText(body)
            .setAutoCancel(true)
            .setSound(defaultSoundUri)
            .setContentIntent(pendingIntent)

        val notificationManager =
            getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                channelId,
                "Channel human readable title",
                NotificationManager.IMPORTANCE_DEFAULT
            )
            notificationManager.createNotificationChannel(channel)
        }

        notificationManager.notify(0, notificationBuilder.build())
    }

    companion object {
        private const val TAG = "MyFirebaseMsgService"
    }
}
