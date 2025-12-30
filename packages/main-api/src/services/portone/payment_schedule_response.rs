use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
pub struct PaymentScheduleResponse {
    pub schedule: PaymentSchedule,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
pub struct PaymentSchedule {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
pub struct PaymentCancelScheduleResponse {
    pub revoked_schedule_ids: Vec<String>,
    pub revoked_at: Option<String>,
}
