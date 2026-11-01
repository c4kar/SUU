use crate::domain::{
    clamp_goal_ml, compute_day_progress, current_tracking_date, evaluate_streak, lookup_portion,
    shift_tracking_date, CalendarDayEntry, DashboardState, IntakeLog, UserSettings,
};
use chrono::{Local, NaiveDate};
use rusqlite::{params, Connection};
use std::collections::BTreeMap;
use std::path::PathBuf;
use uuid::Uuid;

pub struct SuuDatabase {
    pub db_path: PathBuf,
}

impl SuuDatabase {
    pub fn new(db_path: PathBuf) -> Result<Self, String> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let instance = Self { db_path };
        instance.init_schema()?;
        Ok(instance)
    }

    fn connect(&self) -> Result<Connection, String> {
        let conn = Connection::open(&self.db_path).map_err(|e| e.to_string())?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA busy_timeout = 5000;",
        )
        .map_err(|e| e.to_string())?;
        Ok(conn)
    }

    fn init_schema(&self) -> Result<(), String> {
        let conn = self.connect()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS user_settings (
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
            INSERT OR IGNORE INTO user_settings (
                id, onboarding_completed, weight_kg, age_bracket, activity_level,
                daily_goal_ml, active_start_min, active_end_min, interval_min, updated_at_ms
            ) VALUES (1, 0, 65, '30_55', 'low', 2000, 540, 1320, 120, 0);

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
            CREATE INDEX IF NOT EXISTS idx_intake_tracking_date ON intake_logs(tracking_date);

            CREATE TABLE IF NOT EXISTS daily_goals (
                tracking_date TEXT PRIMARY KEY,
                goal_ml INTEGER NOT NULL
            );",
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn read_settings(&self) -> Result<UserSettings, String> {
        let conn = self.connect()?;
        conn.query_row(
            "SELECT onboarding_completed, weight_kg, age_bracket, activity_level,
                    daily_goal_ml, active_start_min, active_end_min, interval_min, next_alarm_epoch_ms
             FROM user_settings WHERE id = 1",
            [],
            |row| {
                Ok(UserSettings {
                    onboarding_completed: row.get::<_, i32>(0)? != 0,
                    weight_kg: row.get(1)?,
                    age_bracket: row.get(2)?,
                    activity_level: row.get(3)?,
                    daily_goal_ml: row.get(4)?,
                    active_start_min: row.get(5)?,
                    active_end_min: row.get(6)?,
                    interval_min: row.get(7)?,
                    next_alarm_epoch_ms: row.get(8)?,
                })
            },
        )
        .map_err(|e| e.to_string())
    }

    pub fn update_settings(&self, settings: &UserSettings) -> Result<DashboardState, String> {
        let conn = self.connect()?;
        let clamped_goal = clamp_goal_ml(settings.daily_goal_ml);
        let now_ms = Local::now().timestamp_millis();
        let today = current_tracking_date();

        conn.execute(
            "UPDATE user_settings
             SET onboarding_completed = ?1,
                 weight_kg = ?2,
                 age_bracket = ?3,
                 activity_level = ?4,
                 daily_goal_ml = ?5,
                 active_start_min = ?6,
                 active_end_min = ?7,
                 interval_min = ?8,
                 updated_at_ms = ?9
             WHERE id = 1",
            params![
                if settings.onboarding_completed { 1 } else { 0 },
                settings.weight_kg,
                settings.age_bracket,
                settings.activity_level,
                clamped_goal,
                settings.active_start_min,
                settings.active_end_min,
                settings.interval_min,
                now_ms
            ],
        )
        .map_err(|e| e.to_string())?;

        // Bugünün hedefini anında güncelle, geçmiş günleri koru
        conn.execute(
            "INSERT INTO daily_goals (tracking_date, goal_ml) VALUES (?1, ?2)
             ON CONFLICT(tracking_date) DO UPDATE SET goal_ml = excluded.goal_ml",
            params![today, clamped_goal],
        )
        .map_err(|e| e.to_string())?;

        self.get_dashboard_state(Some(&today))
    }

    pub fn log_intake(
        &self,
        portion_id: &str,
        tracking_date: Option<&str>,
    ) -> Result<DashboardState, String> {
        let spec = lookup_portion(portion_id)
            .ok_or_else(|| format!("Bilinmeyen porsiyon kimliği: {portion_id}"))?;
        let today = current_tracking_date();
        let yesterday = shift_tracking_date(&today, -1);
        let target_date = tracking_date.unwrap_or(&today);

        if target_date != today && target_date != yesterday {
            return Err("Yalnızca Bugün ve Dün için kayıt eklenebilir.".to_string());
        }

        let settings = self.read_settings()?;
        let conn = self.connect()?;
        let now_ms = Local::now().timestamp_millis();
        let id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT OR IGNORE INTO daily_goals (tracking_date, goal_ml) VALUES (?1, ?2)",
            params![target_date, settings.daily_goal_ml],
        )
        .map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO intake_logs (
                id, tracking_date, logged_at_ms, category, portion_id, gross_ml, effective_ml, source
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'app')",
            params![
                id,
                target_date,
                now_ms,
                spec.category,
                spec.id,
                spec.gross_ml,
                spec.effective_ml
            ],
        )
        .map_err(|e| e.to_string())?;

        self.get_dashboard_state(Some(target_date))
    }

    pub fn delete_intake(
        &self,
        log_id: &str,
        tracking_date: Option<&str>,
    ) -> Result<DashboardState, String> {
        let conn = self.connect()?;
        conn.execute("DELETE FROM intake_logs WHERE id = ?1", params![log_id])
            .map_err(|e| e.to_string())?;
        self.get_dashboard_state(tracking_date)
    }

    fn read_logs_for_date(&self, conn: &Connection, tracking_date: &str) -> Result<Vec<IntakeLog>, String> {
        let mut stmt = conn
            .prepare(
                "SELECT id, tracking_date, logged_at_ms, category, portion_id, gross_ml, effective_ml, source
                 FROM intake_logs
                 WHERE tracking_date = ?1
                 ORDER BY logged_at_ms DESC",
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map(params![tracking_date], |row| {
                Ok(IntakeLog {
                    id: row.get(0)?,
                    tracking_date: row.get(1)?,
                    logged_at_ms: row.get(2)?,
                    category: row.get(3)?,
                    portion_id: row.get(4)?,
                    gross_ml: row.get(5)?,
                    effective_ml: row.get(6)?,
                    source: row.get(7)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut logs = Vec::new();
        for r in rows {
            logs.push(r.map_err(|e| e.to_string())?);
        }
        Ok(logs)
    }

    fn read_goal_for_date(
        &self,
        conn: &Connection,
        tracking_date: &str,
        default_goal_ml: i32,
    ) -> i32 {
        conn.query_row(
            "SELECT goal_ml FROM daily_goals WHERE tracking_date = ?1",
            params![tracking_date],
            |r| r.get(0),
        )
        .unwrap_or(default_goal_ml)
    }

    pub fn get_dashboard_state(
        &self,
        tracking_date: Option<&str>,
    ) -> Result<DashboardState, String> {
        let settings = self.read_settings()?;
        let today_date = current_tracking_date();
        let yesterday_date = shift_tracking_date(&today_date, -1);
        let target_date = tracking_date.unwrap_or(&today_date);

        let conn = self.connect()?;
        let goal_ml = self.read_goal_for_date(&conn, target_date, settings.daily_goal_ml);
        let logs = self.read_logs_for_date(&conn, target_date)?;
        let active_day = compute_day_progress(target_date, logs, goal_ml);

        // Seri hesabı için tüm kayıtlı günleri oku
        let mut all_dates: BTreeMap<String, i32> = BTreeMap::new();
        let mut stmt = conn
            .prepare("SELECT DISTINCT tracking_date FROM intake_logs ORDER BY tracking_date ASC")
            .map_err(|e| e.to_string())?;
        let date_rows = stmt
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(|e| e.to_string())?;

        for dr in date_rows {
            let dt = dr.map_err(|e| e.to_string())?;
            let g = self.read_goal_for_date(&conn, &dt, settings.daily_goal_ml);
            let dl = self.read_logs_for_date(&conn, &dt)?;
            let prog = compute_day_progress(&dt, dl, g);
            all_dates.insert(dt, prog.pct);
        }

        let (streak_days, weekly_freeze_available) = evaluate_streak(&today_date, &all_dates);

        Ok(DashboardState {
            settings,
            today_date,
            yesterday_date,
            active_day,
            streak_days,
            weekly_freeze_available,
        })
    }

    pub fn get_calendar_history(&self, year_month: &str) -> Result<Vec<CalendarDayEntry>, String> {
        let settings = self.read_settings()?;
        let conn = self.connect()?;

        let parts: Vec<&str> = year_month.split('-').collect();
        if parts.len() != 2 {
            return Err("Geçersiz yıl-ay formatı".to_string());
        }
        let year: i32 = parts[0].parse().map_err(|_| "Geçersiz yıl")?;
        let month: u32 = parts[1].parse().map_err(|_| "Geçersiz ay")?;

        let mut entries = Vec::new();
        for day in 1..=31 {
            let Some(nd) = NaiveDate::from_ymd_opt(year, month, day) else {
                break;
            };
            let dt = nd.format("%Y-%m-%d").to_string();
            let goal_ml = self.read_goal_for_date(&conn, &dt, settings.daily_goal_ml);
            let logs = self.read_logs_for_date(&conn, &dt)?;
            let has_logs = !logs.is_empty();
            let prog = compute_day_progress(&dt, logs, goal_ml);
            let status = if !has_logs {
                "empty"
            } else if prog.pct >= 100 {
                "completed"
            } else if prog.pct >= 80 {
                "protected"
            } else {
                "freeze_used"
            };

            entries.push(CalendarDayEntry {
                tracking_date: dt,
                pct: prog.pct,
                goal_ml,
                total_effective_ml: prog.total_effective_ml,
                status: status.to_string(),
                cup_summary: prog.cup_summary,
            });
        }
        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_wal_roundtrip_and_quota() {
        let tmp_path = std::env::temp_dir().join(format!("suu_test_{}.db", Uuid::new_v4()));
        let db = SuuDatabase::new(tmp_path.clone()).expect("DB init failed");

        let s = UserSettings {
            onboarding_completed: true,
            weight_kg: 65,
            age_bracket: "30_55".to_string(),
            activity_level: "low".to_string(),
            daily_goal_ml: 2000,
            active_start_min: 540,
            active_end_min: 1320,
            interval_min: 120,
            next_alarm_epoch_ms: None,
        };
        let state = db.update_settings(&s).unwrap();
        assert!(state.settings.onboarding_completed);
        assert_eq!(state.active_day.goal_ml, 2000);

        // Log 1 Su Bardağı (200 ml) + 1 Yudum (50 ml) + 1 İnce Belli Çay (net 90 ml)
        db.log_intake("water_glass", None).unwrap();
        db.log_intake("water_sip", None).unwrap();
        let after = db.log_intake("tea_ince", None).unwrap();

        assert_eq!(after.active_day.pure_water_ml, 250);
        assert_eq!(after.active_day.counted_other_ml, 90);
        assert_eq!(after.active_day.total_effective_ml, 340);
        assert_eq!(after.active_day.logs.len(), 3);

        // Delete one log
        let first_id = after.active_day.logs[0].id.clone();
        let after_del = db.delete_intake(&first_id, None).unwrap();
        assert_eq!(after_del.active_day.logs.len(), 2);

        let _ = std::fs::remove_file(tmp_path);
    }
}
