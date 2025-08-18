use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use anyhow::{Result, anyhow};
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use ring::rand::SystemRandom;
use ring::signature::{ED25519, Ed25519KeyPair, KeyPair, Signature, UnparsedPublicKey};

pub fn generate_usersig_token(sign_domain: &str) -> Result<String> {
    let rng = SystemRandom::new();
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng)
        .map_err(|e| anyhow!("key generation failed: {:?}", e))?;
    let keypair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref())
        .map_err(|e| anyhow!("invalid pkcs8: {:?}", e))?;
    let public_key = keypair.public_key().as_ref();

    let timestamp = Utc::now().timestamp();

    let message = format!("{}-{}", sign_domain, timestamp);
    let message_bytes = message.as_bytes();

    let signature: Signature = keypair.sign(message_bytes);

    UnparsedPublicKey::new(&ED25519, public_key)
        .verify(message_bytes, signature.as_ref())
        .map_err(|_| anyhow!("Signature verification failed"))?;

    let public_key_b64 = general_purpose::STANDARD.encode(public_key);
    let signature_b64 = general_purpose::STANDARD.encode(signature.as_ref());

    let token = format!("{}:eddsa:{}:{}", timestamp, public_key_b64, signature_b64);

    Ok(token)
}
// ...existing code...
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub nickname: String,
    pub principal: String,
    pub email: String,
    pub profile_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSignupRequest {
    pub nickname: String,
    pub email: String,
    pub profile_url: String,
    pub term_agreed: bool,
    pub informed_agreed: bool,
    pub username: String,
    pub password: String,
    pub telegram_raw: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAction {
    email_signup: EmailSignupRequest,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeedResponse {
    id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpaceResponse {
    id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ArtworkResponse {
    id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOracleRequest {
    pub user_id: i64,
    pub oracle_type: i64,
    pub space_id: Option<i64>,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: option_env!("API_BASE_URL")
                .expect("API_BASE_URL must be set")
                .to_string(),
        }
    }
    pub async fn vote(&self, space_id: i64, artwork_id: i64, jwt: &str) -> Result<()> {
        let url = format!(
            "{}/v2/dagits/{}/artworks/{}/vote",
            self.base_url, space_id, artwork_id
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&serde_json::json!({
              "description": null,
              "vote_type": 1
            }))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!("Failed to vote: {} - {}", status, error_text))
        }
    }
    pub async fn create_user(&self, user: &EmailSignupRequest, usersig: String) -> Result<User> {
        let url = format!("{}/v1/users", self.base_url);

        let user_action = UserAction {
            email_signup: user.clone(),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("usersig {}", usersig))
            .json(&user_action)
            .send()
            .await?;

        if response.status().is_success() {
            let user_response: User = response.json().await?;
            Ok(user_response)
        } else {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!(
                "Failed to create user: {} - {}",
                status,
                error_text
            ))
        }
    }

    pub async fn add_oracle_to_space(
        &self,
        user_id: i64,
        space_id: i64,
        token: &str,
    ) -> Result<()> {
        let url = format!("{}/v2/dagits/{}/oracles", self.base_url, space_id);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&serde_json::json!({
                "user_id": user_id,
            }))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!(
                "Failed to add oracle to space: {} - {}",
                status,
                error_text
            ))
        }
    }

    pub async fn add_oracle(&self, user_id: i64, space_id: i64, token: &str) -> Result<()> {
        let url = format!("{}/m2/oracles", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&serde_json::json!({
                "dagit_id": space_id,
                "user_id": user_id,
                "oracle_type": 1,
            }))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!("Failed to add oracle: {} - {}", status, error_text))
        }
    }

    pub async fn create_dagit_space(&self, user_id: i64, jwt: &str) -> Result<i64> {
        let url = format!("{}/v1/feeds", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "create_draft": {
                    "feed_type": 1,
                    "user_id": user_id
                }
            }))
            .header("Authorization", format!("Bearer {}", jwt))
            .send()
            .await?;

        let feed_id = if response.status().is_success() {
            let feed = response.json::<FeedResponse>().await?;
            feed.id
        } else {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Failed to create dagit space: {} - {}",
                status,
                error_text
            ));
        };

        let url = format!("{}/v1/feeds/{}", self.base_url, feed_id);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&serde_json::json!({
              "update": {
                "industry_id": 1,
                "title": "VERIFICATION",
                "html_contents": "<p class=\"relative mb-1\" dir=\"ltr\"><span style=\"white-space: pre-wrap;\">VERIFICATION</span></p>",
                "url_type": 1,
                "url": "",
                "files": []
              }
            }))
            .send()
            .await?;

        if response.status().is_client_error() {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Failed to create feed: {} - {}",
                status,
                error_text
            ));
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&serde_json::json!({
              "publish": {
              }
            }))
            .send()
            .await?;

        if response.status().is_client_error() {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Failed to publish feed: {} - {}",
                status,
                error_text
            ));
        }

        let url = format!("{}/v1/spaces", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&serde_json::json!({
                "create_space": {
                    "booster_type": 1,
                    "feed_id": feed_id,
                    "num_of_redeem_codes": 0,
                    "space_type": 8,
                    "user_ids": []
                }
            }))
            .send()
            .await?;

        if response.status().is_client_error() {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Failed to create space: {} - {}",
                status,
                error_text
            ));
        }
        let space = response
            .json::<SpaceResponse>()
            .await
            .map_err(|e| anyhow!("Failed to parse space response: {:?}", e))?;

        let url = format!("{}/v1/spaces", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&serde_json::json!({
                "posting_space": {

                }
            }))
            .send()
            .await?;

        if response.status().is_client_error() {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Failed to create space: {} - {}",
                status,
                error_text
            ));
        }

        Ok(space.id)
    }

    pub async fn create_artwork(&self, space_id: i64, jwt: &str) -> Result<i64> {
        // Create Artwork
        let url = format!("{}/v2/dagits/{}/artworks", self.base_url, space_id);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&serde_json::json!({
                    "title": "Artwork for Consensus",
                    "description": "This artwork is created for the Consensus space.",
                    "file": {
                    "ext": "JPG",
                    "name": "",
                    "size": "",
                    "url": "https://metadata.ratel.foundation/metadata/7ccb90b3-4d74-4460-a820-d1b9e2e3f1a4"
                    }
                }))
            .send()
            .await?;

        if response.status().is_client_error() {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Failed to create artwork: {} - {}",
                status,
                error_text
            ));
        }

        let artwork = response
            .json::<ArtworkResponse>()
            .await
            .map_err(|e| anyhow!("Failed to parse artwork response: {:?}", e))?;

        //Start Consensus
        let url = format!("{}/v2/dagits/{}/consensus", self.base_url, space_id);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", jwt))
            .json(&serde_json::json!({
                "artwork_id": artwork.id,
            }))
            .send()
            .await?;

        if response.status().is_client_error() {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Failed to create artwork: {} - {}",
                status,
                error_text
            ));
        }

        Ok(artwork.id)
    }
    pub async fn login_user(
        &self,
        username: &str,
        password: &str,
        usersig: &str,
    ) -> Result<(String, i64)> {
        let url = format!("{}/v1/users", self.base_url);

        let response = self
            .client
            .get(&url)
            .query(&[
                ("action", "login-by-password"),
                ("email", username),
                ("password", password),
            ])
            .header("Authorization", format!("usersig {}", usersig))
            .send()
            .await?;

        if response.status().is_success() {
            for cookie_header in response.headers().get_all("set-cookie") {
                if let Ok(cookie_str) = cookie_header.to_str() {
                    if let Some(token) = extract_token_from_cookie(cookie_str) {
                        let user: User = response.json().await?;
                        return Ok((token, user.id));
                    }
                }
            }
            Err(anyhow!("Login successful but no token found in cookies"))
        } else {
            let status = response.status().clone();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(anyhow!("Failed to login: {} - {}", status, error_text))
        }
    }
}

