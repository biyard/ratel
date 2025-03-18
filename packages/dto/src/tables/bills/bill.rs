use super::super::proposers::Proposer;
use bdk::prelude::*;
use by_types::QueryResponse;

#[api_model(base = "/v1/bills", table = bills, iter_type = QueryResponse)]
pub struct Bill {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,

    #[api_model(summary, unique)]
    pub bill_no: String, // actual bills number in the assembly
    #[api_model(summary)]
    pub title: String,
    #[api_model(summary)]
    pub book_id: String, // for file download, type = 0 (hwp), 1 (pdf)

    #[api_model(summary)]
    pub en_title: Option<String>,
    #[api_model(summary, action_by_id = set_summary)]
    pub summary: Option<String>,
    #[api_model(summary, action_by_id = set_en_summary)]
    pub en_summary: Option<String>,

    #[api_model(one_to_many = proposers, foreign_key = bill_id)]
    #[serde(default)]
    pub proponents: Vec<Proposer>,
}
