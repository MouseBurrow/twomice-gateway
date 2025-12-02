use chrono::{DateTime, Local, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

fn format_date(time: Option<DateTime<Utc>>) -> ColoredString {
    match time {
        Some(dt) => dt
            .with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .as_str()
            .bright_black(),
        None => "[None]".bright_black(),
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = 41;
        let wall = "║".cyan().bold();
        let bottom = format!("╚{}╝", "═".repeat(width)).as_str().cyan().bold();

        let is_admin = if self.is_admin {
            "True".green()
        } else {
            "False".red()
        };

        writeln!(
            f,
            "{} {} {}",
            "╔═".cyan().bold(),
            self.username.as_str().cyan().bold(),
            format!("{}╗", "═".repeat(width - self.username.len() - 3))
                .as_str()
                .cyan()
                .bold(),
        )?;
        // writeln!(f, "{} Username: {}", wall, self.username.as_str().bold())?;
        writeln!(f, "{} Is Admin:           {}", wall, is_admin)?;
        writeln!(
            f,
            "{} Created At:         {}",
            wall,
            format_date(self.created_at)
        )?;
        writeln!(
            f,
            "{} Last Updated At:    {}",
            wall,
            format_date(self.updated_at)
        )?;
        writeln!(f, "{}", bottom)
    }
}
