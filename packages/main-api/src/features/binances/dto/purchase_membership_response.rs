use crate::aide::OperationIo;
use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct PurchaseMembershipResponse {
    pub checkout_url: String,
    pub deeplink: String,
    pub prepay_id: String,
    pub qr_content: String,
    pub qrcode_link: String,
}
