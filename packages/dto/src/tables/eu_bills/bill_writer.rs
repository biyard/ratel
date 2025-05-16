use bdk::prelude::*;

#[api_model(base = "/m1/eu/bills", table = eu_bills, action = [fetch_bills(start_bill_no = i64, end_bill_no = i64)])]
pub struct EUBillWriter {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, unique, action = [fetch_bill])]
    pub bill_id: String, // equal identifier
    #[api_model(summary, action = [fetch_bills, fetch_recent_bills])]
    pub year: i64,
    #[api_model(summary, action = [fetch_recent_bills])]
    pub parliamentary_term: i64,
    #[api_model(summary)]
    pub bill_no: i64,
    #[api_model(summary)]
    pub date: i32,
    #[api_model(summary)]
    pub label: String,
    #[api_model(summary)]
    pub ep_number: String,

    #[api_model(summary)]
    pub title: String,
    #[api_model(summary)]
    pub alternative_title: Option<String>,
    #[api_model(summary)]
    pub pdf_url: Option<String>,
    #[api_model(summary)]
    pub xml_url: Option<String>,
    #[api_model(summary)]
    pub docs_url: Option<String>,
    #[api_model(summary)]
    pub subject_matter: Option<String>,
}
