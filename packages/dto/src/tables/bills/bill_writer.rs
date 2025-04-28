use bdk::prelude::*;

#[api_model(base = "/m1/bills", table = bills, action = [fetch_recent_bills])]
pub struct BillWriter {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,

    #[api_model(summary, unique, action = fetch_bill)]
    pub bill_no: i64, // actual bills number in the assembly
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

    #[api_model(version = v0.1, type = INTEGER)]
    pub proposer_kind: ProposerKind, // Type of proposer (e.g., "의원", "정부")
    #[api_model(version = v0.1)]
    pub proposer_name: String, // Name(s) of the proposer(s)
    #[api_model(version = v0.1)]
    pub proposal_session: Option<String>, // Assembly session when proposed
    #[api_model(version = v0.1)]
    pub proposal_date: i32,

    #[api_model(version = v0.1)]
    pub committee_name: Option<String>, // Name of the responsible committee
    #[api_model(version = v0.1)]
    pub committee_referral_date: Option<i32>, // Date referred to the committee
    #[api_model(version = v0.1)]
    pub committee_presentation_date: Option<i32>, // Date presented to the committee
    #[api_model(version = v0.1)]
    pub committee_processing_date: Option<i32>, // Date processed by the committee
    #[api_model(version = v0.1)]
    pub committee_processing_result: Option<String>, // Result from the committee (e.g., "원안가결")
    #[api_model(version = v0.1, type = INTEGER)]
    pub committee: Option<Commitee>,

    #[api_model(version = v0.1)]
    pub law_committee_referral_date: Option<i32>, // Date referred to Legislation & Judiciary Committee
    #[api_model(version = v0.1)]
    pub law_committee_presentation_date: Option<i32>, // Date presented to L&J Committee
    #[api_model(version = v0.1)]
    pub law_committee_processing_date: Option<i32>, // Date processed by L&J Committee
    #[api_model(version = v0.1)]
    pub law_committee_processing_result: Option<String>, // Result from L&J Committee

    #[api_model(version = v0.1)]
    pub plenary_presentation_date: Option<i32>, // Date presented to plenary session
    #[api_model(version = v0.1)]
    pub plenary_resolution_date: Option<i32>, // Date resolved (voted on) in plenary session
    #[api_model(version = v0.1)]
    pub plenary_conference_name: Option<String>, // Name/ID of the plenary session meeting
    #[api_model(version = v0.1)]
    pub plenary_conference_result: Option<String>, // Result from plenary session (e.g., "가결")

    #[api_model(version = v0.1)]
    pub government_transfer_date: Option<i32>, // Date transferred to the government

    #[api_model(version = v0.1)]
    pub promulgated_law_name: Option<String>, // Official name of the promulgated law
    #[api_model(version = v0.1)]
    pub promulgation_date: Option<i32>, // Date promulgated
    #[api_model(version = v0.1)]
    pub promulgation_number: Option<String>, // Promulgation number
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProposerKind {
    #[default]
    #[translate(ko = "의원", en = "Member")]
    Member = 1,
    #[translate(ko = "정부", en = "Government")]
    Government = 2,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum BillStatus {
    #[default]
    Proposed = 1,
    UnderReviewingByCommitee = 2,
    UnderReviewingByLawCommittee = 3,
    UnderReviewingByPlenary = 5,
    Resulted = 4,
}

impl BillStatus {
    pub fn from_bill_writer(bill: &BillWriter) -> BillStatus {
        if bill.plenary_conference_result.is_some() {
            return BillStatus::Resulted;
        } else if bill.plenary_presentation_date.is_some() {
            return BillStatus::UnderReviewingByPlenary;
        } else if bill.law_committee_presentation_date.is_some() {
            return BillStatus::UnderReviewingByLawCommittee;
        } else if bill.committee_presentation_date.is_some() {
            return BillStatus::UnderReviewingByCommitee;
        } else {
            return BillStatus::Proposed;
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Commitee {
    #[default]
    None = 0,

    #[translate(
        ko = "국토교통위원회",
        en = "Land Infrastructure and Transport Committee"
    )]
    LITC = 1,
}
