use super::base_model::*;
use aws_sdk_dynamodb::types::AttributeValue;
use dto::{Membership, Result, Theme, User, UserType, Follower};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoUser {
    pub base: BaseModel,
    pub id: i64,
    pub nickname: String,
    pub principal: String,
    pub email: String,
    pub profile_url: Option<String>,
    pub term_agreed: bool,
    pub informed_agreed: bool,
    pub user_type: UserType,
    pub parent_id: Option<i64>,
    pub username: String,
    pub followers_count: i64,
    pub followings_count: i64,
    pub html_contents: String,
    pub evm_address: String,
    pub password: String,
    pub membership: Membership,
    pub theme: Option<Theme>,
    pub points: i64,
    pub referral_code: String,
    pub phone_number: Option<String>,
    pub telegram_id: Option<i64>,
}

impl DynamoUser {
    pub fn from_postgres_user(user: &User) -> Self {
        let pk = format!("{}#{}", USER_PREFIX, user.id);
        let sk = METADATA_SK.to_string();

        let base = BaseModel::new(pk, sk, "USER".to_string())
            .with_gsi1(format!("EMAIL#{}", user.email), None)
            .with_gsi2(format!("USERNAME#{}", user.username), None);

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

    pub fn to_postgres_user(&self) -> User {
        User {
            id: self.id,
            created_at: self.base.created_at,
            updated_at: self.base.updated_at,
            nickname: self.nickname.clone(),
            principal: self.principal.clone(),
            email: self.email.clone(),
            profile_url: self.profile_url.clone().unwrap_or_default(),
            term_agreed: self.term_agreed,
            informed_agreed: self.informed_agreed,
            user_type: self.user_type,
            parent_id: self.parent_id,
            username: self.username.clone(),
            followers_count: self.followers_count,
            followings_count: self.followings_count,
            groups: Vec::new(), // Will be loaded separately
            teams: Vec::new(),  // Will be loaded separately
            html_contents: self.html_contents.clone(),
            followers: Vec::new(),  // Will be loaded separately
            followings: Vec::new(), // Will be loaded separately
            badges: Vec::new(),     // Will be loaded separately
            evm_address: self.evm_address.clone(),
            password: self.password.clone(),
            membership: self.membership,
            theme: self.theme,
            points: self.points,
            referral_code: self.referral_code.clone(),
            phone_number: self.phone_number.clone(),
            phone: self.phone_number.clone().unwrap_or_default(),
            telegram_id: self.telegram_id,
            telegram_raw: String::new(), // Not stored in DynamoDB
            industry: Vec::new(),        // Will be loaded separately
        }
    }

    pub fn from_postgres_follower(follower: &Follower) -> UserFollower {
        // TODO: Fix field access when proper Follower structure is available
        UserFollower::new(
            follower.id,
            1, // placeholder follower_id
            "Unknown".to_string(), // placeholder nickname
            None // placeholder profile_url
        )
    }
}

impl DynamoModel for DynamoUser {
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();

