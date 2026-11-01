package com.c4kar.suu.notifications

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

class AlarmReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent?) {
        val snapshot = SuuDatabaseHelper.readTodaySnapshot(context)
        if (!snapshot.isGoalCompleted) {
            NotificationHelper.showReminderNotification(context, snapshot)
        }
        ReminderScheduler.scheduleNext(context)
    }
}
