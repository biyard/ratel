use bdk::prelude::*;
use by_types::QueryResponse;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/suggested_users", table = suggested_users, iter_type=QueryResponse)]
pub struct SuggestedUser {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    
    #[api_model(action = [set_dismissed], action_by_id = [update])]
    pub dismissed: bool,

    #[api_model(summary, action = [insert])]
    pub nickname: String,          // suggested user's nickname

    #[api_model(summary, action = [insert], nullable)]
    pub profile_image_url: Option<String>, // suggested user's profile image URL

    #[api_model(summary, action = [insert])]
    pub profile_url: String,    // suggested user's profile URL

    #[api_model(summary, action = [insert], nullable)]
    pub description: Option<String>, // suggested user's description

    #[api_model(many_to_one = users)]
    pub user_id: i64,                   // user this suggestion is for

    #[api_model(many_to_one = users)]
    pub suggested_user_id: i64,         // suggested user id

    // #[api_model(summary)]
    // pub suggestion_score: i64,

    // #[api_model(type = INTEGER, indexed, version = v0.1)]
    // pub suggestions_reason: SuggestionReason,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SuggestionReason {
    MutualFollows = 1,       // Users with mutual followers
    SimilarInterests = 2,    // Based on similar political interests/activity
    PopularInNetwork = 3,    // Popular users in user's network
    SameGroups = 4,          // Users in same groups/teams
    GeographicProximity = 5, // Users from same region/country
    SimilarVotes = 6,        // Users with similar voting patterns
    ActiveUsers = 7,         // Most active users recently
    IndustryMatch = 8,       // Users from same industry
    ContentInteraction = 9,  // Users who interact with similar content
    NewUsers = 10,           // Recently joined users
    #[default]
    Random = 99,             // Randomly suggested users
}