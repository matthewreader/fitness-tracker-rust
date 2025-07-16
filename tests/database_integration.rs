mod common;

use common::TestDatabase;
use sqlx::Row;

#[tokio::test]
async fn test_database_with_helper() {
    let test_db = TestDatabase::new().await;
    test_db.setup_tables().await.unwrap();
    
    // Test user operations
    let user_id: i32 = sqlx::query_scalar(
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id"
    )
    .bind("testuser")
    .bind("test@example.com")
    .fetch_one(&test_db.pool)
    .await
    .unwrap();
    
    // Test workout operations
    let _workout_id: i32 = sqlx::query_scalar(
        "INSERT INTO workouts (user_id, workout_type, duration_minutes, calories_burned, notes) 
         VALUES ($1, $2, $3, $4, $5) RETURNING id"
    )
    .bind(user_id)
    .bind("Running")
    .bind(30)
    .bind(300)
    .bind("Morning run")
    .fetch_one(&test_db.pool)
    .await
    .unwrap();
    
    // Verify data
    let row = sqlx::query(
        "SELECT u.username, w.workout_type, w.duration_minutes, w.calories_burned 
         FROM users u JOIN workouts w ON u.id = w.user_id 
         WHERE u.id = $1"
    )
    .bind(user_id)
    .fetch_one(&test_db.pool)
    .await
    .unwrap();
    
    assert_eq!(row.get::<String, _>("username"), "testuser");
    assert_eq!(row.get::<String, _>("workout_type"), "Running");
    assert_eq!(row.get::<i32, _>("duration_minutes"), 30);
    assert_eq!(row.get::<Option<i32>, _>("calories_burned"), Some(300));
}

#[tokio::test]
async fn test_multiple_users() {
    let test_db = TestDatabase::new().await;
    test_db.setup_tables().await.unwrap();
    
    // Insert multiple users
    let users = vec![
        ("alice", "alice@example.com"),
        ("bob", "bob@example.com"),
        ("charlie", "charlie@example.com"),
    ];
    
    for (username, email) in users {
        sqlx::query("INSERT INTO users (username, email) VALUES ($1, $2)")
            .bind(username)
            .bind(email)
            .execute(&test_db.pool)
            .await
            .unwrap();
    }
    
    // Verify count
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&test_db.pool)
        .await
        .unwrap();
    
    assert_eq!(count, 3);
}