#[derive(Debug)]
pub enum AuthError {
    UsernameExists,
    SessionExpired,
    TokenInvalid,
    UserNotFound,
    InvalidPassword,
    Db(sqlx::Error),
}
