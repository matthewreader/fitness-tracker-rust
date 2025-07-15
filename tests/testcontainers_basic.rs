use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use sqlx::{PgPool, Row};

#[tokio::test]
async fn test_postgres_with_testcontainers() {
    let postgres = Postgres::default();
    let container = postgres.start().await.unwrap();
    
    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    );
    
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to create database pool");
    
    // Test basic connection
    let result = sqlx::query("SELECT 1 as test_value")
        .fetch_one(&pool)
        .await
        .unwrap();
    
    let test_value: i32 = result.get("test_value");
    assert_eq!(test_value, 1);
    
    // Test table creation and data insertion
    sqlx::query(r#"
        CREATE TABLE test_users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(50) NOT NULL
        )
    "#)
    .execute(&pool)
    .await
    .unwrap();
    
    sqlx::query("INSERT INTO test_users (name) VALUES ($1)")
        .bind("Test User")
        .execute(&pool)
        .await
        .unwrap();
    
    let row = sqlx::query("SELECT name FROM test_users WHERE id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    
    let name: String = row.get("name");
    assert_eq!(name, "Test User");
}