use crate::features::auth::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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

#[cfg(feature = "server")]
impl UserRefreshToken {
    pub fn new(user: &User, device_id: String) -> (Self, String) {
        let sk = EntityType::UserRefreshToken(device_id.clone());
        let plain_token = uuid::Uuid::now_v7().to_string();
        let token_hash = crate::common::utils::sha256::sha256_base64url(&plain_token);
        let created_at = crate::common::utils::time::now();

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
