use actix_web::HttpRequest;
use awc::error::SendRequestError;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Deserialize)]
struct ValidateResponse {
    user_id: Option<String>,
}

type CacheMap = HashMap<String, (Option<String>, Instant)>;
pub struct GatewayApp {
    cache: Arc<RwLock<CacheMap>>,
}

impl GatewayApp {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn handle_validate_request(
        &self,
        token: String,
    ) -> Result<Option<String>, SendRequestError> {
        let auth_validate = "http://auth-service:8080/validate";
        let client = awc::Client::new();

        let mut resp = client
            .post(auth_validate)
            .insert_header(("X-Session-Token", token.clone()))
            .send()
            .await?;

        let parsed: ValidateResponse = resp.json().await.unwrap();
        let response_user_id = parsed.user_id;

        let ttl = Duration::from_secs(3600);
        let expires_at = Instant::now() + ttl;
        {
            let mut cache_write = self.cache.write().await;
            cache_write.insert(token, (response_user_id.clone(), expires_at));
        }

        Ok(response_user_id)
    }

    pub async fn validate_token(
        &self,
        req: &HttpRequest,
    ) -> Result<Option<String>, SendRequestError> {
        if let Some(token) = req.cookie("session_token").map(|c| c.value().to_string()) {
            let cache_result = {
                let cache_read = self.cache.read().await;
                cache_read.get(&token).cloned()
            };

            match cache_result {
                Some((Some(user_id), expires_at)) => {
                    if Instant::now() > expires_at {
                        self.handle_validate_request(token).await
                    } else {
                        Ok(Some(user_id.clone()))
                    }
                }
                Some((None, expires_at)) => {
                    if Instant::now() > expires_at {
                        self.handle_validate_request(token).await
                    } else {
                        Ok(None)
                    }
                }
                None => self.handle_validate_request(token).await,
            }
        } else {
            Ok(None)
        }
    }
}
