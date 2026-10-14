//! # mekaar-server: Standalone Blind Relay Server (teplok-pc)
//!
//! Binary entrypoint untuk relay server mandiri di teplok-pc.
//! Berjalan 24/7 di jaringan privat Tailscale.

use mekaar_server::db::Database;
use mekaar_server::hub::Hub;
use mekaar_server::create_router;
use std::env;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Inisialisasi logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("mekaar_server=info,tower_http=info")),
        )
        .init();

    tracing::info!("🦀 Memulai mekaar-server v{}...", env!("CARGO_PKG_VERSION"));
    tracing::info!("   Mode: Zero-Knowledge Blind Relay (No Decryption Keys on Server)");
    tracing::info!("   Target Host: teplok-pc via Tailscale Private Mesh");

    // 2. Konfigurasi path database & binding
    let db_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "./mekaar_server.db".to_string());
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8085".to_string());
    let bind_addr = format!("{}:{}", host, port);

    // 3. Buka database SQLite lokal dengan mode WAL
    tracing::info!("📂 Membuka database SQLite WAL di: {}", db_path);
    let db = Database::open(&db_path)?;
    let hub = Arc::new(Hub::new(Arc::new(db)));

    // 4. Inisialisasi Router
    let app = create_router(hub);

    // 5. Jalankan server TCP listener
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    tracing::info!("🚀 mekaar-server siap melayani di http://{}", bind_addr);
    tracing::info!("   WebSocket Endpoint: ws://{}/ws", bind_addr);
    tracing::info!("   Health Endpoint:    http://{}/health", bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
