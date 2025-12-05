use crate::db::errors::AuthError;
use crate::services::password_service::verify_password;
use burrow_db::db_call;
use sqlx::PgPool;
use uuid::Uuid;

pub struct AuthHandler {}

impl AuthHandler {
    /// Tries to create an account in the database.
    ///
    /// # Arguments
    /// * `pool` - A reference to a database connection pool.
    /// * `username` - The username of the new account.
    /// * `password_hash` - The password hash of the new account.
    ///
    /// # Returns
    /// A tuple containing the account ID and session token.
    ///
    /// # Errors
    /// * `AuthError::UsernameExists` - An error indicating that the account with the given username already exists.
    /// * `AuthError::Db` - If there was an unexpected error!
    pub async fn create_account(
        pool: &PgPool,
        username: &str,
        password_hash: &str,
    ) -> Result<(Uuid, String), AuthError> {
        db_call!(
            pool   = pool,
            query  = sqlx::query_as(r#"SELECT account_id, session_token FROM create_account($1, $2)"#),
            binds  = [username, password_hash],
            errors = {
                "23505" => AuthError::UsernameExists
            },
            fallback = AuthError::Db
        )
    }

    /// Tries to login an account in the database using a session token
    ///
    /// # Arguments
    /// * `pool` - A reference to a database connection pool
    /// * `token` - The session token to log in with
    ///
    /// # Returns
    /// The associated account ID.
    ///
    /// # Errors
    /// * `AuthError::TokenInvalid` - If the session token is invalid.
    /// * `AuthError::SessionExpired` - If the session token has expired.
    /// * `AuthError::Db` - If there was an unexpected error!
    pub async fn login_with_token(pool: &PgPool, token: &str) -> Result<Uuid, AuthError> {
        db_call!(
            pool   = pool,
            query  = sqlx::query_scalar(r#"SELECT login_with_token($1)"#),
            binds  = [token],
            errors = {
                "P1001" => AuthError::TokenInvalid,
                "P1002" => AuthError::SessionExpired
            },
            fallback = AuthError::Db
        )
    }

    /// Tries to login an account in the database.
    ///
    /// # Arguments
    /// * `pool` - A reference to a database connection pool
    /// * `username` - The username of the account to login.
    /// * `password` - The password of the account to login
    ///
    /// # Returns
    /// A tuple containing the account ID and session token.
    ///
    /// # Errors
    /// * `AuthError::UserNotFound` - If the username does not exist in the database.
    /// * `AuthError::InvalidPassword` - If the given password hash does not match the stored hash.
    /// * `AuthError::Db` - If there was an unexpected error!
    pub async fn login_account(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<(Uuid, String), AuthError> {
        let stored_hash: String = db_call!(
            pool   = pool,
            query  = sqlx::query_scalar(r#"SELECT get_password_hash($1)"#),
            binds  = [username],
            errors = {
                "P2001" => AuthError::UserNotFound
            },
            fallback = AuthError::Db
        )?;

        if verify_password(password, stored_hash).is_err() {
            return Err(AuthError::InvalidPassword);
        }

        let user_id: Uuid = db_call!(
            pool = pool,
            query = sqlx::query_scalar(r#"SELECT id FROM accounts WHERE username=$1"#),
            binds = [username],
            fallback = AuthError::Db
        )?;

        let session_token: String = db_call!(
            pool = pool,
            query = sqlx::query_scalar(r#"SELECT create_session($1)"#),
            binds = [user_id],
            fallback = AuthError::Db
        )?;

        Ok((user_id, session_token))
    }

    /// Logout a user's session.
    ///
    /// # Parameters
    /// * `pool` - A reference to the database connection pool
    /// * `session_token` - The user's session token
    ///
    /// # Returns
    /// * `Ok(true)` if the user's session was successfully deleted, `Ok(false)` otherwise
    ///
    /// # Errors
    /// * `AuthError::Db` - If there was an unexpected error!
    pub async fn logout_session(pool: &PgPool, session_token: String) -> Result<bool, AuthError> {
        db_call!(
            pool = pool,
            query = sqlx::query_scalar(r#"SELECT logout_session($1)"#),
            binds = [session_token],
            fallback = AuthError::Db
        )
    }
}