pub fn generate_user(nickname: &str, email: &str, password: &str) -> EmailSignupRequest {
    EmailSignupRequest {
        nickname: nickname.to_string(),
        email: email.to_string(),
        profile_url: "https://metadata.ratel.foundation/ratel/default-profile.png".to_string(),
        term_agreed: true,
        informed_agreed: true,
        username: nickname.to_string(),
        password: password.to_string(),
        telegram_raw: None,
    }
}
pub fn generate_test_users(count: usize, password: &str) -> Vec<EmailSignupRequest> {
    let mut users = Vec::new();

    for i in 1..=count {
        users.push(generate_user(
            &format!("User-Veri-{:03}", i),
            &format!("User-Veri-{:03}@example.com", i),
            password,
        ));
    }

    users
}

pub fn extract_token_from_cookie(cookie_str: &str) -> Option<String> {
    let cookies: Vec<&str> = cookie_str.split(&[',', ';'][..]).collect();

    for cookie in cookies {
        let cookie = cookie.trim();

        if let Some(equals_pos) = cookie.find('=') {
            let key = cookie[..equals_pos].trim().to_lowercase();
            let value = cookie[equals_pos + 1..].trim();

            match key.as_str() {
                "token" | "auth_token" | "access_token" | "jwt" | "bearer" | "authorization" => {
                    return Some(value.to_string());
                }
                _ => {
                    if value.len() > 50
                        && (value.contains('.')
                            || value
                                .chars()
                                .all(|c| c.is_alphanumeric() || c == '_' || c == '-'))
                    {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}
