use crate::gateway_app::GatewayApp;
use actix_web::error::ErrorBadGateway;
use actix_web::{web, HttpRequest, HttpResponse};

macro_rules! route_request {
    (
        ($input_path:ident, $input_method:expr):
        $(
            $method:ident $path:literal -> $target:literal $flag:ident
        ),* $(,)?
    ) => {
        match ($input_path, $input_method) {
            $(
                ($path, actix_web::http::Method::$method) => ($target, route_request!(@flag $flag)),
            )*
            _ => return HttpResponse::NotFound().finish(),
        }
    };

    (@flag VALIDATE) => { true };
    (@flag NOT_VALIDATE) => { false };
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
    let method = req.method().clone();

    let (target, must_validate) = route_request!((path, method.clone()):
        POST "/login" -> "http://auth-service:8080/login" NOT_VALIDATE,
        POST "/sign_in" -> "http://auth-service:8080/sign_in" NOT_VALIDATE,
        POST "/post" -> "http://post-service:8080/post" VALIDATE
    );

    let validation = match app.validate_token(&req).await {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadGateway().finish(),
    };

    if must_validate && validation.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    forward_request(&req, body, target, validation)
        .await
        .unwrap_or_else(|_| HttpResponse::BadGateway().finish())
}
