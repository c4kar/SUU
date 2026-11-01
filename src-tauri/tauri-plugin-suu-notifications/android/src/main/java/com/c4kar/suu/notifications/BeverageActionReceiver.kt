package com.c4kar.suu.notifications

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

/**
 * Bildirim üzerindeki "+Su (1 Su Bardağı)" ve "+Su (1 Yudum)" butonlarına basıldığında
 * uygulamayı öne açmadan (MainActivity başlatmadan) doğrudan `suu.db` veritabanına yazar,
 * bildirimi kapatır ve bir sonraki hatırlatma sayacını `şimdi + sıklık` olarak sıfırlar.
 */
class BeverageActionReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent?) {
        val portionId = intent?.getStringExtra(NotificationHelper.EXTRA_PORTION_ID) ?: "water_glass"
        SuuDatabaseHelper.logQuickWaterFromNotification(context, portionId)
        NotificationHelper.dismissNotification(context)
        ReminderScheduler.scheduleNext(context)
        SuuNotificationsPlugin.notifyWebViewIntakeUpdated()
    }
}
