use crate::tables::Vote;

use super::super::proposers::Proposer;
use bdk::prelude::*;
use by_types::QueryResponse;

#[derive(
    Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, dioxus_translate::Translate, Copy,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum BillSorter {
    #[default]
    #[translate(ko = "최신순", en = "Newest")]
    Newest = 1,
}

#[api_model(base = "/v1/bills", table = bills, iter_type = QueryResponse)]
pub struct Bill {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,

    #[api_model(summary, unique)]
    pub bill_no: String, // actual bills number in the assembly
    #[api_model(summary, unique)]
    pub bill_id: String, // ex. PRC_E0O9Q0W6A3S0T1U3M0H0O5H7Q6C6H2
    #[api_model(summary)]
    pub title: String,
    #[api_model(summary)]
    pub book_id: String, // for file download, type = 0 (hwp), 1 (pdf)

    #[api_model(summary, version = v0.1)]
    pub site_url: String,

    #[api_model(summary)]
    pub en_title: Option<String>,
    #[api_model(summary, action_by_id = set_summary)]
    pub summary: Option<String>,
    #[api_model(summary, action_by_id = set_en_summary)]
    pub en_summary: Option<String>,

    #[api_model(one_to_many = proposers, foreign_key = bill_id)]
    #[serde(default)]
    pub proponents: Vec<Proposer>,

    #[api_model(summary, one_to_many = votes, foreign_key = bill_id)]
    #[serde(default)]
    pub votes: Vec<Vote>,
    // // FIXME: need conditional sum
    // #[api_model(one_to_many = bill_votes, foreign_key = bill_id, aggregator = sum(amount))]
    // pub pros: i64,
    // // FIXME: need conditional sum
    // #[api_model(one_to_many = bill_votes, foreign_key = bill_id, aggregator = sum(amount))]
    // pub cons: i64,
}

impl BillSummary {
    pub fn summary(&self, lang: Language) -> String {
        match lang {
            Language::En => self.en_summary.clone().unwrap_or_default(),
            _ => self.summary.clone().unwrap_or_default(),
        }
    }

    pub fn title(&self, lang: Language) -> String {
        match lang {
            Language::En => self.en_title.clone().unwrap_or(self.title.clone()),
            _ => self.title.clone(),
        }
    }
}
