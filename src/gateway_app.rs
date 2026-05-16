use awc::Client;
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
    pub client: Client,
    pub auth_service_url: String,
    pub post_service_url: String,
}

impl GatewayApp {
    pub fn new(auth_service_url: String, post_service_url: String) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            client: Client::new(),
            auth_service_url,
            post_service_url,
        }
    }

    async fn handle_validate_request(
        &self,
        token: String,
    ) -> Result<Option<String>, awc::error::SendRequestError> {
        let validate_url = format!("{}/validate", self.auth_service_url);

        let mut resp = self
            .client
            .post(validate_url)
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

    pub async fn validate_token(&self, token: String) -> Result<Option<String>, awc::error::SendRequestError> {
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
    }

    pub async fn invalidate_token(&self, token: &str) {
        {
            let mut cache_write = self.cache.write().await;
            cache_write.remove(token);
        }
    }
}
