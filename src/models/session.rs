use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub session_token: String,
    pub last_used_at: DateTime<Local>,
    pub expires_at: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CreateAccountResult {
    pub account_id: Option<Uuid>,
    pub session_token: Option<String>,
}
