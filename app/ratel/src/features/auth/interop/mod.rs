mod google_auth;
pub use google_auth::*;

mod wallet_connect;
use super::*;
pub use wallet_connect::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub access_token: String,
    pub id_token: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
}
