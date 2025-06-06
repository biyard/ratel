use bdk::prelude::*;
use validator::Validate;

//FIXME: fix to connect data
#[derive(Validate)]
#[api_model(base = "/m1/logs", table = event_logs, action = [fetch_logs])]
pub struct EventLog {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub tx_hash: String,
    #[api_model(version = v0.1, summary)]
    pub contract_address: String,
    #[api_model(summary)]
    pub operator: String,
    #[api_model(summary)]
    pub from_address: String,
    #[api_model(summary)]
    pub to_address: String,
    #[api_model(summary)]
    pub token_id: i64,
    #[api_model(summary)]
    pub value: i64,
    #[api_model(summary, nullable)]
    pub block_number: Option<i64>,
}
