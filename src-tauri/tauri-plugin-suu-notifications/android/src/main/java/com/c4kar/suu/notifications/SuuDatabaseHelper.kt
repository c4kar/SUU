package com.c4kar.suu.notifications

import android.content.Context
import android.database.sqlite.SQLiteDatabase
import java.io.File
import java.time.Instant
import java.time.ZoneId
import java.time.format.DateTimeFormatter
import java.util.UUID
import kotlin.math.ceil
import kotlin.math.max
import kotlin.math.min
import kotlin.math.roundToInt

data class UserSettingsRow(
    val dailyGoalMl: Int,
    val activeStartMin: Int,
    val activeEndMin: Int,
    val intervalMin: Int
)

data class DaySnapshot(
    val trackingDate: String,
    val goalMl: Int,
    val pureWaterMl: Int,
    val countedOtherMl: Int,
    val totalEffectiveMl: Int,
    val remainingGlasses: Int,
    val isGoalCompleted: Boolean,
    val isOtherCapReached: Boolean
)

object SuuDatabaseHelper {
    private const val DB_NAME = "suu.db"
    private val DATE_FMT = DateTimeFormatter.ofPattern("yyyy-MM-dd")

    /**
     * 04:00 uyku odaklı gün dönümü:
     * Yerel saat 00:00-03:59 arasındayken yapılan kayıtlar bir önceki takvim gününe yazılır.
     */
    fun currentTrackingDate(epochMs: Long = System.currentTimeMillis()): String {
        val shifted = Instant.ofEpochMilli(epochMs - 4L * 3600_000L)
            .atZone(ZoneId.systemDefault())
            .toLocalDate()
        return shifted.format(DATE_FMT)
    }

    private fun openDb(context: Context): SQLiteDatabase {
        val dbFile = File(context.filesDir, DB_NAME)
        dbFile.parentFile?.mkdirs()
        val db = SQLiteDatabase.openDatabase(
            dbFile.absolutePath,
            null,
            SQLiteDatabase.OPEN_READWRITE or
                SQLiteDatabase.CREATE_IF_NECESSARY or
                SQLiteDatabase.ENABLE_WRITE_AHEAD_LOGGING
        )
        db.execSQL("PRAGMA busy_timeout = 5000;")
        ensureSchema(db)
        return db
    }

    private fun ensureSchema(db: SQLiteDatabase) {
        db.execSQL(
            """
            CREATE TABLE IF NOT EXISTS user_settings (
              id INTEGER PRIMARY KEY CHECK (id = 1),
              onboarding_completed INTEGER NOT NULL DEFAULT 0,
              weight_kg INTEGER NOT NULL DEFAULT 65,
              age_bracket TEXT NOT NULL DEFAULT '30_55',
              activity_level TEXT NOT NULL DEFAULT 'low',
              daily_goal_ml INTEGER NOT NULL DEFAULT 2000,
              active_start_min INTEGER NOT NULL DEFAULT 540,
              active_end_min INTEGER NOT NULL DEFAULT 1320,
              interval_min INTEGER NOT NULL DEFAULT 120,
              next_alarm_epoch_ms INTEGER,
              updated_at_ms INTEGER NOT NULL DEFAULT 0
            );
            """.trimIndent()
        )
        db.execSQL(
            """
            INSERT OR IGNORE INTO user_settings (
              id, onboarding_completed, weight_kg, age_bracket, activity_level,
              daily_goal_ml, active_start_min, active_end_min, interval_min, updated_at_ms
            ) VALUES (1, 0, 65, '30_55', 'low', 2000, 540, 1320, 120, 0);
            """.trimIndent()
        )
        db.execSQL(
            """
            CREATE TABLE IF NOT EXISTS intake_logs (
              id TEXT PRIMARY KEY,
              tracking_date TEXT NOT NULL,
              logged_at_ms INTEGER NOT NULL,
              category TEXT NOT NULL,
              portion_id TEXT NOT NULL,
              gross_ml INTEGER NOT NULL,
              effective_ml INTEGER NOT NULL,
              source TEXT NOT NULL DEFAULT 'app'
            );
            """.trimIndent()
        )
        db.execSQL("CREATE INDEX IF NOT EXISTS idx_intake_tracking_date ON intake_logs(tracking_date);")
        db.execSQL(
            """
            CREATE TABLE IF NOT EXISTS daily_goals (
              tracking_date TEXT PRIMARY KEY,
              goal_ml INTEGER NOT NULL
            );
            """.trimIndent()
        )
    }

