use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Runtime;

pub async fn init_db() -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/mydb")
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
