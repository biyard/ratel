use by_types::QueryResponse;

use bdk::prelude::*;

use crate::Industry;
use crate::{Badge, Group};

use super::Follower;
use super::Team;
use crate::GroupRepositoryQueryBuilder;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users", read_action = [user_info], table = users, action = [signup(telegram_raw = Option<String>), email_signup(telegram_raw = Option<String>), update_telegram_id(telegram_raw = Option<String>)], iter_type=QueryResponse)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = [signup, email_signup], action_by_id = edit_profile)]
    pub nickname: String,
    #[api_model(unique, read_action = by_principal)]
    pub principal: String,
    #[api_model(action = [signup, email_signup], read_action = [check_email, login, login_by_password, find_by_email], unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = [signup, email_signup], nullable, action_by_id = edit_profile)]
    // #[validate(url)]
    pub profile_url: String,

    #[api_model(action = [signup, email_signup])]
    pub term_agreed: bool,
    #[api_model(action = [signup, email_signup])]
    pub informed_agreed: bool,

    #[api_model(type = INTEGER, indexed, version = v0.1)]
    pub user_type: UserType,
    #[api_model(version = v0.1, indexed)]
    pub parent_id: Option<i64>,
    #[api_model(action = [signup, email_signup], read_action = [find_by_username], version = v0.1, indexed, unique)]
    #[serde(default)]
    pub username: String,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = follower_id, foreign_reference_key = following_id, aggregator = count)]
    #[serde(default)]
    pub followers_count: i64,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = following_id, foreign_reference_key = follower_id, aggregator = count)]
    #[serde(default)]
    pub followings_count: i64,

    #[api_model(many_to_many = group_members, foreign_table_name = groups, foreign_primary_key = group_id, foreign_reference_key = user_id, nested)]
    #[serde(default)]
    pub groups: Vec<Group>,

    #[api_model(many_to_many = team_members, foreign_table_name = users, foreign_primary_key = team_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub teams: Vec<Team>,

    // profile contents
    #[api_model(version = v0.2, action_by_id = edit_profile)]
    #[serde(default)]
    pub html_contents: String,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = follower_id, foreign_reference_key = following_id)]
    #[serde(default)]
    pub followers: Vec<Follower>,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = following_id, foreign_reference_key = follower_id)]
    #[serde(default)]
    pub followings: Vec<Follower>,

    #[api_model(many_to_many = user_badges, foreign_table_name = badges, foreign_primary_key = badge_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub badges: Vec<Badge>,

    #[api_model(version = v0.3, indexed, unique, action = signup, action = update_evm_address)]
    #[serde(default)]
    pub evm_address: String,

    #[api_model(version = v0.4, action = [email_signup], read_action = login_by_password)]
    #[serde(default)]
    pub password: String,

    #[api_model(version = v0.5, type = INTEGER)]
    #[serde(default)]
    pub membership: Membership,

    #[api_model(version = v1.0, type = INTEGER)]
    #[serde(default)]
    pub theme: Option<Theme>,

    #[api_model(one_to_many = user_points, foreign_key = user_id, aggregator = sum(amount))]
    pub points: i64,

    #[api_model(version = v0.6, unique, indexed)]
    #[serde(default)]
    pub referral_code: String,

    #[api_model(version = v0.9, unique)]
    #[serde(default)]
    pub phone_number: Option<String>,

    #[api_model(read_action = find_by_phone_number, skip)]
    #[serde(default)]
    pub phone: String,

    #[api_model(version = v0.8, unique)]
    #[serde(default)]
    pub telegram_id: Option<i64>,

    #[api_model(read_action = login_by_telegram, skip)]
    #[serde(default)]
    pub telegram_raw: String,

    #[api_model(one_to_many = user_industries, foreign_table_name = industries, foreign_primary_key = industry_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub industry: Vec<Industry>,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.groups
            .iter()
            .any(|g| g.permissions == 0xffffffffffffffffu64 as i64)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum UserType {
    #[default]
    Individual = 1,
    Team = 2,
    Bot = 3,
    Anonymous = 99,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Membership {
    #[default]
    Free = 1,
    Paid1 = 2,
    Paid2 = 3,
    Paid3 = 4,
    Admin = 99,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Theme {
    #[default]
    Light = 1,
    Dark = 2,
    SystemDefault = 3,
}
