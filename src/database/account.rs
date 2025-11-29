use crate::utils::password::verify_password;
use sqlx::PgPool;
use uuid::Uuid;

#[macro_export]
macro_rules! db_call {
    // -----------------------------------
    // Handle no bindings, no errors
    // -----------------------------------
    (
        pool   = $pool:expr,
        query  = $query:expr,
    ) => {{
        db_call!(
            pool   = $pool,
            query  = $query,
            binds  = [],
            errors = {}
        )
    }};

    // -----------------------------------
    // Handle no errors
    // -----------------------------------
    (
        pool   = $pool:expr,
        query  = $query:expr,
        binds  = [$($param:expr),* $(,)?]
    ) => {{
        db_call!(
            pool   = $pool,
            query  = $query,
            binds  = [$($param),*],
            errors = {}
        )
    }};

    // -----------------------------------
    // Handle no bindings
    // -----------------------------------
    (
        pool   = $pool:expr,
        query  = $query:expr,
        errors = { $( $code:expr => $variant:expr ),* $(,)? }
    ) => {{
        db_call!(
            pool   = $pool,
            query  = $query,
            binds  = [],
            errors = { $( $code => $variant ),* }
        )
    }};

    // -----------------------------------
    // Main solution with every parameter
    // -----------------------------------
    (
        pool   = $pool:expr,
        query  = $query:expr,
        binds  = [$($param:expr),* $(,)?],
        errors = { $( $code:expr => $variant:expr ),* $(,)? }
    ) => {{
        // Handle the bindings
        let mut query = $query;
        $( query = query.bind($param); )*

        // Handle the fetching and error mapping
        query.fetch_one($pool)
            .await
            .map_err(|err: sqlx::Error| {
            if let sqlx::Error::Database(db_err) = &err {
                // If any of the input errors match return that
                match db_err.code().as_deref() {
                    $(
                        Some($code) => { return $variant },
                    )*
                    _ => {}
                }
            }
            // If nothing fits, return textual error
            AuthError::Db(err)
        })
    }};
}

#[derive(Debug)]
pub enum AuthError {
    UsernameExists,
    SessionExpired,
    TokenInvalid,
    UserNotFound,
    InvalidPassword,
    QueryFailed,
    Db(sqlx::Error),
}

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
pub async fn create_account(
    pool: &PgPool,
    username: &str,
    password_hash: &str,
) -> Result<(Uuid, String), AuthError> {
    db_call!(
        pool   = pool,
        query  = sqlx::query_as(r#"SELECT account_id, session_token FROM auth.create_account($1, $2)"#),
        binds  = [username, password_hash],
        errors = {
            "23505" => AuthError::UsernameExists
        }
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
pub async fn login_with_token(pool: &PgPool, token: &str) -> Result<Uuid, AuthError> {
    db_call!(
        pool   = pool,
        query  = sqlx::query_scalar(r#"SELECT auth.login_with_token($1)"#),
        binds  = [token],
        errors = {
            "P1001" => AuthError::TokenInvalid,
            "P1002" => AuthError::SessionExpired
        }
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
/// * `AuthError::Db` - If there was an unexpected error!
/// * `AuthError::UserNotFound` - If the username does not exist in the database.
/// * `AuthError::InvalidPassword` - If the given password hash does not match the stored hash.
pub async fn login_account(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<(Uuid, String), AuthError> {
    let stored_hash: String = db_call!(
        pool   = pool,
        query  = sqlx::query_scalar(r#"SELECT auth.get_password_hash($1)"#),
        binds  = [username],
        errors = {
            "P2001" => AuthError::UserNotFound
        }
    )?;

    if verify_password(password, stored_hash).is_err() {
        return Err(AuthError::InvalidPassword);
    }

    let user_id: Uuid = db_call!(
        pool = pool,
        query = sqlx::query_scalar(r#"SELECT id FROM auth.accounts WHERE username=$1"#),
        binds = [username]
    )?;

    let session_token: String = db_call!(
        pool = pool,
        query = sqlx::query_scalar(r#"SELECT auth.create_session($1)"#),
        binds = [user_id]
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
        query = sqlx::query_scalar(r#"SELECT auth.logout_session($1)"#),
        binds = [session_token]
    )
}