    fun readSettings(context: Context): UserSettingsRow {
        val db = openDb(context)
        db.use {
            it.rawQuery(
                "SELECT daily_goal_ml, active_start_min, active_end_min, interval_min FROM user_settings WHERE id = 1",
                null
            ).use { c ->
                if (c.moveToFirst()) {
                    return UserSettingsRow(
                        dailyGoalMl = c.getInt(0),
                        activeStartMin = c.getInt(1),
                        activeEndMin = c.getInt(2),
                        intervalMin = c.getInt(3)
                    )
                }
            }
        }
        return UserSettingsRow(2000, 540, 1320, 120)
    }

    fun logQuickWaterFromNotification(context: Context, portionId: String): DaySnapshot {
        val (grossMl, effectiveMl) = when (portionId) {
            "water_sip" -> 50 to 50
            else -> 200 to 200 // default "water_glass"
        }
        val now = System.currentTimeMillis()
        val trackingDate = currentTrackingDate(now)
        val settings = readSettings(context)

        val db = openDb(context)
        db.use {
            it.beginTransaction()
            try {
                it.execSQL(
                    "INSERT OR IGNORE INTO daily_goals (tracking_date, goal_ml) VALUES (?, ?)",
                    arrayOf<Any>(trackingDate, settings.dailyGoalMl)
                )
                it.execSQL(
                    """
                    INSERT INTO intake_logs (
                      id, tracking_date, logged_at_ms, category, portion_id, gross_ml, effective_ml, source
                    ) VALUES (?, ?, ?, 'water', ?, ?, ?, 'notification')
                    """.trimIndent(),
                    arrayOf<Any>(UUID.randomUUID().toString(), trackingDate, now, portionId, grossMl, effectiveMl)
                )
                it.setTransactionSuccessful()
            } finally {
                it.endTransaction()
            }
        }
        return readTodaySnapshot(context, trackingDate)
    }

    fun readTodaySnapshot(context: Context, trackingDate: String = currentTrackingDate()): DaySnapshot {
        val settings = readSettings(context)
        var pureWaterMl = 0
        var rawOtherEffectiveMl = 0

        val db = openDb(context)
        db.use {
            it.rawQuery(
                "SELECT category, SUM(effective_ml) FROM intake_logs WHERE tracking_date = ? GROUP BY category",
                arrayOf(trackingDate)
            ).use { c ->
                while (c.moveToNext()) {
                    val cat = c.getString(0)
                    val sum = c.getInt(1)
                    if (cat == "water") pureWaterMl += sum else rawOtherEffectiveMl += sum
                }
            }
        }

        val goalMl = settings.dailyGoalMl
        val otherCapMl = (goalMl * 0.30).roundToInt()
        val countedOtherMl = min(rawOtherEffectiveMl, otherCapMl)
        val totalEffectiveMl = pureWaterMl + countedOtherMl
        val remainingMl = max(0, goalMl - totalEffectiveMl)
        val remainingGlasses = ceil(remainingMl / 200.0).toInt()

        return DaySnapshot(
            trackingDate = trackingDate,
            goalMl = goalMl,
            pureWaterMl = pureWaterMl,
            countedOtherMl = countedOtherMl,
            totalEffectiveMl = totalEffectiveMl,
            remainingGlasses = remainingGlasses,
            isGoalCompleted = totalEffectiveMl >= goalMl,
            isOtherCapReached = rawOtherEffectiveMl >= otherCapMl
        )
    }

    fun saveNextAlarmEpoch(context: Context, nextEpochMs: Long) {
        val db = openDb(context)
        db.use {
            it.execSQL(
                "UPDATE user_settings SET next_alarm_epoch_ms = ?, updated_at_ms = ? WHERE id = 1",
                arrayOf<Any>(nextEpochMs, System.currentTimeMillis())
            )
        }
    }
}
