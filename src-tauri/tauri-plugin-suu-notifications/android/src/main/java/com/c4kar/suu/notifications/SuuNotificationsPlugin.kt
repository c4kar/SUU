package com.c4kar.suu.notifications

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Build
import android.os.PowerManager
import android.provider.Settings
import android.webkit.WebView
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import java.lang.ref.WeakReference

@TauriPlugin
class SuuNotificationsPlugin(private val activity: Activity) : Plugin(activity) {

    companion object {
        @Volatile
        private var pendingQuickDrawerOpen: Boolean = false
        private var instanceRef: WeakReference<SuuNotificationsPlugin>? = null

        fun notifyWebViewIntakeUpdated() {
            instanceRef?.get()?.trigger("suu://intake-updated", JSObject())
        }
    }

    override fun load(webView: WebView) {
        super.load(webView)
        instanceRef = WeakReference(this)
        NotificationHelper.ensureChannel(activity)
        inspectIntent(activity.intent)
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        inspectIntent(intent)
        if (pendingQuickDrawerOpen) {
            trigger("suu://intake-updated", JSObject())
        }
    }

    private fun inspectIntent(intent: Intent?) {
        if (intent?.getBooleanExtra(NotificationHelper.EXTRA_OPEN_QUICK_DRAWER, false) == true) {
            pendingQuickDrawerOpen = true
            intent.removeExtra(NotificationHelper.EXTRA_OPEN_QUICK_DRAWER)
        }
    }

    @Command
    fun rescheduleNextReminder(invoke: Invoke) {
        val nextMs = ReminderScheduler.scheduleNext(activity)
        val ret = JSObject()
        ret.put("nextAlarmEpochMs", nextMs)
        invoke.resolve(ret)
    }

    @SuppressLint("BatteryLife")
    @Command
    fun requestOneUiPermissions(invoke: Invoke) {
        // 1. Android 13+ (API 33+) Bildirim İzni (POST_NOTIFICATIONS)
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            val granted = ContextCompat.checkSelfPermission(
                activity,
                Manifest.permission.POST_NOTIFICATIONS
            ) == PackageManager.PERMISSION_GRANTED
            if (!granted) {
                ActivityCompat.requestPermissions(
                    activity,
                    arrayOf(Manifest.permission.POST_NOTIFICATIONS),
                    4202
                )
            }
        }

        // 2. Samsung One UI Derin Uyku (Deep Sleep) Koruması Muafiyeti
        val pm = activity.getSystemService(Context.POWER_SERVICE) as PowerManager
        if (!pm.isIgnoringBatteryOptimizations(activity.packageName)) {
            try {
                val intent = Intent(Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS).apply {
                    data = Uri.parse("package:${activity.packageName}")
                }
                activity.startActivity(intent)
            } catch (_: Exception) {
                // Cihaz desteklemiyorsa sessizce geç
            }
        }

        ReminderScheduler.scheduleNext(activity)
        invoke.resolve(JSObject())
    }

    @Command
    fun consumeLaunchIntent(invoke: Invoke) {
        val shouldOpen = pendingQuickDrawerOpen
        pendingQuickDrawerOpen = false
        val ret = JSObject()
        ret.put("openQuickDrawer", shouldOpen)
        invoke.resolve(ret)
    }
}
