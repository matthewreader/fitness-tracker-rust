mod common;

use common::{TestDatabase, create_test_user, create_test_workout};
use sqlx::Row;

#[tokio::test]
async fn test_user_workout_flow() {
    let test_db = TestDatabase::new().await;
    test_db.setup_tables().await.unwrap();
    
    // Create a user
    let user_id = create_test_user(&test_db.pool, "john_doe", "john@example.com")
        .await
        .unwrap();
    
    // Create multiple workouts for the user
    let workouts = vec![
        ("Running", 30, Some(300)),
        ("Cycling", 45, Some(400)),
        ("Swimming", 60, Some(500)),
    ];
    
    for (workout_type, duration, calories) in workouts {
        create_test_workout(&test_db.pool, user_id, workout_type, duration, calories)
            .await
            .unwrap();
    }
    
    // Query user's total workout stats
    let stats = sqlx::query(
        "SELECT 
            COUNT(*) as workout_count,
            SUM(duration_minutes) as total_duration,
            SUM(calories_burned) as total_calories
         FROM workouts 
         WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(&test_db.pool)
    .await
    .unwrap();
    
    assert_eq!(stats.get::<i64, _>("workout_count"), 3);
    assert_eq!(stats.get::<i64, _>("total_duration"), 135);
    assert_eq!(stats.get::<i64, _>("total_calories"), 1200);
}

#[tokio::test]
async fn test_user_deletion_cascade() {
    let test_db = TestDatabase::new().await;
    test_db.setup_tables().await.unwrap();
    
    // Create user and workout
    let user_id = create_test_user(&test_db.pool, "temp_user", "temp@example.com")
        .await
        .unwrap();
    
    create_test_workout(&test_db.pool, user_id, "Running", 30, Some(300))
        .await
        .unwrap();
    
    // Verify workout exists
    let workout_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM workouts WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&test_db.pool)
        .await
        .unwrap();
    assert_eq!(workout_count, 1);
    
    // Delete user
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(&test_db.pool)
        .await
        .unwrap();
    
    // Verify workouts were cascaded
    let workout_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM workouts WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&test_db.pool)
        .await
        .unwrap();
    assert_eq!(workout_count, 0);
}