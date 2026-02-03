use crate::services::portone::{PageInfo, PaymentItem};
use crate::*;
use bdk::prelude::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct AdminPaymentDetail {
    pub payment_id: String,
    pub status: String,
    pub currency: String,
    pub paid_at: Option<String>,
    pub order_name: String,
    pub is_subscription: bool,
    pub user_id: Option<String>,
    pub user_email: Option<String>,
    pub user_name: Option<String>,
    pub total: i64,
    pub tax_free: Option<i64>,
    pub vat: Option<i64>,
    pub supply: Option<i64>,
    pub discount: Option<i64>,
    pub paid: i64,
    pub cancelled: Option<i64>,
    pub cancelled_tax_free: Option<i64>,
}

impl From<&PaymentItem> for AdminPaymentDetail {
    fn from(item: &PaymentItem) -> Self {
        Self {
            payment_id: item.id.clone(),
            status: item.status.clone(),
            currency: item.currency.clone(),
            paid_at: item.paid_at.clone(),
            order_name: item.order_name.clone(),
            total: item.amount.total,
            tax_free: item.amount.tax_free,
            vat: item.amount.vat,
            supply: item.amount.supply,
            discount: item.amount.discount,
            paid: item.amount.paid,
            cancelled: item.amount.cancelled,
            cancelled_tax_free: item.amount.cancelled_tax_free,
            is_subscription: item.billing_key.is_some(),
            ..Default::default()
        }
    }
}

impl AdminPaymentDetail {
    pub fn with_user(mut self, user_id: String, email: String, name: String) -> Self {
        self.user_id = Some(user_id);
        self.user_email = Some(email);
        self.user_name = Some(name);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct AdminPaymentListResponse {
    pub items: Vec<AdminPaymentDetail>,
    pub page: PageInfo,
}
