package com.c4kar.suu.notifications

import android.app.AlarmManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.os.Build
import java.time.Instant
import java.time.ZoneId
import java.time.ZonedDateTime

object ReminderScheduler {
    private const val ALARM_REQUEST_CODE = 5201

    /**
     * Bir sonraki hatırlatma zamanını hesaplar ve `setExactAndAllowWhileIdle` ile kurar:
     * - Günlük Hedef %100 tamamlandıysa o günün kalan alarmları iptal edilir, doğrudan ertesi sabah `activeStartMin` (09:00) saatine kurulur.
     * - Hedef tamamlanmadıysa `now + intervalMin` olarak kurulur; eğer bu zaman `activeEndMin` (22:00) sonrasına düşerse ertesi sabah `activeStartMin` saatine kurulur.
     */
    fun scheduleNext(context: Context): Long {
        val settings = SuuDatabaseHelper.readSettings(context)
        val snapshot = SuuDatabaseHelper.readTodaySnapshot(context)
        val nowMs = System.currentTimeMillis()
        val zone = ZoneId.systemDefault()
        val nowZdt = Instant.ofEpochMilli(nowMs).atZone(zone)

        val nextZdt: ZonedDateTime = if (snapshot.isGoalCompleted) {
            NotificationHelper.dismissNotification(context)
            nextMorningStart(nowZdt, settings.activeStartMin)
        } else {
            val candidate = nowZdt.plusMinutes(settings.intervalMin.toLong())
            val candidateMinOfDay = candidate.hour * 60 + candidate.minute
            if (candidateMinOfDay < settings.activeStartMin) {
                candidate
                    .withHour(settings.activeStartMin / 60)
                    .withMinute(settings.activeStartMin % 60)
                    .withSecond(0)
                    .withNano(0)
            } else if (candidateMinOfDay > settings.activeEndMin) {
                nextMorningStart(candidate, settings.activeStartMin)
            } else {
                candidate
            }
        }

        val triggerAtMs = nextZdt.toInstant().toEpochMilli()
        val alarmManager = context.getSystemService(Context.ALARM_SERVICE) as AlarmManager
        val intent = Intent(context, AlarmReceiver::class.java).apply {
            action = "com.c4kar.suu.ACTION_REMINDER_ALARM"
        }
        val pendingIntent = PendingIntent.getBroadcast(
            context,
            ALARM_REQUEST_CODE,
            intent,
            PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE
        )

        try {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S && !alarmManager.canScheduleExactAlarms()) {
                alarmManager.setAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, triggerAtMs, pendingIntent)
            } else {
                alarmManager.setExactAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, triggerAtMs, pendingIntent)
            }
        } catch (_: SecurityException) {
            alarmManager.setAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, triggerAtMs, pendingIntent)
        }

        SuuDatabaseHelper.saveNextAlarmEpoch(context, triggerAtMs)
        return triggerAtMs
    }

    private fun nextMorningStart(ref: ZonedDateTime, activeStartMin: Int): ZonedDateTime {
        val startHour = activeStartMin / 60
        val startMinute = activeStartMin % 60
        val todayStart = ref.withHour(startHour).withMinute(startMinute).withSecond(0).withNano(0)
        return if (ref.isBefore(todayStart)) {
            todayStart
        } else {
            todayStart.plusDays(1)
        }
    }
}
