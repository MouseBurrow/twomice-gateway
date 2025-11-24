use crate::models::account::User;
use crate::models::session::CreateAccountResult;
use sqlx::PgPool;

pub async fn create_account(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> Result<CreateAccountResult, sqlx::Error> {
    let result = sqlx::query_as!(
        CreateAccountResult,
        r#"SELECT account_id, session_token FROM auth.create_account($1, $2)"#,
        username,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}

pub async fn get_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT * FROM auth.users WHERE username = $1"#,
        username
    )
    .fetch_optional(pool)
    .await
}
