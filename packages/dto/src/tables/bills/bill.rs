use crate::{VoteOption, tables::Vote};

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
    #[api_model(summary)]
    pub date: String,

    // #[api_model(summary, version = v0.1)]
    // pub site_url: String, // ex. https://likms.assembly.go.kr/bill/billDetail.do?billId=PRC_E2F4E1M0N1J5I1G7F1N2M0L8M1K8T3
    #[api_model(summary)]
    pub en_title: Option<String>,
    #[api_model(summary, action_by_id = set_summary)]
    pub summary: Option<String>,
    #[api_model(summary, action_by_id = set_en_summary)]
    pub en_summary: Option<String>,

    #[api_model(summary, one_to_many = votes, foreign_key = bill_id)]
    #[serde(default)]
    pub votes: Vec<Vote>,

    // FIXME: Currently, the system doesn't properly support Option<T> types in relationships.
    // When a row is not exist in the database, deserialization fails for the entire struct
    // if the field type isn't nullable.
    //
    // To work around this limitation, we're using Vec<Vote> instead of Option<Vote>
    // since an empty vector ([]) is properly handled during deserialization, while null values cause errors.
    //
    // This is a temporary solution. In the future, we should modify the by-macros
    // to properly handle Option<T> types for single-object relationships, allowing
    // null values to be correctly deserialized as None.
    #[api_model(summary, one_to_many = votes, foreign_key = bill_id, filter_by = user_id)]
    #[serde(default)]
    pub user_vote: Vec<Vote>,
}

impl Bill {
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

    pub fn votes(&self) -> (i64, i64) {
        let mut yes = 0;
        let mut no = 0;

        for v in self.votes.iter() {
            match v.selected {
                VoteOption::Supportive => {
                    yes += 1;
                }
                VoteOption::Against => {
                    no += 1;
                }
            }
        }

        (yes, no)
    }

    pub fn votes_percent(&self) -> (f64, f64) {
        let (yes, no) = self.votes();
        let total = yes + no;

        let yes_percent = if total > 0 {
            yes as f64 / total as f64
        } else {
            0.0
        };

        let no_percent = if total > 0 {
            no as f64 / total as f64
        } else {
            0.0
        };

        (yes_percent * 100.0, no_percent * 100.0)
    }

    pub fn file_link(&self) -> String {
        format!(
            "{}?bookId={}&type={}",
            "https://likms.assembly.go.kr/filegate/servlet/FileGate",
            self.book_id,
            "0" // 0: hwp 1: pdf
        )
    }

    pub fn detail_link(&self) -> String {
        format!(
            "{}?billId={}",
            "https://likms.assembly.go.kr/bill/billDetail.do", self.bill_id,
        )
    }
}
