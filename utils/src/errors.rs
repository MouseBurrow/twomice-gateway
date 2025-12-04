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

pub enum PostError {
    UserNotFound,
    Db(sqlx::Error),
}
