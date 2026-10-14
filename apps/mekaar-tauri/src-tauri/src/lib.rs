pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_engine_info,
            commands::ping_relay,
            commands::verify_vault_pin,
            commands::unlock_vault,
            commands::lock_vault,
            commands::list_conversations,
            commands::list_messages,
            commands::send_message,
            commands::get_guardians,
            commands::trigger_sos_alert,
            commands::export_forensic_audit_trail,
            commands::purge_expired_messages,
            commands::connect_relay,
            commands::disconnect_relay,
        ])
        .run(tauri::generate_context!())
        .expect("error while running mekaar application");
}
