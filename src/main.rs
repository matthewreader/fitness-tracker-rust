use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "fitness-tracker"
    })))
}
