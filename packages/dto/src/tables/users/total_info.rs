use crate::UserType;
use bdk::prelude::*;
#[derive(validator::Validate)]
#[api_model(base = "/v1/totals", table = users)]
pub struct TotalInfo {
    #[api_model(summary)]
    pub id: i64,
    #[api_model(summary)]
    pub created_at: i64,
    #[api_model(summary)]
    pub updated_at: i64,
    #[api_model(summary)]
    pub username: String,
    #[api_model(summary)]
    pub nickname: String,
    #[api_model(summary)]
    pub email: String,
    #[api_model(summary)]
    pub profile_url: String,
    #[api_model(summary, type = INTEGER, indexed, version = v0.1)]
    pub user_type: UserType,
}
