pub mod db;
pub mod domain;

use db::SuuDatabase;
use domain::{CalendarDayEntry, DashboardState, UserSettings};
use serde::Deserialize;
use tauri::{Manager, State};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsPayload {
    pub onboarding_completed: bool,
    pub weight_kg: i32,
    pub age_bracket: String,
    pub activity_level: String,
    pub daily_goal_ml: i32,
    pub active_start_min: i32,
    pub active_end_min: i32,
    pub interval_min: i32,
}

#[tauri::command]
fn get_dashboard_state(
    db: State<'_, SuuDatabase>,
    tracking_date: Option<String>,
) -> Result<DashboardState, String> {
    db.get_dashboard_state(tracking_date.as_deref())
}

#[tauri::command]
fn log_intake(
    db: State<'_, SuuDatabase>,
    portion_id: String,
    tracking_date: Option<String>,
) -> Result<DashboardState, String> {
    db.log_intake(&portion_id, tracking_date.as_deref())
}

#[tauri::command]
fn delete_intake(
    db: State<'_, SuuDatabase>,
    log_id: String,
    tracking_date: Option<String>,
) -> Result<DashboardState, String> {
    db.delete_intake(&log_id, tracking_date.as_deref())
}

#[tauri::command]
fn update_settings(
    db: State<'_, SuuDatabase>,
    payload: UpdateSettingsPayload,
) -> Result<DashboardState, String> {
    let settings = UserSettings {
        onboarding_completed: payload.onboarding_completed,
        weight_kg: payload.weight_kg,
        age_bracket: payload.age_bracket,
        activity_level: payload.activity_level,
        daily_goal_ml: payload.daily_goal_ml,
        active_start_min: payload.active_start_min,
        active_end_min: payload.active_end_min,
        interval_min: payload.interval_min,
        next_alarm_epoch_ms: None,
    };
    db.update_settings(&settings)
}

#[tauri::command]
fn get_calendar_history(
    db: State<'_, SuuDatabase>,
    year_month: String,
) -> Result<Vec<CalendarDayEntry>, String> {
    db.get_calendar_history(&year_month)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_suu_notifications::init())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("Uygulama veri dizini alınamadı");
            let db_path = data_dir.join("suu.db");
            let db = SuuDatabase::new(db_path).expect("suu.db başlatılamadı");
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_dashboard_state,
            log_intake,
            delete_intake,
            update_settings,
            get_calendar_history,
        ])
        .run(tauri::generate_context!())
        .expect("SUU başlatılırken hata oluştu");
}
