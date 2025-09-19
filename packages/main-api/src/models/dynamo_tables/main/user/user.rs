use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct User {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub display_name: String,
    pub profile_url: String,
    #[dynamo(prefix = "EMAIL", name = "find_by_email", index = "gsi1", pk)]
    pub email: String,
    // NOTE: username is linked with gsi2-index of team model.
    #[dynamo(prefix = "USERNAME", name = "find_by_username", index = "gsi2", pk)]
    pub username: String,

    pub term_agreed: bool,
    pub informed_agreed: bool,

    pub user_type: UserType,
    pub parent_id: Option<String>,

    pub followers_count: i64,
    pub followings_count: i64,

    // profile contents
    pub html_contents: String,
    pub password: String,

    pub membership: Membership,
    pub theme: Theme,
    pub points: i64,
}

impl User {
    pub fn new(
        display_name: String,
        email: String,
        profile_url: String,
        term_agreed: bool,
        informed_agreed: bool,
        user_type: UserType,
        parent_id: Option<String>,
        username: String,
        password: String,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let pk = Partition::User(uid);
        let sk = EntityType::User;

        let now = get_now_timestamp_millis();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            display_name,
            email,
            profile_url,
            term_agreed,
            informed_agreed,
            user_type,
            parent_id,
            username,
            password,
            ..Default::default()
        }
    }
}
