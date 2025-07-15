mod routes;
#[cfg(test)]
mod test_helpers;
use tokio::net::TcpListener;
use crate::routes::health_check::health_check;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()        .route("/health", get(health_check));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}


