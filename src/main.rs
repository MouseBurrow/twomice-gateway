use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/ping")]
async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new(|| App::new().service(ping))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}
