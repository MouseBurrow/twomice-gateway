use crate::config::Config;
use crate::database::account::{create_account, get_user_by_username};
use crate::models::account::User;
use crate::models::session::CreateAccountResult;
use crate::utils::password::hash_password;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod config;
mod database;
mod models;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config: Config = Config::init().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str())
        .await?;

    let username = "newShrimp";
    let password = "myTestPassword";
    let hash = hash_password(password).unwrap();

    let user: Option<User> = get_user_by_username(&pool, username).await?;
    if let Some(user) = user {
        println!("{}", user);
    } else {
        println!("User not found, creating a new one");
        let result: CreateAccountResult = create_account(&pool, username, password).await?;
        println!("{:?}", result);
    }

    Ok(())
}
