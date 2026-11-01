package com.c4kar.suu.notifications

import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.os.Build
import androidx.core.app.NotificationCompat

object NotificationHelper {
    const val CHANNEL_ID = "suu_hydration_reminders_high"
    const val NOTIFICATION_ID = 4201
    const val EXTRA_PORTION_ID = "extra_portion_id"
    const val EXTRA_OPEN_QUICK_DRAWER = "extra_open_quick_drawer"

    fun ensureChannel(context: Context) {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val nm = context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
            val channel = NotificationChannel(
                CHANNEL_ID,
                "SUU Su Hatırlatmaları",
                NotificationManager.IMPORTANCE_HIGH
            ).apply {
                description = "Gün içi tek dokunuşla su ve içecek kaydı bildirimleri"
                enableVibration(true)
            }
            nm.createNotificationChannel(channel)
        }
    }

    fun showReminderNotification(context: Context, snapshot: DaySnapshot) {
        if (snapshot.isGoalCompleted) {
            dismissNotification(context)
            return
        }
        ensureChannel(context)

        val launchIntent = context.packageManager
            .getLaunchIntentForPackage(context.packageName)
            ?.apply {
                flags = Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TOP
            } ?: Intent()

        val bodyPendingIntent = PendingIntent.getActivity(
            context,
            100,
            launchIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        // 1. Buton: +Su (1 Su Bardağı) -> Arka planda sessizce kaydeder (uygulamayı açmaz)
        val glassIntent = Intent(context, BeverageActionReceiver::class.java).apply {
            action = "com.c4kar.suu.ACTION_LOG_WATER_GLASS"
            putExtra(EXTRA_PORTION_ID, "water_glass")
        }
        val glassPending = PendingIntent.getBroadcast(
            context,
            101,
            glassIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        val glassAction = NotificationCompat.Action.Builder(
            0,
            "+Su (1 Su Bardağı)",
            glassPending
        ).setShowsUserInterface(false).build()

        // 2. Buton: +Su (1 Yudum) -> Arka planda sessizce kaydeder (uygulamayı açmaz)
        val sipIntent = Intent(context, BeverageActionReceiver::class.java).apply {
            action = "com.c4kar.suu.ACTION_LOG_WATER_SIP"
            putExtra(EXTRA_PORTION_ID, "water_sip")
        }
        val sipPending = PendingIntent.getBroadcast(
            context,
            102,
            sipIntent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )
        val sipAction = NotificationCompat.Action.Builder(
            0,
            "+Su (1 Yudum)",
            sipPending
        ).setShowsUserInterface(false).build()

        val title: String
        val body: String
        if (snapshot.isOtherCapReached) {
            title = "Çay/kahve hakkın doldu! Saf su zamanı 💧"
            body = "Hedefin için kalan ${snapshot.remainingGlasses} su bardağını saf su ile tamamlamalısın."
        } else {
            title = "Bir bardak suya ne dersin? 💧"
            body = "Bugünkü hedefinin tamamlanmasına ${snapshot.remainingGlasses} su bardağı kaldı."
        }

        val builder = NotificationCompat.Builder(context, CHANNEL_ID)
            .setSmallIcon(android.R.drawable.ic_dialog_info)
            .setContentTitle(title)
            .setContentText(body)
            .setStyle(NotificationCompat.BigTextStyle().bigText(body))
            .setPriority(NotificationCompat.PRIORITY_HIGH)
            .setCategory(NotificationCompat.CATEGORY_REMINDER)
            .setAutoCancel(true)
            .setContentIntent(bodyPendingIntent)
            .addAction(glassAction)
            .addAction(sipAction)

        // 3. Buton: "Diğer" -> %30 diğer içecek kotası dolmamışken gösterilir; Hızlı Seçim Çekmecesini açar
        if (!snapshot.isOtherCapReached) {
            val drawerIntent = Intent(launchIntent).apply {
                putExtra(EXTRA_OPEN_QUICK_DRAWER, true)
            }
            val drawerPending = PendingIntent.getActivity(
                context,
                103,
                drawerIntent,
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
            )
            val otherAction = NotificationCompat.Action.Builder(
                0,
                "Diğer",
                drawerPending
            ).setShowsUserInterface(true).build()
            builder.addAction(otherAction)
        }

        val nm = context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        nm.notify(NOTIFICATION_ID, builder.build())
    }

    fun dismissNotification(context: Context) {
        val nm = context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        nm.cancel(NOTIFICATION_ID)
    }
}
