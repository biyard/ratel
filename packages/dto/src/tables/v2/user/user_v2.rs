use by_types::QueryResponse;

use bdk::prelude::*;

use crate::{Badge, Follower, Group, Membership, Team, Theme, UserType};
use crate::{Feed, GroupRepositoryQueryBuilder};

#[derive(validator::Validate)]
#[api_model(base = "/v2/users", table = users, iter_type=QueryResponse)]
pub struct UserV2 {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    pub nickname: String,
    pub principal: String,
    #[api_model(unique)]
    pub email: String,
    #[api_model(nullable)]
    pub profile_url: String,

    pub term_agreed: bool,
    pub informed_agreed: bool,

    #[api_model(type = INTEGER, indexed, version = v0.1)]
    pub user_type: UserType,
    #[api_model(version = v0.1, indexed)]
    pub parent_id: Option<i64>,
    #[api_model(version = v0.1, indexed, unique)]
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
    #[api_model(version = v0.2)]
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

    #[api_model(many_to_many = feed_bookmark_users, foreign_table_name = feeds, foreign_primary_key = feed_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub bookmarked_feeds: Vec<Feed>,

    #[api_model(version = v0.3, indexed, unique)]
    #[serde(default)]
    pub evm_address: String,

    #[api_model(version = v0.4)]
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

    #[api_model(version = v0.8, unique)]
    #[serde(default)]
    pub telegram_id: Option<i64>,
}
