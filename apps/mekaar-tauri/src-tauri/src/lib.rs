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
            commands::setup_pins,
            commands::get_vault_status,
            commands::get_conversations,
            commands::get_messages,
            commands::send_e2ee_message,
            commands::get_contacts,
            commands::toggle_block_contact,
            commands::trigger_sos,
            commands::cancel_sos,
            commands::get_security_logs,
            commands::export_signed_forensic_bundle,
            commands::get_sos_settings,
            commands::update_sos_settings,
            commands::get_app_settings,
            commands::update_app_settings,
            commands::set_screenshot_protection,
            commands::trigger_device_lost_wipe,
            commands::get_trips,
            commands::start_trip,
            commands::complete_trip,
            commands::get_saved_places,
            commands::add_saved_place,
            commands::delete_saved_place,
            commands::get_guardians,
            commands::add_guardian,
            commands::update_guardian_status,
            commands::delete_guardian,
            commands::update_telemetry,
            commands::get_telemetry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running mekaar-tauri application");
}
