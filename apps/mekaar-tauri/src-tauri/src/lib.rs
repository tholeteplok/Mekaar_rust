pub mod commands;

use commands::MekaarAppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(MekaarAppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::get_connection_config,
            commands::set_connection_config,
            commands::unlock_vault,
            commands::lock_vault,
            commands::verify_pin,
            commands::set_pin,
            commands::trigger_sos,
            commands::cancel_sos,
            commands::get_sos_settings,
            commands::update_sos_settings,
            commands::get_app_settings,
            commands::update_app_settings,
            commands::set_screenshot_protection,
            commands::export_forensic_bundle,
            commands::trigger_device_lost_wipe,
        ])
        .run(tauri::generate_context!())
        .expect("error while running mekaar-tauri application");
}
