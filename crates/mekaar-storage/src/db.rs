//! # mekaar-storage: Encrypted SQLCipher Database
//!
//! Implementasi database lokal terenkripsi penuh AES-256-CBC via SQLCipher.
//! Menyediakan persistensi aman untuk seluruh data aplikasi di disk klien.

use crate::error::StorageError;
use crate::models::{
    ChatRoom, Contact, DeliveryStatus, GuardianRecord, LocalMessage, SavedPlaceRecord, SecurityLog,
    SosSessionRecord, UserTripRecord,
};
use mekaar_proto::MessageType;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Database lokal terenkripsi SQLCipher.
#[derive(Clone)]
pub struct EncryptedDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl EncryptedDatabase {
    /// Membuka file database terenkripsi SQLCipher menggunakan passkey 256-bit.
    pub fn open(path: &str, passkey: &str) -> Result<Self, StorageError> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init(passkey)?;
        Ok(db)
    }

    /// Membuka in-memory database terenkripsi untuk testing.
    pub fn open_in_memory(passkey: &str) -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init(passkey)?;
        Ok(db)
    }

    /// Menerapkan passkey enkripsi SQLCipher dan migrasi skema tabel.
    fn init(&self, passkey: &str) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();

        // 1. Terapkan passkey SQLCipher (wajib dieksekusi sebelum query lain)
        let escaped_key = passkey.replace('\'', "''");
        conn.execute_batch(&format!("PRAGMA key = '{}';", escaped_key))?;

        // 2. Aktifkan WAL mode dan integritas foreign keys
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;",
        )?;

        // 3. Buat skema tabel
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS local_messages (
                id TEXT PRIMARY KEY,
                room_id TEXT NOT NULL,
                sender_id TEXT NOT NULL,
                recipient_id TEXT NOT NULL,
                content TEXT NOT NULL,
                media_url TEXT,
                msg_type TEXT NOT NULL,
                is_view_once INTEGER NOT NULL,
                is_opened INTEGER NOT NULL,
                opened_at INTEGER,
                reply_to_id TEXT,
                is_deleted INTEGER NOT NULL,
                is_silent_deleted INTEGER NOT NULL,
                auto_delete_at INTEGER,
                edited_at INTEGER,
                reactions_json TEXT NOT NULL,
                is_encrypted INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                status TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_local_msg_room ON local_messages(room_id, created_at);
            CREATE INDEX IF NOT EXISTS idx_local_msg_expiry ON local_messages(auto_delete_at);

            CREATE TABLE IF NOT EXISTS chat_rooms (
                id TEXT PRIMARY KEY,
                room_type TEXT NOT NULL,
                name TEXT NOT NULL,
                peer_id TEXT,
                disappearing_timer INTEGER,
                theme_preset TEXT NOT NULL,
                last_message_at INTEGER NOT NULL,
                is_archived INTEGER NOT NULL,
                is_muted INTEGER NOT NULL,
                is_vault_hidden INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                display_name TEXT NOT NULL,
                public_key BLOB NOT NULL,
                safety_number TEXT NOT NULL,
                status TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS security_logs (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                sos_session_id TEXT,
                event_type TEXT NOT NULL,
                details_json TEXT,
                created_at INTEGER NOT NULL,
                checksum TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_security_logs_user ON security_logs(user_id);

            CREATE TABLE IF NOT EXISTS sos_sessions (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                guardian_id TEXT,
                started_at INTEGER NOT NULL,
                ended_at INTEGER,
                status TEXT NOT NULL,
                gps_enabled INTEGER NOT NULL,
                mic_enabled INTEGER NOT NULL,
                video_enabled INTEGER NOT NULL,
                ended_reason TEXT,
                created_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS key_store (
                key_name TEXT PRIMARY KEY,
                key_data TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS user_trips (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                origin_name TEXT NOT NULL,
                destination_name TEXT NOT NULL,
                dest_lat REAL NOT NULL,
                dest_lng REAL NOT NULL,
                dest_radius INTEGER NOT NULL,
                eta_minutes INTEGER NOT NULL,
                status TEXT NOT NULL,
                started_at INTEGER NOT NULL,
                ended_at INTEGER,
                active_guardians_json TEXT NOT NULL,
                progress_percent INTEGER NOT NULL,
                created_at INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_trips_user ON user_trips(user_id);

            CREATE TABLE IF NOT EXISTS saved_places (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                address TEXT,
                latitude REAL NOT NULL,
                longitude REAL NOT NULL,
                radius_meters INTEGER NOT NULL,
                auto_notify_guardian INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_places_user ON saved_places(user_id);

            CREATE TABLE IF NOT EXISTS guardians (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                name TEXT NOT NULL,
                username TEXT NOT NULL,
                avatar_color TEXT NOT NULL,
                relation TEXT NOT NULL,
                status TEXT NOT NULL,
                can_view_live_location INTEGER NOT NULL,
                can_receive_sos_alert INTEGER NOT NULL,
                battery_percent INTEGER NOT NULL,
                last_latitude REAL,
                last_longitude REAL,
                updated_at INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_guardians_user ON guardians(user_id);",
        )?;

        Ok(())
    }

    // ── Local Messages ─────────────────────────────────────────────────

    /// Menyimpan pesan baru ke database lokal.
    pub fn insert_message(&self, msg: &LocalMessage) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        let reactions_json = serde_json::to_string(&msg.reactions).unwrap_or_else(|_| "{}".to_string());
        let msg_type_str = serde_json::to_string(&msg.msg_type).unwrap_or_else(|_| "\"text\"".to_string());
        let clean_msg_type = msg_type_str.trim_matches('"');

        conn.execute(
            "INSERT INTO local_messages (
                id, room_id, sender_id, recipient_id, content, media_url, msg_type,
                is_view_once, is_opened, opened_at, reply_to_id, is_deleted, is_silent_deleted,
                auto_delete_at, edited_at, reactions_json, is_encrypted, created_at, updated_at, status
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20
            ) ON CONFLICT(id) DO UPDATE SET
                content = excluded.content,
                status = excluded.status,
                is_opened = excluded.is_opened,
                opened_at = excluded.opened_at,
                is_deleted = excluded.is_deleted,
                reactions_json = excluded.reactions_json,
                updated_at = excluded.updated_at",
            params![
                msg.id.to_string(),
                msg.room_id.to_string(),
                msg.sender_id.to_string(),
                msg.recipient_id.to_string(),
                msg.content,
                msg.media_url,
                clean_msg_type,
                msg.is_view_once as i32,
                msg.is_opened as i32,
                msg.opened_at.map(|v| v as i64),
                msg.reply_to_id.map(|v| v.to_string()),
                msg.is_deleted as i32,
                msg.is_silent_deleted as i32,
                msg.auto_delete_at.map(|v| v as i64),
                msg.edited_at.map(|v| v as i64),
                reactions_json,
                msg.is_encrypted as i32,
                msg.created_at as i64,
                msg.updated_at as i64,
                msg.status.as_str()
            ],
        )?;

        Ok(())
    }

    /// Mengambil daftar riwayat pesan dalam room dengan batasan limit (terurut dari yang terlama ke terbaru).
    pub fn get_messages_for_room(
        &self,
        room_id: &Uuid,
        limit: usize,
    ) -> Result<Vec<LocalMessage>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, room_id, sender_id, recipient_id, content, media_url, msg_type,
                    is_view_once, is_opened, opened_at, reply_to_id, is_deleted, is_silent_deleted,
                    auto_delete_at, edited_at, reactions_json, is_encrypted, created_at, updated_at, status
             FROM local_messages
             WHERE room_id = ?1 AND is_deleted = 0
             ORDER BY created_at DESC
             LIMIT ?2",
        )?;

        let rows = stmt.query_map(params![room_id.to_string(), limit as i64], |row| {
            Self::row_to_message(row)
        })?;

        let mut messages = Vec::new();
        for item in rows {
            messages.push(item?);
        }

        messages.reverse(); // Urutkan kronologis
        Ok(messages)
    }

    /// Mengambil pesan spesifik berdasarkan ID.
    pub fn get_message(&self, message_id: &Uuid) -> Result<Option<LocalMessage>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, room_id, sender_id, recipient_id, content, media_url, msg_type,
                    is_view_once, is_opened, opened_at, reply_to_id, is_deleted, is_silent_deleted,
                    auto_delete_at, edited_at, reactions_json, is_encrypted, created_at, updated_at, status
             FROM local_messages WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![message_id.to_string()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(Self::row_to_message(row)?));
        }

        Ok(None)
    }

    /// Memperbarui status pengiriman pesan lokal (queued, sent, delivered, read, failed).
    pub fn update_message_status(
        &self,
        message_id: &Uuid,
        status: DeliveryStatus,
    ) -> Result<bool, StorageError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        let rows = conn.execute(
            "UPDATE local_messages SET status = ?1, updated_at = ?2 WHERE id = ?3",
            params![status.as_str(), now, message_id.to_string()],
        )?;

        Ok(rows > 0)
    }

    /// Scrubbing View-Once: Menandai pesan sekali lihat telah dibuka,
    /// serta meng-overwrite isi konten plaintext dengan placeholder aman.
    pub fn mark_as_viewed(&self, message_id: &Uuid) -> Result<bool, StorageError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        let rows = conn.execute(
            "UPDATE local_messages SET
                content = '🔒 Pesan sekali lihat telah dibuka',
                is_opened = 1,
                opened_at = ?1,
                updated_at = ?1
             WHERE id = ?2 AND is_view_once = 1 AND is_opened = 0",
            params![now, message_id.to_string()],
        )?;

        Ok(rows > 0)
    }

    /// Menghapus pesan secara permanen dari database (digunakan oleh Disappearing Message worker).
    pub fn purge_expired_messages(&self, now: u64) -> Result<usize, StorageError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute(
            "DELETE FROM local_messages WHERE auto_delete_at IS NOT NULL AND auto_delete_at <= ?1",
            params![now as i64],
        )?;

        Ok(rows)
    }

    /// Menghapus seluruh pesan dalam room tertentu (Burn-on-Exit / Wipe).
    pub fn wipe_room_messages(&self, room_id: &Uuid) -> Result<usize, StorageError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute(
            "DELETE FROM local_messages WHERE room_id = ?1",
            params![room_id.to_string()],
        )?;

        Ok(rows)
    }

    // ── Chat Rooms ─────────────────────────────────────────────────────

    /// Mendaftarkan atau memperbarui ruang obrolan.
    pub fn upsert_room(&self, room: &ChatRoom) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO chat_rooms (
                id, room_type, name, peer_id, disappearing_timer, theme_preset,
                last_message_at, is_archived, is_muted, is_vault_hidden
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                disappearing_timer = excluded.disappearing_timer,
                theme_preset = excluded.theme_preset,
                last_message_at = excluded.last_message_at,
                is_archived = excluded.is_archived,
                is_muted = excluded.is_muted,
                is_vault_hidden = excluded.is_vault_hidden",
            params![
                room.id.to_string(),
                room.room_type,
                room.name,
                room.peer_id.map(|v| v.to_string()),
                room.disappearing_timer.map(|v| v as i64),
                room.theme_preset,
                room.last_message_at as i64,
                room.is_archived as i32,
                room.is_muted as i32,
                room.is_vault_hidden as i32
            ],
        )?;

        Ok(())
    }

    /// Mengambil detail room berdasarkan ID.
    pub fn get_room(&self, room_id: &Uuid) -> Result<Option<ChatRoom>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, room_type, name, peer_id, disappearing_timer, theme_preset,
                    last_message_at, is_archived, is_muted, is_vault_hidden
             FROM chat_rooms WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![room_id.to_string()])?;
        if let Some(row) = rows.next()? {
            return Ok(Some(Self::row_to_room(row)?));
        }

        Ok(None)
    }

    /// Mengambil daftar seluruh room. Parameter `include_vault_hidden` mengontrol apakah
    /// chat room rahasia (Private Vault) disertakan atau disembunyikan.
    pub fn list_rooms(&self, include_vault_hidden: bool) -> Result<Vec<ChatRoom>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let query = if include_vault_hidden {
            "SELECT id, room_type, name, peer_id, disappearing_timer, theme_preset,
                    last_message_at, is_archived, is_muted, is_vault_hidden
             FROM chat_rooms ORDER BY last_message_at DESC"
        } else {
            "SELECT id, room_type, name, peer_id, disappearing_timer, theme_preset,
                    last_message_at, is_archived, is_muted, is_vault_hidden
             FROM chat_rooms WHERE is_vault_hidden = 0 ORDER BY last_message_at DESC"
        };

        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([], |row| Self::row_to_room(row))?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }

        Ok(list)
    }

    // ── Contacts ───────────────────────────────────────────────────────

    pub fn upsert_contact(&self, contact: &Contact) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO contacts (id, username, display_name, public_key, safety_number, status, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
                username = excluded.username,
                display_name = excluded.display_name,
                public_key = excluded.public_key,
                safety_number = excluded.safety_number,
                status = excluded.status,
                updated_at = excluded.updated_at",
            params![
                contact.id.to_string(),
                contact.username,
                contact.display_name,
                contact.public_key.as_slice(),
                contact.safety_number,
                contact.status,
                contact.updated_at as i64
            ],
        )?;

        Ok(())
    }

    pub fn get_contact(&self, id: &Uuid) -> Result<Option<Contact>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, display_name, public_key, safety_number, status, updated_at
             FROM contacts WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id.to_string()])?;
        if let Some(row) = rows.next()? {
            let id_str: String = row.get(0)?;
            let pub_blob: Vec<u8> = row.get(3)?;
            let mut pub_key = [0u8; 32];
            if pub_blob.len() == 32 {
                pub_key.copy_from_slice(&pub_blob);
            }

            return Ok(Some(Contact {
                id: Uuid::parse_str(&id_str).map_err(|e| StorageError::CorruptedData(e.to_string()))?,
                username: row.get(1)?,
                display_name: row.get(2)?,
                public_key: pub_key,
                safety_number: row.get(4)?,
                status: row.get(5)?,
                updated_at: row.get::<_, i64>(6)? as u64,
            }));
        }

        Ok(None)
    }

    /// Mengambil seluruh kontak yang tersimpan di database lokal.
    pub fn list_contacts(&self) -> Result<Vec<Contact>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, username, display_name, public_key, safety_number, status, updated_at
             FROM contacts ORDER BY display_name ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            let id_str: String = row.get(0)?;
            let pub_blob: Vec<u8> = row.get(3)?;
            let mut pub_key = [0u8; 32];
            if pub_blob.len() == 32 {
                pub_key.copy_from_slice(&pub_blob);
            }

            Ok(Contact {
                id: Uuid::parse_str(&id_str).unwrap_or_default(),
                username: row.get(1)?,
                display_name: row.get(2)?,
                public_key: pub_key,
                safety_number: row.get(4)?,
                status: row.get(5)?,
                updated_at: row.get::<_, i64>(6)? as u64,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }

        Ok(list)
    }

    // ── Security Logs (Audit Trail Anti-Tamper) ─────────────────────────

    /// Mencatat log audit bukti hukum permanen dengan checksum SHA-256.
    pub fn insert_security_log(
        &self,
        user_id: &Uuid,
        sos_session_id: Option<Uuid>,
        event_type: &str,
        details: Option<&serde_json::Value>,
    ) -> Result<SecurityLog, StorageError> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let details_json = details.map(|d| d.to_string());

        // Hitung checksum tamper-evident
        let mut hasher = Sha256::new();
        hasher.update(id.as_bytes());
        hasher.update(user_id.as_bytes());
        hasher.update(event_type.as_bytes());
        hasher.update(now.to_le_bytes());
        if let Some(ref d) = details_json {
            hasher.update(d.as_bytes());
        }
        let checksum = format!("{:02x}", hasher.finalize());

        conn.execute(
            "INSERT INTO security_logs (id, user_id, sos_session_id, event_type, details_json, created_at, checksum)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id.to_string(),
                user_id.to_string(),
                sos_session_id.map(|v| v.to_string()),
                event_type,
                details_json,
                now as i64,
                checksum
            ],
        )?;

        Ok(SecurityLog {
            id,
            user_id: *user_id,
            sos_session_id,
            event_type: event_type.to_string(),
            details_json,
            created_at: now,
            checksum,
        })
    }

    /// Mengambil seluruh log audit untuk verifikasi.
    pub fn get_security_logs(&self, user_id: &Uuid) -> Result<Vec<SecurityLog>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, sos_session_id, event_type, details_json, created_at, checksum
             FROM security_logs WHERE user_id = ?1 ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map(params![user_id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            let uid_str: String = row.get(1)?;
            let sos_str: Option<String> = row.get(2)?;

            Ok(SecurityLog {
                id: Uuid::parse_str(&id_str).unwrap_or_default(),
                user_id: Uuid::parse_str(&uid_str).unwrap_or_default(),
                sos_session_id: sos_str.and_then(|s| Uuid::parse_str(&s).ok()),
                event_type: row.get(3)?,
                details_json: row.get(4)?,
                created_at: row.get::<_, i64>(5)? as u64,
                checksum: row.get(6)?,
            })
        })?;

        let mut list = Vec::new();
        for item in rows {
            list.push(item?);
        }

        Ok(list)
    }

    // ── Key Store ──────────────────────────────────────────────────────

    pub fn set_key_item(&self, key_name: &str, key_data: &str) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        conn.execute(
            "INSERT INTO key_store (key_name, key_data, updated_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(key_name) DO UPDATE SET
                key_data = excluded.key_data,
                updated_at = excluded.updated_at",
            params![key_name, key_data, now],
        )?;

        Ok(())
    }

    pub fn get_key_item(&self, key_name: &str) -> Result<Option<String>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key_data FROM key_store WHERE key_name = ?1")?;

        let mut rows = stmt.query(params![key_name])?;
        if let Some(row) = rows.next()? {
            let data: String = row.get(0)?;
            return Ok(Some(data));
        }

        Ok(None)
    }

    // ── SOS Sessions ───────────────────────────────────────────────────

    pub fn insert_sos_session(&self, record: &SosSessionRecord) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO sos_sessions (
                id, user_id, guardian_id, started_at, ended_at, status,
                gps_enabled, mic_enabled, video_enabled, ended_reason, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ON CONFLICT(id) DO UPDATE SET
                ended_at = excluded.ended_at,
                status = excluded.status,
                ended_reason = excluded.ended_reason",
            params![
                record.id.to_string(),
                record.user_id.to_string(),
                record.guardian_id.map(|v| v.to_string()),
                record.started_at as i64,
                record.ended_at.map(|v| v as i64),
                record.status,
                record.gps_enabled as i32,
                record.mic_enabled as i32,
                record.video_enabled as i32,
                record.ended_reason,
                record.created_at as i64
            ],
        )?;

        Ok(())
    }

    pub fn get_active_sos_session(&self, user_id: &Uuid) -> Result<Option<SosSessionRecord>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, guardian_id, started_at, ended_at, status,
                    gps_enabled, mic_enabled, video_enabled, ended_reason, created_at
             FROM sos_sessions WHERE user_id = ?1 AND status = 'active' ORDER BY started_at DESC LIMIT 1",
        )?;

        let mut rows = stmt.query(params![user_id.to_string()])?;
        if let Some(row) = rows.next()? {
            let id_str: String = row.get(0)?;
            let uid_str: String = row.get(1)?;
            let gid_str: Option<String> = row.get(2)?;

            return Ok(Some(SosSessionRecord {
                id: Uuid::parse_str(&id_str).unwrap_or_default(),
                user_id: Uuid::parse_str(&uid_str).unwrap_or_default(),
                guardian_id: gid_str.and_then(|s| Uuid::parse_str(&s).ok()),
                started_at: row.get::<_, i64>(3)? as u64,
                ended_at: row.get::<_, Option<i64>>(4)?.map(|v| v as u64),
                status: row.get(5)?,
                gps_enabled: row.get::<_, i32>(6)? != 0,
                mic_enabled: row.get::<_, i32>(7)? != 0,
                video_enabled: row.get::<_, i32>(8)? != 0,
                ended_reason: row.get(9)?,
                created_at: row.get::<_, i64>(10)? as u64,
            }));
        }

        Ok(None)
    }

    // ── Row Mappers ────────────────────────────────────────────────────

    fn row_to_message(row: &rusqlite::Row) -> Result<LocalMessage, rusqlite::Error> {
        let id_str: String = row.get(0)?;
        let room_str: String = row.get(1)?;
        let sender_str: String = row.get(2)?;
        let recipient_str: String = row.get(3)?;
        let reply_str: Option<String> = row.get(10)?;
        let reactions_raw: String = row.get(15)?;
        let msg_type_str: String = row.get(6)?;
        let status_str: String = row.get(19)?;

        let reactions: HashMap<String, Vec<String>> =
            serde_json::from_str(&reactions_raw).unwrap_or_default();
        let msg_type = match msg_type_str.as_str() {
            "image" => MessageType::Image,
            "voice" => MessageType::Voice,
            "video" => MessageType::Video,
            "location" => MessageType::Location,
            "system" => MessageType::System,
            _ => MessageType::Text,
        };

        Ok(LocalMessage {
            id: Uuid::parse_str(&id_str).unwrap_or_default(),
            room_id: Uuid::parse_str(&room_str).unwrap_or_default(),
            sender_id: Uuid::parse_str(&sender_str).unwrap_or_default(),
            recipient_id: Uuid::parse_str(&recipient_str).unwrap_or_default(),
            content: row.get(4)?,
            media_url: row.get(5)?,
            msg_type,
            is_view_once: row.get::<_, i32>(7)? != 0,
            is_opened: row.get::<_, i32>(8)? != 0,
            opened_at: row.get::<_, Option<i64>>(9)?.map(|v| v as u64),
            reply_to_id: reply_str.and_then(|s| Uuid::parse_str(&s).ok()),
            is_deleted: row.get::<_, i32>(11)? != 0,
            is_silent_deleted: row.get::<_, i32>(12)? != 0,
            auto_delete_at: row.get::<_, Option<i64>>(13)?.map(|v| v as u64),
            edited_at: row.get::<_, Option<i64>>(14)?.map(|v| v as u64),
            reactions,
            is_encrypted: row.get::<_, i32>(16)? != 0,
            created_at: row.get::<_, i64>(17)? as u64,
            updated_at: row.get::<_, i64>(18)? as u64,
            status: DeliveryStatus::from_str(&status_str),
        })
    }

    fn row_to_room(row: &rusqlite::Row) -> Result<ChatRoom, rusqlite::Error> {
        let id_str: String = row.get(0)?;
        let peer_str: Option<String> = row.get(3)?;

        Ok(ChatRoom {
            id: Uuid::parse_str(&id_str).unwrap_or_default(),
            room_type: row.get(1)?,
            name: row.get(2)?,
            peer_id: peer_str.and_then(|s| Uuid::parse_str(&s).ok()),
            disappearing_timer: row.get::<_, Option<i64>>(4)?.map(|v| v as u64),
            theme_preset: row.get(5)?,
            last_message_at: row.get::<_, i64>(6)? as u64,
            is_archived: row.get::<_, i32>(7)? != 0,
            is_muted: row.get::<_, i32>(8)? != 0,
            is_vault_hidden: row.get::<_, i32>(9)? != 0,
        })
    }

    // ── User Trips & Kawal Perjalanan ──────────────────────────────────

    pub fn list_trips(&self, user_id: &Uuid) -> Result<Vec<UserTripRecord>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, origin_name, destination_name, dest_lat, dest_lng,
                    dest_radius, eta_minutes, status, started_at, ended_at,
                    active_guardians_json, progress_percent, created_at
             FROM user_trips
             WHERE user_id = ?1
             ORDER BY created_at DESC",
        )?;

        let rows = stmt.query_map(params![user_id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            let uid_str: String = row.get(1)?;
            let started: i64 = row.get(9)?;
            let ended: Option<i64> = row.get(10)?;
            let created: i64 = row.get(13)?;
            Ok(UserTripRecord {
                id: Uuid::parse_str(&id_str).unwrap_or_default(),
                user_id: Uuid::parse_str(&uid_str).unwrap_or_default(),
                origin_name: row.get(2)?,
                destination_name: row.get(3)?,
                dest_lat: row.get(4)?,
                dest_lng: row.get(5)?,
                dest_radius: row.get::<_, i64>(6)? as u32,
                eta_minutes: row.get::<_, i64>(7)? as u32,
                status: row.get(8)?,
                started_at: started as u64,
                ended_at: ended.map(|v| v as u64),
                active_guardians_json: row.get(11)?,
                progress_percent: row.get::<_, i64>(12)? as u32,
                created_at: created as u64,
            })
        })?;

        let mut trips = Vec::new();
        for r in rows {
            trips.push(r?);
        }
        Ok(trips)
    }

    pub fn upsert_trip(&self, trip: &UserTripRecord) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO user_trips (
                id, user_id, origin_name, destination_name, dest_lat, dest_lng,
                dest_radius, eta_minutes, status, started_at, ended_at,
                active_guardians_json, progress_percent, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
            ON CONFLICT(id) DO UPDATE SET
                origin_name = excluded.origin_name,
                destination_name = excluded.destination_name,
                dest_lat = excluded.dest_lat,
                dest_lng = excluded.dest_lng,
                dest_radius = excluded.dest_radius,
                eta_minutes = excluded.eta_minutes,
                status = excluded.status,
                ended_at = excluded.ended_at,
                active_guardians_json = excluded.active_guardians_json,
                progress_percent = excluded.progress_percent",
            params![
                trip.id.to_string(),
                trip.user_id.to_string(),
                trip.origin_name,
                trip.destination_name,
                trip.dest_lat,
                trip.dest_lng,
                trip.dest_radius as i64,
                trip.eta_minutes as i64,
                trip.status,
                trip.started_at as i64,
                trip.ended_at.map(|v| v as i64),
                trip.active_guardians_json,
                trip.progress_percent as i64,
                trip.created_at as i64,
            ],
        )?;
        Ok(())
    }

    pub fn complete_trip(&self, trip_id: &Uuid, status: &str) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        conn.execute(
            "UPDATE user_trips
             SET status = ?1, progress_percent = 100, ended_at = ?2
             WHERE id = ?3",
            params![status, now, trip_id.to_string()],
        )?;
        Ok(())
    }

    // ── Saved Places & Zona Aman ───────────────────────────────────────

    pub fn list_saved_places(&self, user_id: &Uuid) -> Result<Vec<SavedPlaceRecord>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, category, address, latitude, longitude,
                    radius_meters, auto_notify_guardian, updated_at
             FROM saved_places
             WHERE user_id = ?1
             ORDER BY updated_at DESC",
        )?;

        let rows = stmt.query_map(params![user_id.to_string()], |row| {
            let uid_str: String = row.get(1)?;
            let auto_notify: i64 = row.get(8)?;
            let updated: i64 = row.get(9)?;
            Ok(SavedPlaceRecord {
                id: row.get(0)?,
                user_id: Uuid::parse_str(&uid_str).unwrap_or_default(),
                name: row.get(2)?,
                category: row.get(3)?,
                address: row.get(4)?,
                latitude: row.get(5)?,
                longitude: row.get(6)?,
                radius_meters: row.get::<_, i64>(7)? as u32,
                auto_notify_guardian: auto_notify != 0,
                updated_at: updated as u64,
            })
        })?;

        let mut places = Vec::new();
        for r in rows {
            places.push(r?);
        }
        Ok(places)
    }

    pub fn upsert_saved_place(&self, place: &SavedPlaceRecord) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO saved_places (
                id, user_id, name, category, address, latitude, longitude,
                radius_meters, auto_notify_guardian, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                category = excluded.category,
                address = excluded.address,
                latitude = excluded.latitude,
                longitude = excluded.longitude,
                radius_meters = excluded.radius_meters,
                auto_notify_guardian = excluded.auto_notify_guardian,
                updated_at = excluded.updated_at",
            params![
                place.id,
                place.user_id.to_string(),
                place.name,
                place.category,
                place.address,
                place.latitude,
                place.longitude,
                place.radius_meters as i64,
                if place.auto_notify_guardian { 1 } else { 0 },
                place.updated_at as i64,
            ],
        )?;
        Ok(())
    }

    pub fn delete_saved_place(&self, user_id: &Uuid, place_id: &str) -> Result<bool, StorageError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute(
            "DELETE FROM saved_places WHERE id = ?1 AND user_id = ?2",
            params![place_id, user_id.to_string()],
        )?;
        Ok(affected > 0)
    }

    // ── Guardians & Pendamping ─────────────────────────────────────────

    pub fn list_guardians(&self, user_id: &Uuid) -> Result<Vec<GuardianRecord>, StorageError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, user_id, name, username, avatar_color, relation, status,
                    can_view_live_location, can_receive_sos_alert, battery_percent,
                    last_latitude, last_longitude, updated_at
             FROM guardians
             WHERE user_id = ?1
             ORDER BY updated_at DESC",
        )?;

        let rows = stmt.query_map(params![user_id.to_string()], |row| {
            let uid_str: String = row.get(1)?;
            let can_loc: i64 = row.get(7)?;
            let can_sos: i64 = row.get(8)?;
            let bat: i64 = row.get(9)?;
            let updated: i64 = row.get(12)?;
            Ok(GuardianRecord {
                id: row.get(0)?,
                user_id: Uuid::parse_str(&uid_str).unwrap_or_default(),
                name: row.get(2)?,
                username: row.get(3)?,
                avatar_color: row.get(4)?,
                relation: row.get(5)?,
                status: row.get(6)?,
                can_view_live_location: can_loc != 0,
                can_receive_sos_alert: can_sos != 0,
                battery_percent: bat as u8,
                last_latitude: row.get(10)?,
                last_longitude: row.get(11)?,
                updated_at: updated as u64,
            })
        })?;

        let mut guardians = Vec::new();
        for r in rows {
            guardians.push(r?);
        }
        Ok(guardians)
    }

    pub fn upsert_guardian(&self, guardian: &GuardianRecord) -> Result<(), StorageError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO guardians (
                id, user_id, name, username, avatar_color, relation, status,
                can_view_live_location, can_receive_sos_alert, battery_percent,
                last_latitude, last_longitude, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                username = excluded.username,
                avatar_color = excluded.avatar_color,
                relation = excluded.relation,
                status = excluded.status,
                can_view_live_location = excluded.can_view_live_location,
                can_receive_sos_alert = excluded.can_receive_sos_alert,
                battery_percent = excluded.battery_percent,
                last_latitude = excluded.last_latitude,
                last_longitude = excluded.last_longitude,
                updated_at = excluded.updated_at",
            params![
                guardian.id,
                guardian.user_id.to_string(),
                guardian.name,
                guardian.username,
                guardian.avatar_color,
                guardian.relation,
                guardian.status,
                if guardian.can_view_live_location { 1 } else { 0 },
                if guardian.can_receive_sos_alert { 1 } else { 0 },
                guardian.battery_percent as i64,
                guardian.last_latitude,
                guardian.last_longitude,
                guardian.updated_at as i64,
            ],
        )?;
        Ok(())
    }

    pub fn delete_guardian(&self, user_id: &Uuid, guardian_id: &str) -> Result<bool, StorageError> {
        let conn = self.conn.lock().unwrap();
        let affected = conn.execute(
            "DELETE FROM guardians WHERE id = ?1 AND user_id = ?2",
            params![guardian_id, user_id.to_string()],
        )?;
        Ok(affected > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_lifecycle_and_view_once() {
        let db = EncryptedDatabase::open_in_memory("test_passkey_123").expect("open db");
        let msg_id = Uuid::new_v4();
        let room_id = Uuid::new_v4();
        let sender = Uuid::new_v4();
        let recipient = Uuid::new_v4();

        let msg = LocalMessage {
            id: msg_id,
            room_id,
            sender_id: sender,
            recipient_id: recipient,
            content: "Foto Rahasia View-Once".to_string(),
            media_url: Some("media://secret.jpg".to_string()),
            msg_type: MessageType::Image,
            is_view_once: true,
            is_opened: false,
            opened_at: None,
            reply_to_id: None,
            is_deleted: false,
            is_silent_deleted: false,
            auto_delete_at: None,
            edited_at: None,
            reactions: HashMap::new(),
            is_encrypted: true,
            created_at: 1000,
            updated_at: 1000,
            status: DeliveryStatus::Queued,
        };

        db.insert_message(&msg).expect("insert");

        let fetched = db.get_message(&msg_id).expect("fetch").unwrap();
        assert_eq!(fetched.content, "Foto Rahasia View-Once");
        assert!(!fetched.is_opened);

        // Scrub view-once
        let marked = db.mark_as_viewed(&msg_id).expect("mark viewed");
        assert!(marked);

        let after_scrub = db.get_message(&msg_id).expect("fetch").unwrap();
        assert!(after_scrub.is_opened);
        assert_eq!(after_scrub.content, "🔒 Pesan sekali lihat telah dibuka");
    }

    #[test]
    fn test_disappearing_messages_purge() {
        let db = EncryptedDatabase::open_in_memory("test_passkey_123").expect("open db");
        let room_id = Uuid::new_v4();
        let now = 1000000;

        let msg1 = LocalMessage {
            id: Uuid::new_v4(),
            room_id,
            sender_id: Uuid::new_v4(),
            recipient_id: Uuid::new_v4(),
            content: "Pesan kedaluwarsa".to_string(),
            media_url: None,
            msg_type: MessageType::Text,
            is_view_once: false,
            is_opened: false,
            opened_at: None,
            reply_to_id: None,
            is_deleted: false,
            is_silent_deleted: false,
            auto_delete_at: Some(now - 100), // Sudah lewat
            edited_at: None,
            reactions: HashMap::new(),
            is_encrypted: true,
            created_at: now - 500,
            updated_at: now - 500,
            status: DeliveryStatus::Read,
        };

        let msg2 = LocalMessage {
            id: Uuid::new_v4(),
            room_id,
            sender_id: Uuid::new_v4(),
            recipient_id: Uuid::new_v4(),
            content: "Pesan masih aktif".to_string(),
            media_url: None,
            msg_type: MessageType::Text,
            is_view_once: false,
            is_opened: false,
            opened_at: None,
            reply_to_id: None,
            is_deleted: false,
            is_silent_deleted: false,
            auto_delete_at: Some(now + 3600000), // Masih 1 jam lagi
            edited_at: None,
            reactions: HashMap::new(),
            is_encrypted: true,
            created_at: now,
            updated_at: now,
            status: DeliveryStatus::Delivered,
        };

        db.insert_message(&msg1).unwrap();
        db.insert_message(&msg2).unwrap();

        let purged = db.purge_expired_messages(now).expect("purge");
        assert_eq!(purged, 1);

        let active_msgs = db.get_messages_for_room(&room_id, 10).unwrap();
        assert_eq!(active_msgs.len(), 1);
        assert_eq!(active_msgs[0].content, "Pesan masih aktif");
    }

    #[test]
    fn test_security_audit_logs_checksum() {
        let db = EncryptedDatabase::open_in_memory("test_passkey_123").expect("open db");
        let user_id = Uuid::new_v4();

        let log = db
            .insert_security_log(
                &user_id,
                None,
                "sos_started",
                Some(&serde_json::json!({"lat": -6.2088, "lng": 106.8456})),
            )
            .expect("insert log");

        assert_eq!(log.event_type, "sos_started");
        assert!(!log.checksum.is_empty());

        let logs = db.get_security_logs(&user_id).unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].checksum, log.checksum);
    }
}
