use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod routes;
mod handlers;
mod validation;
mod config;

use cleaner::database::Database;
use crate::routes::create_router;
use crate::config::Config;

// Shared database state
#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Database>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::default();

    // Initialize database
    let db = Database::new(&config.database_path).expect("Failed to initialize database");
    
    // Migrate rules from YAML if database is empty
    if let Err(e) = db.migrate_from_yaml() {
        tracing::error!("Failed to migrate rules from YAML: {}", e);
    } else {
        tracing::info!("Successfully migrated rules from YAML");
    }

    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    // Create router
    let app = create_router()
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    tracing::info!("🚀 Server running on http://{}", config.addr);
    tracing::info!(" Database: {} (shared with CLI/Discord)", config.database_path);

    let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}