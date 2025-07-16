mod routes;
mod database;
#[cfg(test)]
mod test_helpers;

use tokio::net::TcpListener;
use crate::routes::health_check::health_check;
use crate::database::{create_pool, run_migrations};

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let pool = create_pool().await.expect("Failed to create database pool");
    
    // Run migrations
    run_migrations(&pool).await.expect("Failed to run migrations");
    
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}


