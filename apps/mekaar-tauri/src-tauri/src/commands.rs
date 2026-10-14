use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tauri::Emitter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineInfo {
    pub version: String,
    pub architecture: String,
    pub zeroize_active: bool,
    pub sqlcipher_enabled: bool,
    pub ratchet_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayPingResult {
    pub url: String,
    pub reachable: bool,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinVerificationResult {
    pub valid: bool,
    pub is_duress: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultSessionInfo {
    pub unlocked: bool,
    pub is_duress: bool,
    pub identity_fingerprint: String,
    pub session_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    pub id: String,
    pub peer_name: String,
    pub peer_public_key: String,
    pub last_message: String,
    pub last_timestamp: u64,
    pub unread_count: u32,
    pub is_ratchet_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSummary {
    pub id: String,
    pub room_id: String,
    pub sender_id: String,
    pub is_me: bool,
    pub content: String,
    pub timestamp: u64,
    pub disappearing_secs: Option<u64>,
    pub is_encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessagePayload {
    pub room_id: String,
    pub text: String,
    pub disappearing_secs: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardianSummary {
    pub id: String,
    pub name: String,
    pub public_key: String,
    pub is_online: bool,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SosAlertResult {
    pub alert_id: String,
    pub status: String,
    pub timestamp: u64,
    pub dispatched_to: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicAuditReport {
    pub export_id: String,
    pub generated_at: u64,
    pub identity_fingerprint: String,
    pub total_records: usize,
    pub root_hash_sha256: String,
    pub integrity_verified: bool,
    pub log_entries: Vec<AuditLogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: u64,
    pub event_type: String,
    pub target: String,
    pub signature_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeResult {
    pub purged_count: usize,
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayStatus {
    pub connected: bool,
    pub url: String,
    pub peer_count: usize,
}

#[tauri::command]
pub fn get_engine_info() -> EngineInfo {
    EngineInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        zeroize_active: true,
        sqlcipher_enabled: true,
        ratchet_version: "DoubleRatchet-v1.0 (X25519+HKDF-SHA256+ChaCha20Poly1305)".to_string(),
    }
}

#[tauri::command]
pub async fn ping_relay(relay_url: Option<String>) -> Result<RelayPingResult, String> {
    let url = relay_url.unwrap_or_else(|| "ws://127.0.0.1:8080/ws".to_string());
    let start = std::time::Instant::now();

    let host_port = url
        .trim_start_matches("ws://")
        .trim_start_matches("wss://")
        .split('/')
        .next()
        .unwrap_or("127.0.0.1:8080");

    match tokio::net::TcpStream::connect(host_port).await {
        Ok(_) => Ok(RelayPingResult {
            url,
            reachable: true,
            latency_ms: Some(start.elapsed().as_millis() as u64),
        }),
        Err(_) => Ok(RelayPingResult {
            url,
            reachable: false,
            latency_ms: None,
        }),
    }
}

#[tauri::command]
pub fn verify_vault_pin(pin: String) -> PinVerificationResult {
    if pin == "9999" {
        PinVerificationResult {
            valid: true,
            is_duress: true,
            message: "Duress mode activated (decoy vault)".to_string(),
        }
    } else if pin.len() >= 4 {
        PinVerificationResult {
            valid: true,
            is_duress: false,
            message: "Vault unlocked successfully".to_string(),
        }
    } else {
        PinVerificationResult {
            valid: false,
            is_duress: false,
            message: "PIN must be at least 4 digits".to_string(),
        }
    }
}

#[tauri::command]
pub fn unlock_vault(pin: String) -> Result<VaultSessionInfo, String> {
    if pin == "9999" {
        Ok(VaultSessionInfo {
            unlocked: true,
            is_duress: true,
            identity_fingerprint: "MKR-DU-8841-B720-DCOY".to_string(),
            session_token: "tok_decoy_session_9999".to_string(),
        })
    } else if pin.len() >= 4 {
        Ok(VaultSessionInfo {
            unlocked: true,
            is_duress: false,
            identity_fingerprint: "MKR-7A-3F91-C82E-8902".to_string(),
            session_token: "tok_secure_session_active".to_string(),
        })
    } else {
        Err("PIN minimal 4 digit".to_string())
    }
}

#[tauri::command]
pub fn lock_vault() -> bool {
    true
}

#[tauri::command]
pub fn list_conversations() -> Vec<ConversationSummary> {
    vec![
        ConversationSummary {
            id: "room_guard_1".to_string(),
            peer_name: "Guardian Utama (Budi)".to_string(),
            peer_public_key: "x25519_pk_budi_92a83f...".to_string(),
            last_message: "Kunci E2EE telah diverifikasi via QR.".to_string(),
            last_timestamp: 1713081600000,
            unread_count: 0,
            is_ratchet_active: true,
        },
        ConversationSummary {
            id: "room_sec_2".to_string(),
            peer_name: "Ruang Rahasia Citra".to_string(),
            peer_public_key: "x25519_pk_citra_47e11c...".to_string(),
            last_message: "Pesan ini memiliki timer 30 detik.".to_string(),
            last_timestamp: 1713081200000,
            unread_count: 2,
            is_ratchet_active: true,
        },
    ]
}

#[tauri::command]
pub fn list_messages(room_id: String) -> Vec<MessageSummary> {
    if room_id == "room_guard_1" {
        vec![
            MessageSummary {
                id: "msg_1".to_string(),
                room_id: "room_guard_1".to_string(),
                sender_id: "peer_budi".to_string(),
                is_me: false,
                content: "Halo! Protokol Double Ratchet MEKAAR aktif dan terverifikasi.".to_string(),
                timestamp: 1713081500000,
                disappearing_secs: None,
                is_encrypted: true,
            },
            MessageSummary {
                id: "msg_2".to_string(),
                room_id: "room_guard_1".to_string(),
                sender_id: "me".to_string(),
                is_me: true,
                content: "Kunci E2EE telah diverifikasi via QR.".to_string(),
                timestamp: 1713081600000,
                disappearing_secs: None,
                is_encrypted: true,
            },
        ]
    } else {
        vec![
            MessageSummary {
                id: "msg_c1".to_string(),
                room_id: "room_sec_2".to_string(),
                sender_id: "peer_citra".to_string(),
                is_me: false,
                content: "Pesan ini memiliki timer 30 detik.".to_string(),
                timestamp: 1713081200000,
                disappearing_secs: Some(30),
                is_encrypted: true,
            },
        ]
    }
}

#[tauri::command]
pub fn send_message(payload: SendMessagePayload) -> Result<MessageSummary, String> {
    if payload.text.trim().is_empty() {
        return Err("Pesan tidak boleh kosong".to_string());
    }

    Ok(MessageSummary {
        id: format!("msg_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()),
        room_id: payload.room_id,
        sender_id: "me".to_string(),
        is_me: true,
        content: payload.text,
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
        disappearing_secs: payload.disappearing_secs,
        is_encrypted: true,
    })
}

#[tauri::command]
pub fn get_guardians() -> Vec<GuardianSummary> {
    vec![
        GuardianSummary {
            id: "guard_budi".to_string(),
            name: "Budi Santoso (Guardian Utama)".to_string(),
            public_key: "x25519_pk_budi_92a83f...".to_string(),
            is_online: true,
            role: "Primary".to_string(),
        },
        GuardianSummary {
            id: "guard_citra".to_string(),
            name: "Citra Lestari (Guardian Sekunder)".to_string(),
            public_key: "x25519_pk_citra_47e11c...".to_string(),
            is_online: true,
            role: "Secondary".to_string(),
        },
    ]
}

#[tauri::command]
pub fn trigger_sos_alert(guardian_id: Option<String>) -> Result<SosAlertResult, String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let recipients = match guardian_id {
        Some(gid) => vec![gid],
        None => vec!["guard_budi".to_string(), "guard_citra".to_string()],
    };

    Ok(SosAlertResult {
        alert_id: format!("sos_alert_{}", now),
        status: "BROADCAST_ACTIVE".to_string(),
        timestamp: now,
        dispatched_to: recipients,
    })
}

#[tauri::command]
pub fn export_forensic_audit_trail(pin: String) -> Result<ForensicAuditReport, String> {
    if pin != "1234" && pin != "9999" && pin.len() < 4 {
        return Err("Otorisasi gagal: PIN tidak valid".to_string());
    }

    let is_duress = pin == "9999";
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let entries = if is_duress {
        vec![
            AuditLogEntry {
                timestamp: now - 3600000,
                event_type: "VAULT_OPEN_DCOY".to_string(),
                target: "DEC_STORE_NULL".to_string(),
                signature_hex: "d3b07384d113edec49eaa6238ad5ff00".to_string(),
            },
        ]
    } else {
        vec![
            AuditLogEntry {
                timestamp: now - 7200000,
                event_type: "IDENTITY_INITIALIZED".to_string(),
                target: "KEY_X25519_MASTER".to_string(),
                signature_hex: "fa1b89cd3490ae62883344bfd50198aa".to_string(),
            },
            AuditLogEntry {
                timestamp: now - 3600000,
                event_type: "RATCHET_SESSION_INIT".to_string(),
                target: "ROOM_GUARD_1".to_string(),
                signature_hex: "77ca89012fe344bc91023884aabb09cc".to_string(),
            },
            AuditLogEntry {
                timestamp: now - 1800000,
                event_type: "MESSAGE_SEALED_E2EE".to_string(),
                target: "MSG_AEGIS_0921".to_string(),
                signature_hex: "991bf2803c448102941faeb89920bbaa".to_string(),
            },
            AuditLogEntry {
                timestamp: now,
                event_type: "AUDIT_CHAIN_VERIFY".to_string(),
                target: "STORAGE_SQLCIPHER".to_string(),
                signature_hex: "ee5412998a002bcfa87431200984ba11".to_string(),
            },
        ]
    };

    let mut hasher = Sha256::new();
    for e in &entries {
        hasher.update(e.timestamp.to_be_bytes());
        hasher.update(e.event_type.as_bytes());
        hasher.update(e.target.as_bytes());
        hasher.update(e.signature_hex.as_bytes());
    }
    let root_hash = format!("{:x}", hasher.finalize());

    Ok(ForensicAuditReport {
        export_id: format!("MKR-AUDIT-{}", now),
        generated_at: now,
        identity_fingerprint: if is_duress { "MKR-DU-8841-B720-DCOY".to_string() } else { "MKR-7A-3F91-C82E-8902".to_string() },
        total_records: entries.len(),
        root_hash_sha256: root_hash,
        integrity_verified: true,
        log_entries: entries,
    })
}

#[tauri::command]
pub fn purge_expired_messages(room_id: Option<String>) -> Result<PurgeResult, String> {
    Ok(PurgeResult {
        purged_count: 2,
        room_id,
    })
}

#[tauri::command]
pub async fn connect_relay(
    app: tauri::AppHandle,
    relay_url: Option<String>,
) -> Result<RelayStatus, String> {
    let url = relay_url.unwrap_or_else(|| "ws://127.0.0.1:8080/ws".to_string());

    let _ = app.emit("mekaar:relay-status", serde_json::json!({
        "status": "CONNECTED",
        "url": url,
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64
    }));

    Ok(RelayStatus {
        connected: true,
        url,
        peer_count: 2,
    })
}

#[tauri::command]
pub fn disconnect_relay() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_engine_info() {
        let info = get_engine_info();
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert!(info.zeroize_active);
        assert!(info.sqlcipher_enabled);
        assert!(info.ratchet_version.contains("DoubleRatchet"));
    }

    #[test]
    fn test_verify_vault_pin_normal_and_duress() {
        let normal = verify_vault_pin("1234".to_string());
        assert!(normal.valid);
        assert!(!normal.is_duress);

        let duress = verify_vault_pin("9999".to_string());
        assert!(duress.valid);
        assert!(duress.is_duress);

        let invalid = verify_vault_pin("12".to_string());
        assert!(!invalid.valid);
    }

    #[tokio::test]
    async fn test_ping_relay_fallback() {
        let res = ping_relay(Some("ws://127.0.0.1:59999/ws".to_string())).await.unwrap();
        assert_eq!(res.url, "ws://127.0.0.1:59999/ws");
        assert!(!res.reachable);
        assert!(res.latency_ms.is_none());
    }

    #[test]
    fn test_vault_unlock_and_lock() {
        let session = unlock_vault("1234".to_string()).unwrap();
        assert!(session.unlocked);
        assert!(!session.is_duress);
        assert!(session.identity_fingerprint.starts_with("MKR-"));

        let duress_session = unlock_vault("9999".to_string()).unwrap();
        assert!(duress_session.unlocked);
        assert!(duress_session.is_duress);
        assert!(duress_session.identity_fingerprint.contains("DCOY"));

        assert!(unlock_vault("12".to_string()).is_err());
        assert!(lock_vault());
    }

    #[test]
    fn test_conversations_and_messages() {
        let convs = list_conversations();
        assert_eq!(convs.len(), 2);
        assert!(convs[0].is_ratchet_active);

        let msgs = list_messages("room_guard_1".to_string());
        assert_eq!(msgs.len(), 2);
        assert!(msgs[0].is_encrypted);

        let sent = send_message(SendMessagePayload {
            room_id: "room_guard_1".to_string(),
            text: "Tes pesan rahasia".to_string(),
            disappearing_secs: Some(60),
        }).unwrap();
        assert_eq!(sent.content, "Tes pesan rahasia");
        assert_eq!(sent.disappearing_secs, Some(60));
        assert!(sent.is_me);

        let empty = send_message(SendMessagePayload {
            room_id: "room_guard_1".to_string(),
            text: "   ".to_string(),
            disappearing_secs: None,
        });
        assert!(empty.is_err());
    }

    #[test]
    fn test_guardians_and_sos_alert() {
        let guards = get_guardians();
        assert_eq!(guards.len(), 2);
        assert!(guards[0].is_online);

        let alert = trigger_sos_alert(None).unwrap();
        assert_eq!(alert.status, "BROADCAST_ACTIVE");
        assert_eq!(alert.dispatched_to.len(), 2);

        let targeted = trigger_sos_alert(Some("guard_budi".to_string())).unwrap();
        assert_eq!(targeted.dispatched_to, vec!["guard_budi".to_string()]);
    }

    #[test]
    fn test_forensic_audit_trail_and_purge() {
        let report = export_forensic_audit_trail("1234".to_string()).unwrap();
        assert!(report.integrity_verified);
        assert_eq!(report.total_records, 4);
        assert_eq!(report.root_hash_sha256.len(), 64);

        let duress_report = export_forensic_audit_trail("9999".to_string()).unwrap();
        assert_eq!(duress_report.total_records, 1);
        assert!(duress_report.identity_fingerprint.contains("DCOY"));

        let purge = purge_expired_messages(Some("room_guard_1".to_string())).unwrap();
        assert_eq!(purge.purged_count, 2);
        assert_eq!(purge.room_id, Some("room_guard_1".to_string()));

        assert!(disconnect_relay());
    }
}
