use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "service": "fitness-tracker"
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_returns_ok_status() {
        let (status, json_response) = health_check().await;
        
        assert_eq!(status, StatusCode::OK);
        
        let response_value = json_response.0;
        assert_eq!(response_value["status"], "healthy");
        assert_eq!(response_value["service"], "fitness-tracker");
        assert!(response_value["timestamp"].is_string());
    }
}