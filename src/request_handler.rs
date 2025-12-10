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
    validation: Option<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let method = req.method().clone();
    let client = awc::Client::new();
    let mut builder = client.request(method, target);

    for (header, value) in req.headers() {
        builder = builder.insert_header((header, value));
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
        "/sign_in" -> "http://auth-service:8080",
        "/mcf"     -> "http://post-service:8080",
    );

    if service == "NOT_FOUND" {
        return HttpResponse::NotFound().finish();
    }

    let validation = match app.validate_token(&req).await {
        Ok(v) => v,
        _ => return HttpResponse::BadGateway().finish(),
    };

    let target = format!("{service}{path}");
    forward_request(&req, body, &target, validation)
        .await
        .unwrap_or_else(|_| HttpResponse::BadGateway().finish())
}
