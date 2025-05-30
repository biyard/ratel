use bdk::prelude::*;

#[api_model(base = "/m1/hk/bills", table = hk_bills, action = [fetch_bills(start_bill_no = i64, end_bill_no = i64), fetch_recent_bills])]
pub struct HKBillWriter {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, unique, action = [fetch_bill])]
    pub bill_id: String, // internal bill id (e.g. 00001)
    #[api_model(summary)]
    pub year: i32, // YYYY
    #[api_model(summary)]
    pub bill_no: i32, // bill number

    #[api_model(summary)]
    pub title: String,
    #[api_model(summary)]
    pub proposer: String,

    #[api_model(summary)]
    pub content_url: Option<String>, // URL to the PDF version of the bill (eng)

    #[api_model(summary)]
    pub committee_name: Option<String>, // the name of an ad hoc committee that is formed to examine a particular bill

    #[api_model(summary)]
    pub proposed_date: i32, // YYYYMMDD
    #[api_model(summary)]
    pub first_reading_date: Option<i32>, // YYYYMMDD
    #[api_model(summary)]
    pub second_reading_date: Option<i32>, // YYYYMMDD
    #[api_model(summary)]
    pub third_reading_date: Option<i32>, // YYYYMMDD
    #[api_model(summary)]
    pub ordinance_date: Option<i32>, // YYYYMMDD

    #[api_model(summary)]
    pub additional_information: Option<String>,
    #[api_model(summary)]
    pub remarks: Option<String>,

    #[api_model(summary, version = v0.1, type = INTEGER)]
    pub status: HKBillStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum HKBillStatus {
    #[translate(en = "Unknown", ko = "알 수 없음")]
    #[default]
    Unknown = 0,

    #[translate(en = "Introduced", ko = "발의")]
    Introduced = 1,

    #[translate(en = "First Reading", ko = "1차 독회")]
    FirstReading = 2,

    #[translate(en = "Bills Committee", ko = "법안위원회")]
    BillsCommittee = 3,

    #[translate(en = "Second Reading", ko = "2차 독회")]
    SecondReading = 4,

    #[translate(en = "Third Reading", ko = "3차 독회")]
    ThirdReading = 5,

    #[translate(en = "Passed", ko = "통과")]
    Passed = 6,
}
