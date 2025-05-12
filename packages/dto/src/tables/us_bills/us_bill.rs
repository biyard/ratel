use bdk::prelude::*;

#[api_model(base = "/v1/us_bills", table = bills)]
pub struct USBill {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
}

impl USBill {
    // pub fn detail_date(&self, str_date: &str) -> String {
    //     let date = chrono::NaiveDate::parse_from_str(str_date, "%Y-%m-%d").unwrap_or_default();
    //     let start = date.format("%b %d").to_string();
    //     let end = date.format(" - %b %d, %Y").to_string();

    //     format!("{}{}", start, end)
    // }
}
