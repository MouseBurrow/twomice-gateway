use crate::db::auth::AuthHandler;
use crate::services::password_service::hash_password;
use config::{env_dir, Config};
use sqlx::postgres::PgPoolOptions;
use utils::errors::AuthError;

mod db;
mod models;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Config::load_env(env_dir!())?;

    let config: Config = Config::init().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str())
        .await?;

    // TEST

    let username = "newShrimp";
    let password = "myTestPassword";
    let password_hash = hash_password(password).unwrap();

    match AuthHandler::login_account(&pool, username, password).await {
        Ok((user_id, token)) => {
            println!("Login attempt successful!");
            println!("User ID : {}", user_id);
            println!("Token   : {}", token);
        }
        Err(e) => match e {
            AuthError::InvalidPassword => {
                println!("Invalid password");
            }
            AuthError::UserNotFound => {
                println!("User with that name not found, creating a new account");

                match AuthHandler::create_account(&pool, username, password_hash.as_str()).await {
                    Ok((user_id, token)) => {
                        println!("Account created successfully!");
                        println!("User ID : {}", user_id);
                        println!("Token   : {}", token);
                    }
                    Err(e) => match e {
                        AuthError::UsernameExists => {
                            println!("Account already exists!");
                        }
                        AuthError::Db(err) => {
                            println!("Unexpected database error: {}", err);
                        }
                        _ => (),
                    },
                }
            }
            AuthError::Db(err) => {
                println!("Unexpected database error: {}", err);
            }
            _ => (),
        },
    }

    Ok(())
}
