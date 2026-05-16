use crate::gateway_app::{extract_session_token_from_headers, GatewayApp};
use axum::body::Bytes;
use axum::extract::Request;
use axum::http::{HeaderName, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Extension;

fn route_url(path: &str, app: &GatewayApp) -> Option<String> {
    let base = if path.starts_with("/login")
        || path.starts_with("/logout")
        || path.starts_with("/signup")
        || path.starts_with("/account")
    {
        Some(&app.auth_service_url)
    } else if path.starts_with("/mcf") {
        Some(&app.post_service_url)
    } else if path.starts_with("/moderation") {
        Some(&app.moderation_service_url)
    } else if path.starts_with("/social") {
        Some(&app.social_service_url)
    } else if path.starts_with("/feed") {
        Some(&app.feed_service_url)
    } else {
        None
    };
    base.map(|b| format!("{b}{path}"))
}

async fn forward_request(
    app: &GatewayApp,
    target: &str,
    method: http::Method,
    headers: &http::HeaderMap,
    body: Bytes,
    session_token: Option<String>,
    validation: Option<String>,
) -> Result<Response, Response> {
    let mut req_builder = app.client.request(method.clone(), target);

    for (name, value) in headers {
        let name_str = name.as_str().to_lowercase();
        if name_str == "host" || name_str == "content-length" {
            continue;
        }
        req_builder = req_builder.header(name.clone(), value.clone());
    }

    if let Some(ref token) = session_token {
        req_builder = req_builder.header("X-Session-Token", token);
    }
    if let Some(ref user_id) = validation {
        req_builder = req_builder.header("X-User-Id", user_id);
    }

    req_builder = req_builder.body(body.to_vec());

    let upstream_resp = req_builder
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("upstream error: {e}")).into_response())?;

    let status = upstream_resp.status();
    let upstream_headers = upstream_resp.headers().clone();
    let upstream_body = upstream_resp
        .bytes()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("body error: {e}")).into_response())?;

    let mut response = Response::new(axum::body::Body::from(upstream_body));
    *response.status_mut() = status;

    let headers_mut = response.headers_mut();
    for (name, value) in &upstream_headers {
        let name_str = name.as_str().to_lowercase();
        if name_str.starts_with("x-") && name_str != "x-session-token" && name_str != "x-user-id" {
            continue;
        }
        headers_mut.insert(
            HeaderName::from_bytes(name.as_str().as_bytes()).unwrap(),
            value.clone(),
        );
    }

    Ok(response)
}

pub async fn request_handler(
    Extension(app): Extension<GatewayApp>,
    req: Request,
) -> Response {
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    let target = match route_url(&path, &app) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, "not found").into_response(),
    };

    let (parts, body) = req.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .unwrap_or_default();

    let session_token = extract_session_token_from_headers(&parts.headers);

    if path == "/logout" {
        let resp = forward_request(
            &app,
            &target,
            method,
            &parts.headers,
            body_bytes,
            session_token.clone(),
            None,
        )
        .await
        .unwrap_or_else(|_| (StatusCode::BAD_GATEWAY, "bad gateway").into_response());

        if resp.status().is_success() {
            if let Some(token) = session_token {
                app.invalidate_token(&token).await;
            }
        }

        return resp;
    }

    let validation = if let Some(ref token) = session_token {
        match app.validate_token(token.clone()).await {
            Ok(v) => v,
            Err(_) => return (StatusCode::BAD_GATEWAY, "auth error").into_response(),
        }
    } else {
        None
    };

    forward_request(
        &app,
        &target,
        method,
        &parts.headers,
        body_bytes,
        session_token,
        validation,
    )
    .await
    .unwrap_or_else(|_| (StatusCode::BAD_GATEWAY, "bad gateway").into_response())
}
