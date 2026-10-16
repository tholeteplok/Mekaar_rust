//! # mekaar-server — Standalone Blind Relay Server Library
//!
//! Logika inti router dan state untuk server mandiri Axum di teplok-pc.

pub mod db;
pub mod handlers;
pub mod hub;

use axum::routing::{get, post};
use axum::{extract::DefaultBodyLimit, Router};
use hub::Hub;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Membuat Axum Router lengkap dengan seluruh route REST & WebSocket.
pub fn create_router(hub: Arc<Hub>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        // Public key directory
        .route("/keys", post(handlers::upsert_public_key))
        .route("/keys/{user_id}", get(handlers::get_public_key))
        // Zero-knowledge key backups
        .route("/backups", post(handlers::upsert_key_backup))
        .route("/backups/{user_id}", get(handlers::get_key_backup))
        // Central Directory Auth (Mekaar_chat parity)
        .route("/auth/register", post(handlers::register_account))
        .route("/auth/login", post(handlers::login_account))
        .route("/auth/resolve-email", post(handlers::resolve_email))
        .route("/auth/reset-password", post(handlers::reset_password))
        // Guardian relationships
        .route("/guardians", post(handlers::add_guardian))
        .route("/guardians/{victim_id}", get(handlers::get_guardians))
        // WebSocket duplex stream
        .route("/ws", get(handlers::ws_handler))
        // Middleware: batasi body 64KiB agar flood payload langsung ditolak (413).
        .layer(DefaultBodyLimit::max(64 * 1024))
        // CORS ketat: hanya Tailscale + loopback. Jangan permissive().
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://127.0.0.1:5173".parse().unwrap(),
                    "http://localhost:5173".parse().unwrap(),
                    "http://100.101.96.120:8085".parse().unwrap(),
                ])
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(hub)
}
