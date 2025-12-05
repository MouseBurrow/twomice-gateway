use actix_web::{App, HttpServer};
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod db;
mod models;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // If it is dev mode, load env variables from .env
    #[cfg(debug_assertions)]
    Config::load_local_env();

    // Connect to the database
    let config: Config = Config::init("AUTH");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url.as_str())
        .await?;

    // Start listening endpoint
    let port = env::var("PORT")?;
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting Gateway at http://{}", addr);
    HttpServer::new(App::new)
        .bind(addr)?
        .run()
        .await
        .expect("Could not connect to network");

    Ok(())

    // TEST

    // let username = "newShrimp";
    // let password = "myTestPassword";
    // let password_hash = hash_password(password).unwrap();
    //
    // match AuthHandler::login_account(&pool, username, password).await {
    //     Ok((user_id, token)) => {
    //         println!("Login attempt successful!");
    //         println!("User ID : {}", user_id);
    //         println!("Token   : {}", token);
    //     }
    //     Err(e) => match e {
    //         AuthError::InvalidPassword => {
    //             println!("Invalid password");
    //         }
    //         AuthError::UserNotFound => {
    //             println!("User with that name not found, creating a new account");
    //
    //             match AuthHandler::create_account(&pool, username, password_hash.as_str()).await {
    //                 Ok((user_id, token)) => {
    //                     println!("Account created successfully!");
    //                     println!("User ID : {}", user_id);
    //                     println!("Token   : {}", token);
    //                 }
    //                 Err(e) => match e {
    //                     AuthError::UsernameExists => {
    //                         println!("Account already exists!");
    //                     }
    //                     AuthError::Db(err) => {
    //                         println!("Unexpected database error: {}", err);
    //                     }
    //                     _ => (),
    //                 },
    //             }
    //         }
    //         AuthError::Db(err) => {
    //             println!("Unexpected database error: {}", err);
    //         }
    //         _ => (),
    //     },
    // }

    // Ok(())
}
