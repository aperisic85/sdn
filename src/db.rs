use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Runtime;
use std::env;

pub async fn init_db() -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to database");

    pool
}

pub async fn run_migrations(pool: &PgPool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations");
}
