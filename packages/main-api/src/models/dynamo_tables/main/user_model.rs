use super::base_model::*;
use super::serde_helpers as sh;
use crate::types::dynamo_entity_type::EntityType;
use bdk::prelude::*;
use dto::{Membership, Theme, UserType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub base: BaseModel,
    pub id: i64,
    pub nickname: String,
    pub principal: String,
    pub email: String,
    pub profile_url: Option<String>,
    pub term_agreed: bool,
    pub informed_agreed: bool,
    #[serde(with = "sh::user_type_num")]
    pub user_type: UserType,
    pub parent_id: Option<i64>,
    pub username: String,
    pub followers_count: i64,
    pub followings_count: i64,
    pub html_contents: String,
    pub evm_address: String,
    pub password: String,
    #[serde(with = "sh::membership_num")]
    pub membership: Membership,
    #[serde(with = "sh::theme_opt_num")]
    pub theme: Option<Theme>,
    pub points: i64,
    pub referral_code: String,
    pub phone_number: Option<String>,
    pub telegram_id: Option<i64>,
}

impl User {
    pub fn from_postgres_user(user: &dto::User) -> Self {
        let pk = format!("{}#{}", USER_PREFIX, user.id);
        let sk = METADATA_SK.to_string();
        let base = BaseModel::new(pk, sk, EntityType::User)
            .with_gsi1(format!("EMAIL#{}", user.email), None)
            .with_gsi2(format!("USERNAME#{}", user.username), None)
            .with_gsi3(format!("PRINCIPAL#{}", user.principal), None)
            .with_gsi4(format!("EVM#{}", user.evm_address), None)
            .with_gsi5(
                format!("PHONE#{}", user.phone_number.clone().unwrap_or_default()),
                None,
            )
            .with_gsi6(
                format!("TELEGRAM#{}", user.telegram_id.unwrap_or_default()),
                None,
            );

        Self {
            base,
            id: user.id,
            nickname: user.nickname.clone(),
            principal: user.principal.clone(),
            email: user.email.clone(),
            profile_url: if user.profile_url.is_empty() {
                None
            } else {
                Some(user.profile_url.clone())
            },
            term_agreed: user.term_agreed,
            informed_agreed: user.informed_agreed,
            user_type: user.user_type,
            parent_id: user.parent_id,
            username: user.username.clone(),
            followers_count: user.followers_count,
            followings_count: user.followings_count,
            html_contents: user.html_contents.clone(),
            evm_address: user.evm_address.clone(),
            password: user.password.clone(),
            membership: user.membership,
            theme: user.theme,
            points: user.points,
            referral_code: user.referral_code.clone(),
            phone_number: user.phone_number.clone(),
            telegram_id: user.telegram_id,
        }
    }
}

impl DynamoModel for User {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

// User relationship models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFollower {
    #[serde(flatten)]
    pub base: BaseModel,
    pub user_id: i64,
    pub follower_id: i64,
    pub follower_nickname: String,
    pub follower_profile_url: Option<String>,
}

impl UserFollower {
    pub fn new(
        user_id: i64,
        follower_id: i64,
        follower_nickname: String,
        follower_profile_url: Option<String>,
    ) -> Self {
        let pk = format!("{}#{}", USER_PREFIX, user_id);
        let sk = format!("{}#{}", FOLLOWER_PREFIX, follower_id);
        let base = BaseModel::new(pk, sk, EntityType::Follower).with_gsi1(
            format!("{}#{}", USER_PREFIX, follower_id),
            Some(format!("{}#{}", FOLLOWING_PREFIX, user_id)),
        );

        Self {
            base,
            user_id,
            follower_id,
            follower_nickname,
            follower_profile_url,
        }
    }
}

impl DynamoModel for UserFollower {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBadge {
    #[serde(flatten)]
    pub base: BaseModel,
    pub user_id: i64,
    pub badge_id: i64,
    pub badge_name: String,
    pub badge_description: String,
}

impl UserBadge {
    pub fn new(user_id: i64, badge_id: i64, badge_name: String, badge_description: String) -> Self {
        let pk = format!("{}#{}", USER_PREFIX, user_id);
        let sk = format!("{}#{}", BADGE_PREFIX, badge_id);
        let base = BaseModel::new(pk, sk, EntityType::Badge);

        Self {
            base,
            user_id,
            badge_id,
            badge_name,
            badge_description,
        }
    }
}

impl DynamoModel for UserBadge {
    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}
