#[derive(Debug, Clone, serde::Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "idToken")]
    pub id_token: Option<String>,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<i64>,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub email: Option<String>,
}

crate::define_invoke_tauri!(sign_in, "google_sign_in", res: TokenResponse);
