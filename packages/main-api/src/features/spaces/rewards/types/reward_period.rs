use crate::*;
use chrono::Datelike;
#[derive(
    Debug,
    Default,
    Clone,
    Serialize,
    Deserialize,
    JsonSchema,
    OperationIo,
    DynamoEnum,
    Eq,
    PartialEq,
)]
pub enum RewardPeriod {
    #[default]
    Once, // Permenently, one-time reward
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Unlimited,
}

impl RewardPeriod {
    pub fn to_time_key(&self, timestamp: i64) -> String {
        let dt = chrono::DateTime::from_timestamp_millis(timestamp)
            .unwrap()
            .with_timezone(&chrono::Utc);

        match self {
            RewardPeriod::Once => "ONCE".to_string(),
            RewardPeriod::Hourly => dt.format("%Y%m%d%H").to_string(), // 2025120123
            RewardPeriod::Daily => dt.format("%Y%m%d").to_string(),    // 20251201
            RewardPeriod::Weekly => {
                // ISO week: 2025W48
                format!("{}W{:02}", dt.format("%Y"), dt.iso_week().week())
            }
            RewardPeriod::Monthly => dt.format("%Y%m").to_string(), // 202512
            RewardPeriod::Yearly => dt.format("%Y").to_string(),    // 2025
            RewardPeriod::Unlimited => dt.timestamp_millis().to_string(),
        }
    }
}
