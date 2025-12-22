use crate::gateway_app::GatewayApp;
use actix_web::error::ErrorBadGateway;
use actix_web::{web, HttpRequest, HttpResponse};

macro_rules! route_map {
    (
        $path:expr,
        $( $prefix:literal -> $service:literal ),* $(,)?
    ) => {
        match $path {
            $(
                p if p.starts_with($prefix) => $service,
            )*
            _ => "NOT_FOUND",
        }
    };
}

async fn forward_request(
    req: &HttpRequest,
    payload: web::Payload,
    target: &str,
    session_token: Option<String>,
    validation: Option<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let method = req.method().clone();
    let client = awc::Client::new();
    let mut builder = client.request(method, target);

    for (header, value) in req.headers() {
        builder = builder.insert_header((header, value));
    }
    if let Some(token) = session_token {
        builder = builder.insert_header(("X-Session-Token", token));
    }
    if let Some(user_id) = validation {
        builder = builder.insert_header(("X-User-Id", user_id));
    }

    let mut upstream_resp = builder
        .send_stream(payload)
        .await
        .map_err(|e| ErrorBadGateway(format!("upstream send error: {e}")))?;
    let mut client_resp = HttpResponse::build(upstream_resp.status());

    for (header, value) in upstream_resp.headers() {
        client_resp.insert_header((header, value));
    }

    let body = upstream_resp.body().await?;
    Ok(client_resp.body(body))
}

pub async fn request_handler(
    app: web::Data<GatewayApp>,
    req: HttpRequest,
    body: web::Payload,
) -> HttpResponse {
    let path = req.path();

    let service = route_map!(path,
        "/login"   -> "http://auth-service:8080",
        "/logout"  -> "http://auth-service:8080",
        "/signup"  -> "http://auth-service:8080",
        "/account" -> "http://auth-service:8080",
        "/mcf"     -> "http://post-service:8080",
    );

    if service == "NOT_FOUND" {
        return HttpResponse::NotFound().finish();
    }

    let target = format!("{service}{path}");
    let session_token = req.cookie("session_token").map(|c| c.value().to_string());
    if path == "/logout" {
        let resp = forward_request(&req, body, &target, session_token.clone(), None)
            .await
            .unwrap_or_else(|_| HttpResponse::BadGateway().finish());

        // If logout succeeded, invalidate gateway cache
        if resp.status().is_success()
            && let Some(token) = session_token
        {
            app.invalidate_token(&token).await;
        }

        return resp;
    }

    let validation = if let Some(token) = session_token.clone() {
        match app.validate_token(token).await {
            Ok(v) => v,
            _ => return HttpResponse::BadGateway().finish(),
        }
    } else {
        None
    };

    forward_request(&req, body, &target, session_token, validation)
        .await
        .unwrap_or_else(|_| HttpResponse::BadGateway().finish())
}
