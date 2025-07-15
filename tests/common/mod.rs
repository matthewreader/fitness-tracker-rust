use sqlx::PgPool;
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres;

pub struct TestDatabase {
    pub pool: PgPool,
    #[allow(dead_code)]
    container: ContainerAsync<Postgres>,
}

impl TestDatabase {
    pub async fn new() -> Self {
        let postgres = Postgres::default();
        let container = postgres.start().await.unwrap();
        
        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            container.get_host_port_ipv4(5432).await.unwrap()
        );
        
        let pool = PgPool::connect(&connection_string)
            .await
            .expect("Failed to create test database pool");
        
        Self { pool, container }
    }
    
    pub async fn setup_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                username VARCHAR(50) UNIQUE NOT NULL,
                email VARCHAR(100) UNIQUE NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
        "#)
        .execute(&self.pool)
        .await?;
        
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS workouts (
                id SERIAL PRIMARY KEY,
                user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
                workout_type VARCHAR(50) NOT NULL,
                duration_minutes INTEGER NOT NULL,
                calories_burned INTEGER,
                notes TEXT,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
        "#)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

// Helper function to create a test user
pub async fn create_test_user(pool: &PgPool, username: &str, email: &str) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar("INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id")
        .bind(username)
        .bind(email)
        .fetch_one(pool)
        .await
}

// Helper function to create a test workout
pub async fn create_test_workout(
    pool: &PgPool, 
    user_id: i32, 
    workout_type: &str, 
    duration: i32, 
    calories: Option<i32>
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar(
        "INSERT INTO workouts (user_id, workout_type, duration_minutes, calories_burned) 
         VALUES ($1, $2, $3, $4) RETURNING id"
    )
    .bind(user_id)
    .bind(workout_type)
    .bind(duration)
    .bind(calories)
    .fetch_one(pool)
    .await
}