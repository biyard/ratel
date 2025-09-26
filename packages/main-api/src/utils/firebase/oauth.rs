#[cfg(not(feature = "no-secret"))]
pub use r::*;

#[cfg(feature = "no-secret")]
pub use noop::*;

#[cfg(not(feature = "no-secret"))]
mod r {
    use chrono::Utc;
    use dto::reqwest;
    use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
    use once_cell::sync::Lazy;
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::time::{Duration, Instant};
    use tokio::sync::Mutex;

    use crate::{Error2, config};

    // https://firebase.google.com/docs/auth/admin/verify-id-tokens?_gl=1*rpu45t*_up*MQ..*_ga*MTA3NjIzNjEyOS4xNzU4Njk1MDI0*_ga_CW55HF8NVT*czE3NTg2OTUwMjMkbzEkZzAkdDE3NTg2OTUwMjMkajYwJGwwJGgw#c++

    #[derive(Debug, Deserialize)]
    struct Claims {
        // aud: String, // Audience
        // iss: String, // Issuer
        sub: String, // uid
        // exp: usize,  // Expiration time
        // iat: usize,  // Issued at
        auth_time: i64,
    }

    struct KeyCache {
        keys: HashMap<String, DecodingKey>,
        expires_at: Instant,
    }

    impl KeyCache {
        fn is_expired(&self) -> bool {
            self.expires_at <= Instant::now()
        }
    }

    static PUBLIC_KEYS: Lazy<Mutex<KeyCache>> = Lazy::new(|| {
        Mutex::new(KeyCache {
            keys: HashMap::new(),
            expires_at: Instant::now(),
        })
    });

    const GOOGLE_PUBLIC_KEYS_URL: &str =
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

    async fn fetch_and_cache_keys() -> Result<(), Error2> {
        let client = reqwest::Client::new();
        let response = client
            .get(GOOGLE_PUBLIC_KEYS_URL)
            .send()
            .await
            .map_err(|e| Error2::InternalServerError(format!("Failed to fetch public keys: {}", e)))?;

        let cache_control = response
            .headers()
            .get("Cache-Control")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("max-age=0");

        let max_age = cache_control
            .split(',')
            .find_map(|part| part.trim().strip_prefix("max-age="))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let fetched_keys: HashMap<String, String> = response.json().await.map_err(|e| {
            Error2::InternalServerError(format!("Failed to parse public keys JSON: {}", e))
        })?;

        let decoding_keys = fetched_keys
            .into_iter()
            .map(|(kid, pem)| {
                DecodingKey::from_rsa_pem(pem.as_bytes())
                    .map(|key| (kid, key))
                    .map_err(|e| Error2::InternalServerError(format!("Invalid PEM for kid {:?}", e)))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        let mut cache = PUBLIC_KEYS.lock().await;
        *cache = KeyCache {
            keys: decoding_keys,
            expires_at: Instant::now() + Duration::from_secs(max_age),
        };

        tracing::debug!("Fetched and cached {} public keys", cache.keys.len());
        Ok(())
    }

    /// Verify Firebase ID and return uid if valid.
    ///
    /// # Arguments
    /// * `token_str` - Firebase ID Token string
    ///
    /// # Returns
    /// * `Ok(String)` - User uid
    /// * `Err(String)` - Err Message
    pub async fn verify_token(token_str: &str) -> Result<String, Error2> {
        let project_id = config::get().firebase.project_id;

        let header = decode_header(token_str)
            .map_err(|e| Error2::BadRequest(format!("Invalid Firebase token header: {}", e)))?;

        if header.alg != Algorithm::RS256 {
            return Err(Error2::BadRequest(
                "Invalid Firebase token algorithm".to_string(),
            ));
        }

        let kid = header.kid.ok_or(Error2::BadRequest(
            "Token header missing 'kid' field".to_string(),
        ))?;

        {
            let cache = PUBLIC_KEYS.lock().await;
            if cache.is_expired() {
                drop(cache);
                let _ = fetch_and_cache_keys().await;
            }
        }

        let decoding_key = {
            let cache = PUBLIC_KEYS.lock().await;
            cache
                .keys
                .get(&kid)
                .cloned()
                .ok_or(Error2::BadRequest(format!("Unknown kid: {}", kid)))?
        };

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[project_id]);
        validation.set_issuer(&[format!("https://securetoken.google.com/{}", project_id)]);

        let token_data = decode::<Claims>(token_str, &decoding_key, &validation)
            .map_err(|e| Error2::BadRequest(format!("Failed to decode token: {}", e)))?;

        let claims = token_data.claims;
        let now = Utc::now().timestamp();

        if claims.auth_time > now {
            return Err(Error2::BadRequest(
                "auth_time must be in the past".to_string(),
            ));
        }
        if claims.sub.is_empty() {
            return Err(Error2::BadRequest(
                "sub (uid) must be a non-empty string".to_string(),
            ));
        }

        Ok(claims.sub)
    }
}

#[cfg(feature = "no-secret")]
mod noop {
    use crate::Error2;

    /// No-op token verification for testing.
    ///
    /// Always returns the token string as uid.
    pub async fn verify_token(token_str: &str) -> Result<String, Error2> {
        Ok(token_str.to_string())
    }
}
