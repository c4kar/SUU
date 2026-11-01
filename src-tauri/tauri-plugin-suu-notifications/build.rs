const COMMANDS: &[&str] = &[
    "reschedule_next_reminder",
    "request_one_ui_permissions",
    "consume_launch_intent",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