        // Base model fields
        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));

        if let Some(ref gsi1_pk) = self.base.gsi1_pk {
            item.insert("gsi1_pk".to_string(), string_attr(gsi1_pk));
        }
        if let Some(ref gsi1_sk) = self.base.gsi1_sk {
            item.insert("gsi1_sk".to_string(), string_attr(gsi1_sk));
        }
        if let Some(ref gsi2_pk) = self.base.gsi2_pk {
            item.insert("gsi2_pk".to_string(), string_attr(gsi2_pk));
        }
        if let Some(ref gsi2_sk) = self.base.gsi2_sk {
            item.insert("gsi2_sk".to_string(), string_attr(gsi2_sk));
        }

        // User-specific fields
        item.insert("id".to_string(), number_attr(self.id));
        item.insert("nickname".to_string(), string_attr(&self.nickname));
        item.insert("principal".to_string(), string_attr(&self.principal));
        item.insert("email".to_string(), string_attr(&self.email));

        if let Some(ref profile_url) = self.profile_url {
            item.insert("profile_url".to_string(), string_attr(profile_url));
        }

        item.insert("term_agreed".to_string(), bool_attr(self.term_agreed));
        item.insert(
            "informed_agreed".to_string(),
            bool_attr(self.informed_agreed),
        );
        item.insert("user_type".to_string(), number_attr(self.user_type as i64));

        if let Some(parent_id) = self.parent_id {
            item.insert("parent_id".to_string(), number_attr(parent_id));
        }

        item.insert("username".to_string(), string_attr(&self.username));
        item.insert(
            "followers_count".to_string(),
            number_attr(self.followers_count),
        );
        item.insert(
            "followings_count".to_string(),
            number_attr(self.followings_count),
        );
        item.insert(
            "html_contents".to_string(),
            string_attr(&self.html_contents),
        );
        item.insert("evm_address".to_string(), string_attr(&self.evm_address));
        item.insert("password".to_string(), string_attr(&self.password));
        item.insert(
            "membership".to_string(),
            number_attr(self.membership as i64),
        );

        if let Some(theme) = self.theme {
            item.insert("theme".to_string(), number_attr(theme as i64));
        }

        item.insert("points".to_string(), number_attr(self.points));
        item.insert(
            "referral_code".to_string(),
            string_attr(&self.referral_code),
        );

        if let Some(ref phone_number) = self.phone_number {
            item.insert("phone_number".to_string(), string_attr(phone_number));
        }

        if let Some(telegram_id) = self.telegram_id {
            item.insert("telegram_id".to_string(), number_attr(telegram_id));
        }

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;

        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        let user_type_num = extract_number(&item, "user_type")?;
        let user_type = match user_type_num {
            1 => UserType::Individual,
            2 => UserType::Team,
            3 => UserType::Bot,
            99 => UserType::Anonymous,
            _ => UserType::Individual,
        };

        let membership_num = extract_number(&item, "membership")?;
        let membership = match membership_num {
            1 => Membership::Free,
            2 => Membership::Paid1,
            3 => Membership::Paid2,
            4 => Membership::Paid3,
            99 => Membership::Admin,
            _ => Membership::Free,
        };

        let theme = extract_optional_number(&item, "theme").map(|t| match t {
            1 => Theme::Light,
            2 => Theme::Dark,
            3 => Theme::SystemDefault,
            _ => Theme::Light,
        });

        Ok(Self {
            base,
            id: extract_number(&item, "id")?,
            nickname: extract_string(&item, "nickname")?,
            principal: extract_string(&item, "principal")?,
            email: extract_string(&item, "email")?,
            profile_url: extract_optional_string(&item, "profile_url"),
            term_agreed: extract_bool(&item, "term_agreed")?,
            informed_agreed: extract_bool(&item, "informed_agreed")?,
            user_type,
            parent_id: extract_optional_number(&item, "parent_id"),
            username: extract_string(&item, "username")?,
            followers_count: extract_number(&item, "followers_count")?,
            followings_count: extract_number(&item, "followings_count")?,
            html_contents: extract_string(&item, "html_contents")?,
            evm_address: extract_string(&item, "evm_address")?,
            password: extract_string(&item, "password")?,
            membership,
            theme,
            points: extract_number(&item, "points")?,
            referral_code: extract_string(&item, "referral_code")?,
            phone_number: extract_optional_string(&item, "phone_number"),
            telegram_id: extract_optional_number(&item, "telegram_id"),
        })
    }

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
        let base = BaseModel::new(pk, sk, "USER_FOLLOWER".to_string()).with_gsi1(
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
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();

        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));

        if let Some(ref gsi1_pk) = self.base.gsi1_pk {
            item.insert("gsi1_pk".to_string(), string_attr(gsi1_pk));
        }
        if let Some(ref gsi1_sk) = self.base.gsi1_sk {
            item.insert("gsi1_sk".to_string(), string_attr(gsi1_sk));
        }

        item.insert("user_id".to_string(), number_attr(self.user_id));
        item.insert("follower_id".to_string(), number_attr(self.follower_id));
        item.insert(
            "follower_nickname".to_string(),
            string_attr(&self.follower_nickname),
        );

        if let Some(ref url) = self.follower_profile_url {
            item.insert("follower_profile_url".to_string(), string_attr(url));
        }

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;

        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        Ok(Self {
            base,
            user_id: extract_number(&item, "user_id")?,
            follower_id: extract_number(&item, "follower_id")?,
            follower_nickname: extract_string(&item, "follower_nickname")?,
            follower_profile_url: extract_optional_string(&item, "follower_profile_url"),
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBadge {
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
        let base = BaseModel::new(pk, sk, "USER_BADGE".to_string());

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
    fn to_item(&self) -> Result<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();

        item.insert("pk".to_string(), string_attr(&self.base.pk));
        item.insert("sk".to_string(), string_attr(&self.base.sk));
        item.insert("type".to_string(), string_attr(&self.base.entity_type));
        item.insert("created_at".to_string(), number_attr(self.base.created_at));
        item.insert("updated_at".to_string(), number_attr(self.base.updated_at));

        item.insert("user_id".to_string(), number_attr(self.user_id));
        item.insert("badge_id".to_string(), number_attr(self.badge_id));
        item.insert("badge_name".to_string(), string_attr(&self.badge_name));
        item.insert(
            "badge_description".to_string(),
            string_attr(&self.badge_description),
        );

        Ok(item)
    }

    fn from_item(item: HashMap<String, AttributeValue>) -> Result<Self> {
        let pk = extract_string(&item, "pk")?;
        let sk = extract_string(&item, "sk")?;
        let entity_type = extract_string(&item, "type")?;
        let created_at = extract_number(&item, "created_at")?;
        let updated_at = extract_number(&item, "updated_at")?;

        let base = BaseModel {
            pk,
            sk,
            entity_type,
            created_at,
            updated_at,
            gsi1_pk: extract_optional_string(&item, "gsi1_pk"),
            gsi1_sk: extract_optional_string(&item, "gsi1_sk"),
            gsi2_pk: extract_optional_string(&item, "gsi2_pk"),
            gsi2_sk: extract_optional_string(&item, "gsi2_sk"),
        };

        Ok(Self {
            base,
            user_id: extract_number(&item, "user_id")?,
            badge_id: extract_number(&item, "badge_id")?,
            badge_name: extract_string(&item, "badge_name")?,
            badge_description: extract_string(&item, "badge_description")?,
        })
    }

    fn pk(&self) -> String {
        self.base.pk.clone()
    }

    fn sk(&self) -> String {
        self.base.sk.clone()
    }
}
