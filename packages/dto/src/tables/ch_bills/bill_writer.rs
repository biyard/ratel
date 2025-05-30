use bdk::prelude::*;

#[api_model(base = "/m1/ch/bills", table = ch_bills, action = [fetch_bills(start_page_no = i64, end_page_no = i64), fetch_recent_bills(year = i64)])]
pub struct CHBillWriter {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, unique, action = [fetch_bill])]
    pub bill_id: i64,
    #[api_model(summary)]
    pub year: i64,
    #[api_model(summary)]
    pub bill_no: i64,

    #[api_model(summary)]
    pub title: String,
    #[api_model(summary)]
    pub description: Option<String>,
    #[api_model(summary)]
    pub initial_situation: Option<String>,
    #[api_model(summary)]
    pub proceedings: Option<String>,
    #[api_model(summary)]
    pub date: i32,
}
