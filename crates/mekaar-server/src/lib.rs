//! # mekaar-server — Standalone Blind Relay Server Library
//!
//! Logika inti router dan state untuk server mandiri Axum di teplok-pc.

pub mod db;
pub mod handlers;
pub mod hub;

use axum::routing::{get, post};
use axum::Router;
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
        // Guardian relationships
        .route("/guardians", post(handlers::add_guardian))
        .route("/guardians/{victim_id}", get(handlers::get_guardians))
        // WebSocket duplex stream
        .route("/ws", get(handlers::ws_handler))
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(hub)
}
