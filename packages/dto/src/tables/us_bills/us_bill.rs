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
