use crate::utils::sha256_baseurl::sha256_base64url;
use crate::*;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct UserRefreshToken {
    pub pk: Partition,
    pub sk: EntityType,
    pub token_hash: String,

    #[dynamo(name = "find_by_device_id", prefix = "DEVICE", index = "gsi1", pk)]
    pub device_id: String,

    pub user_display_name: String,
    pub user_profile_url: String,
    pub user_username: String,
    pub user_type: UserType,

    #[dynamo(index = "gsi1", sk, order = 0)]
    pub created_at: i64,
    pub expired_at: Option<i64>,
    pub revoked: bool,
}

impl UserRefreshToken {
    pub fn new(user: &User, device_id: String) -> (Self, String) {
        let sk = EntityType::UserRefreshToken(device_id.clone());
        let plain_token = sorted_uuid();
        let token_hash = sha256_base64url(&plain_token);
        let created_at = now();

        (
            Self {
                pk: user.pk.clone(),
                sk,
                token_hash,
                device_id,
                user_display_name: user.display_name.clone(),
                user_profile_url: user.profile_url.clone(),
                user_username: user.username.clone(),
                user_type: user.user_type.clone(),
                created_at,
                expired_at: None,
                revoked: false,
            },
            plain_token,
        )
    }
}
