use crate::models::User;
use sqlx::PgPool;

pub async fn create_user(
    pool: PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        username,
        email,
        password_hash
    )
    .fetch_one(&pool)
    .await?;

    Ok(result)
}

pub async fn get_users_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        email
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

