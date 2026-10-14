//! # mekaar-server: SQLite Database Layer (WAL Mode)
//!
//! Penyimpanan lokal persisten berbasis SQLite untuk mekaar-server di teplok-pc:
//! - Mode WAL (`PRAGMA journal_mode=WAL`) untuk latensi rendah & konkurensi tinggi.
//! - Direktori Kunci Publik (`public_keys`)
//! - Antrean Pesan Keluar Offline (`offline_messages`)
//! - Relasi Guardian (`guardians`)
//! - Backup Kunci Privat Terenkripsi Klien (`key_backups`)

use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("SQLite database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Format data database tidak valid: {0}")]
    InvalidData(String),
}

/// Handler koneksi thread-safe database SQLite lokal server.
#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    /// Membuka file database SQLite dan menerapkan migrasi schema awal.
    pub fn open(path: &str) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init()?;
        Ok(db)
    }

    /// Membuka in-memory database untuk keperluan testing.
    pub fn open_in_memory() -> Result<Self, DbError> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.init()?;
        Ok(db)
    }

    /// Inisialisasi PRAGMA WAL dan pembuatan tabel-tabel utama.
    fn init(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();

        // Aktifkan mode WAL untuk performa baca/tulis konkurensi tinggi
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;",
        )?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS public_keys (
                user_id TEXT PRIMARY KEY,
                public_key BLOB NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS offline_messages (
                id TEXT PRIMARY KEY,
                recipient_id TEXT NOT NULL,
                sender_id TEXT NOT NULL,
                envelope_blob BLOB NOT NULL,
                created_at INTEGER NOT NULL,
                expires_at INTEGER
            );
            CREATE INDEX IF NOT EXISTS idx_offline_recipient ON offline_messages(recipient_id);

            CREATE TABLE IF NOT EXISTS guardians (
                victim_id TEXT NOT NULL,
                guardian_id TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                PRIMARY KEY (victim_id, guardian_id)
            );
            CREATE INDEX IF NOT EXISTS idx_guardians_victim ON guardians(victim_id);

            CREATE TABLE IF NOT EXISTS key_backups (
                user_id TEXT PRIMARY KEY,
                backup_json TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );",
        )?;

        Ok(())
    }

    // ── Public Keys Directory ──────────────────────────────────────────

    /// Mendaftarkan atau memperbarui kunci publik X25519 pengguna.
    pub fn upsert_public_key(&self, user_id: &Uuid, public_key: &[u8; 32]) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        conn.execute(
            "INSERT INTO public_keys (user_id, public_key, updated_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(user_id) DO UPDATE SET
                public_key = excluded.public_key,
                updated_at = excluded.updated_at",
            params![user_id.to_string(), public_key.as_slice(), now],
        )?;

        Ok(())
    }

    /// Mengambil kunci publik pengguna dari direktori.
    pub fn get_public_key(&self, user_id: &Uuid) -> Result<Option<[u8; 32]>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT public_key FROM public_keys WHERE user_id = ?1")?;

        let mut rows = stmt.query(params![user_id.to_string()])?;
        if let Some(row) = rows.next()? {
            let blob: Vec<u8> = row.get(0)?;
            if blob.len() == 32 {
                let mut key = [0u8; 32];
                key.copy_from_slice(&blob);
                return Ok(Some(key));
            }
        }

        Ok(None)
    }

    // ── Offline Store & Forward Outbox ─────────────────────────────────

    /// Menyimpan paket ciphertext envelope saat penerima sedang offline.
    pub fn store_offline_message(
        &self,
        message_id: &Uuid,
        recipient_id: &Uuid,
        sender_id: &Uuid,
        envelope_blob: &[u8],
        expires_at: Option<u64>,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        conn.execute(
            "INSERT INTO offline_messages (id, recipient_id, sender_id, envelope_blob, created_at, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO NOTHING",
            params![
                message_id.to_string(),
                recipient_id.to_string(),
                sender_id.to_string(),
                envelope_blob,
                now,
                expires_at.map(|e| e as i64)
            ],
        )?;

        Ok(())
    }

    /// Mengambil seluruh pesan offline yang tertunda untuk seorang penerima.
    pub fn get_offline_messages(&self, recipient_id: &Uuid) -> Result<Vec<(Uuid, Vec<u8>)>, DbError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        let mut stmt = conn.prepare(
            "SELECT id, envelope_blob FROM offline_messages
             WHERE recipient_id = ?1 AND (expires_at IS NULL OR expires_at > ?2)
             ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map(params![recipient_id.to_string(), now], |row| {
            let id_str: String = row.get(0)?;
            let blob: Vec<u8> = row.get(1)?;
            Ok((id_str, blob))
        })?;

        let mut messages = Vec::new();
        for item in rows {
            let (id_str, blob) = item?;
            if let Ok(id) = Uuid::parse_str(&id_str) {
                messages.push((id, blob));
            }
        }

        Ok(messages)
    }

    /// Menghapus pesan offline setelah penerima mengirimkan DeliveryAck.
    pub fn delete_offline_message(&self, message_id: &Uuid) -> Result<bool, DbError> {
        let conn = self.conn.lock().unwrap();
        let rows = conn.execute(
            "DELETE FROM offline_messages WHERE id = ?1",
            params![message_id.to_string()],
        )?;
        Ok(rows > 0)
    }

    // ── Guardians Management ───────────────────────────────────────────

    /// Menambahkan hubungan pengawasan Guardian.
    pub fn add_guardian(&self, victim_id: &Uuid, guardian_id: &Uuid) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        conn.execute(
            "INSERT INTO guardians (victim_id, guardian_id, created_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(victim_id, guardian_id) DO NOTHING",
            params![victim_id.to_string(), guardian_id.to_string(), now],
        )?;

        Ok(())
    }

    /// Menghapus hubungan pengawasan Guardian.
    pub fn remove_guardian(&self, victim_id: &Uuid, guardian_id: &Uuid) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM guardians WHERE victim_id = ?1 AND guardian_id = ?2",
            params![victim_id.to_string(), guardian_id.to_string()],
        )?;
        Ok(())
    }

    /// Mengambil seluruh guardian terdaftar untuk seorang korban.
    pub fn get_guardians_for_victim(&self, victim_id: &Uuid) -> Result<Vec<Uuid>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT guardian_id FROM guardians WHERE victim_id = ?1")?;

        let rows = stmt.query_map(params![victim_id.to_string()], |row| {
            let gid: String = row.get(0)?;
            Ok(gid)
        })?;

        let mut guardians = Vec::new();
        for item in rows {
            let gid_str = item?;
            if let Ok(gid) = Uuid::parse_str(&gid_str) {
                guardians.push(gid);
            }
        }

        Ok(guardians)
    }

    // ── Zero-Knowledge Key Backups ─────────────────────────────────────

    /// Menyimpan blob backup kunci privat terenkripsi Argon2id milik pengguna.
    pub fn upsert_key_backup(&self, user_id: &Uuid, backup_json: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        conn.execute(
            "INSERT INTO key_backups (user_id, backup_json, updated_at)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(user_id) DO UPDATE SET
                backup_json = excluded.backup_json,
                updated_at = excluded.updated_at",
            params![user_id.to_string(), backup_json, now],
        )?;

        Ok(())
    }

    /// Mengambil blob backup kunci terenkripsi milik pengguna.
    pub fn get_key_backup(&self, user_id: &Uuid) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT backup_json FROM key_backups WHERE user_id = ?1")?;

        let mut rows = stmt.query(params![user_id.to_string()])?;
        if let Some(row) = rows.next()? {
            let json: String = row.get(0)?;
            return Ok(Some(json));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_key_upsert_and_fetch() {
        let db = Database::open_in_memory().expect("open db");
        let user_id = Uuid::new_v4();
        let sample_key = [0x55; 32];

        assert_eq!(db.get_public_key(&user_id).unwrap(), None);

        db.upsert_public_key(&user_id, &sample_key).unwrap();
        assert_eq!(db.get_public_key(&user_id).unwrap(), Some(sample_key));

        // Update key
        let updated_key = [0x77; 32];
        db.upsert_public_key(&user_id, &updated_key).unwrap();
        assert_eq!(db.get_public_key(&user_id).unwrap(), Some(updated_key));
    }

    #[test]
    fn test_offline_messages_lifecycle() {
        let db = Database::open_in_memory().expect("open db");
        let msg_id = Uuid::new_v4();
        let recipient = Uuid::new_v4();
        let sender = Uuid::new_v4();
        let payload = vec![1, 2, 3, 4, 5];

        db.store_offline_message(&msg_id, &recipient, &sender, &payload, None)
            .unwrap();

        let pending = db.get_offline_messages(&recipient).unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].0, msg_id);
        assert_eq!(pending[0].1, payload);

        // Delete upon delivery ack
        let deleted = db.delete_offline_message(&msg_id).unwrap();
        assert!(deleted);
        assert_eq!(db.get_offline_messages(&recipient).unwrap().len(), 0);
    }

    #[test]
    fn test_guardians_management() {
        let db = Database::open_in_memory().expect("open db");
        let victim = Uuid::new_v4();
        let g1 = Uuid::new_v4();
        let g2 = Uuid::new_v4();

        db.add_guardian(&victim, &g1).unwrap();
        db.add_guardian(&victim, &g2).unwrap();

        let list = db.get_guardians_for_victim(&victim).unwrap();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&g1));
        assert!(list.contains(&g2));

        db.remove_guardian(&victim, &g1).unwrap();
        let list_after = db.get_guardians_for_victim(&victim).unwrap();
        assert_eq!(list_after.len(), 1);
        assert_eq!(list_after[0], g2);
    }

    #[test]
    fn test_key_backup_storage() {
        let db = Database::open_in_memory().expect("open db");
        let user = Uuid::new_v4();
        let backup_json = r#"{"v":2,"kdf":"argon2id","salt":"...","ct":"..."}"#;

        assert_eq!(db.get_key_backup(&user).unwrap(), None);

        db.upsert_key_backup(&user, backup_json).unwrap();
        assert_eq!(db.get_key_backup(&user).unwrap(), Some(backup_json.to_string()));
    }
}
