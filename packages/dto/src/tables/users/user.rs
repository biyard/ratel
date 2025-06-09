use by_types::QueryResponse;

use bdk::prelude::*;

use crate::{Follower, Group};

use super::Team;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users", read_action = user_info, table = users, iter_type=QueryResponse)]
pub struct User {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = signup, action_by_id = edit_profile)]
    pub nickname: String,
    #[api_model(unique, read_action = by_principal)]
    pub principal: String,
    #[api_model(action = signup, read_action = [check_email, login], unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = signup, nullable, action_by_id = edit_profile)]
    #[validate(url)]
    pub profile_url: String,

    #[api_model(action = signup)]
    pub term_agreed: bool,
    #[api_model(action = signup)]
    pub informed_agreed: bool,

    #[api_model(type = INTEGER, indexed, version = v0.1)]
    pub user_type: UserType,
    #[api_model(version = v0.1, indexed)]
    pub parent_id: Option<i64>,
    #[api_model(action = signup, version = v0.1, indexed, unique)]
    pub username: String,

    #[api_model(one_to_many = followers, foreign_key = user_id)]
    #[serde(default)]
    pub followers: Vec<Follower>,
    #[api_model(many_to_many = group_members, foreign_table_name = groups, foreign_primary_key = group_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub groups: Vec<Group>,

    #[api_model(many_to_many = team_members, foreign_table_name = users, foreign_primary_key = team_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub teams: Vec<Team>,

    // profile contents
    #[api_model(version = v0.2, action_by_id = edit_profile)]
    #[serde(default)]
    pub html_contents: String,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.groups
            .iter()
            .any(|g| g.permissions == 0xffffffffffffffffu64 as i64)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum UserType {
    #[default]
    Individual = 1,
    Team = 2,
    Bot = 3,
    Anonymous = 99,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Membership {
    #[default]
    #[translate(en = "General", ko = "일반")]
    General = 1,
    #[translate(en = "Limited", ko = "리미티드")]
    Limited = 2,
    #[translate(en = "Premium", ko = "프리미엄")]
    Premium = 3,
}

impl Membership {
    pub fn get_description(&self) -> &'static str {
        match self {
            Membership::General => "General membership with basic features.",
            Membership::Limited => "Limited membership with some advanced features.",
            Membership::Premium => {
                "Premium membership with all features.(receive legislative updates ahead of others"
            }
        }
    }

    pub fn get_price(&self) -> i32 {
        match self {
            Membership::General => 10000,
            Membership::Limited => 30000,
            Membership::Premium => 50000,
        }
    }
}
