use crate::gateway_app::GatewayApp;
use actix_web::error::ErrorBadGateway;
use actix_web::{web, HttpRequest, HttpResponse};

fn route_url(path: &str, auth_url: &str, post_url: &str) -> Option<String> {
    let base = if path.starts_with("/login")
        || path.starts_with("/logout")
        || path.starts_with("/signup")
        || path.starts_with("/account")
    {
        Some(auth_url)
    } else if path.starts_with("/mcf") {
        Some(post_url)
    } else {
        None
    };
    base.map(|b| format!("{b}{path}"))
}

async fn forward_request(
    req: &HttpRequest,
    payload: web::Payload,
    app: &GatewayApp,
    target: &str,
    session_token: Option<String>,
    validation: Option<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let method = req.method().clone();
    let mut builder = app.client.request(method, target);

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
        let header_name = header.as_str().to_lowercase();
        if header_name.starts_with("x-") && header_name != "x-session-token" && header_name != "x-user-id" {
            continue;
        }
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

    let target = match route_url(path, &app.auth_service_url, &app.post_service_url) {
        Some(t) => t,
        None => return HttpResponse::NotFound().finish(),
    };

    let session_token = req.cookie("session_token").map(|c| c.value().to_string());
    if path == "/logout" {
        let resp = forward_request(&req, body, &app, &target, session_token.clone(), None)
            .await
            .unwrap_or_else(|_| HttpResponse::BadGateway().finish());

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

    forward_request(&req, body, &app, &target, session_token, validation)
        .await
        .unwrap_or_else(|_| HttpResponse::BadGateway().finish())
}
