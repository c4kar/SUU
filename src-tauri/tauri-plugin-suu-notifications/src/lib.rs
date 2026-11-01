use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[cfg(target_os = "android")]
use tauri::plugin::PluginHandle;
#[cfg(target_os = "android")]
use tauri::Manager;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchIntentState {
    pub open_quick_drawer: bool,
}

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.c4kar.suu.notifications";

#[cfg(target_os = "android")]
pub struct SuuNotifications<R: Runtime>(pub PluginHandle<R>);

#[tauri::command]
async fn reschedule_next_reminder<R: Runtime>(_app: tauri::AppHandle<R>) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        if let Some(handle) = _app.try_state::<SuuNotifications<R>>() {
            handle
                .0
                .run_mobile_plugin::<serde_json::Value>("rescheduleNextReminder", ())
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
async fn request_one_ui_permissions<R: Runtime>(_app: tauri::AppHandle<R>) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        if let Some(handle) = _app.try_state::<SuuNotifications<R>>() {
            handle
                .0
                .run_mobile_plugin::<serde_json::Value>("requestOneUiPermissions", ())
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
async fn consume_launch_intent<R: Runtime>(
    _app: tauri::AppHandle<R>,
) -> Result<LaunchIntentState, String> {
    #[cfg(target_os = "android")]
    {
        if let Some(handle) = _app.try_state::<SuuNotifications<R>>() {
            return handle
                .0
                .run_mobile_plugin::<LaunchIntentState>("consumeLaunchIntent", ())
                .map_err(|e| e.to_string());
        }
    }
    Ok(LaunchIntentState {
        open_quick_drawer: false,
    })
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("suu-notifications")
        .invoke_handler(tauri::generate_handler![
            reschedule_next_reminder,
            request_one_ui_permissions,
            consume_launch_intent,
        ])
        .setup(|_app, _api| {
            #[cfg(target_os = "android")]
            {
                let handle = _api.register_android_plugin(
                    PLUGIN_IDENTIFIER,
                    "SuuNotificationsPlugin",
                )?;
                _app.manage(SuuNotifications(handle));
            }
            Ok(())
        })
        .build()
}
