use crate::services::portone::PaymentItem;
use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct AdminPaymentDetail {
    pub payment_id: String,
    pub status: String,
    pub currency: String,
    pub paid_at: Option<String>,
    pub order_name: String,
    pub is_subscription: bool,
    pub user_email: Option<String>,
    pub user_name: Option<String>,
    pub total: i64,
}

impl From<PaymentItem> for AdminPaymentDetail {
    fn from(item: PaymentItem) -> Self {
        Self {
            payment_id: item.id,
            status: item.status,
            currency: item.currency,
            paid_at: item.paid_at,
            order_name: item.order_name,
            total: item.amount.total,
            is_subscription: item.billing_key.is_some(),
            user_email: None,
            user_name: None,
        }
    }
}

impl AdminPaymentDetail {
    pub fn with_user(mut self, email: String, name: String) -> Self {
        self.user_email = Some(email);
        self.user_name = Some(name);
        self
    }
}
