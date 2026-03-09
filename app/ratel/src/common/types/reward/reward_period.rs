use crate::common::*;
use chrono::Datelike;

#[derive(
    Debug,
    Default,
    Clone,
    Serialize,
    Deserialize,
    DynamoEnum,
    Eq,
    PartialEq,
)]
pub enum RewardPeriod {
    #[default]
    Once, // Permanently, one-time reward
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
            RewardPeriod::Hourly => dt.format("%Y%m%d%H").to_string(),
            RewardPeriod::Daily => dt.format("%Y%m%d").to_string(),
            RewardPeriod::Weekly => {
                format!("{}W{:02}", dt.format("%Y"), dt.iso_week().week())
            }
            RewardPeriod::Monthly => dt.format("%Y%m").to_string(),
            RewardPeriod::Yearly => dt.format("%Y").to_string(),
            RewardPeriod::Unlimited => dt.timestamp_millis().to_string(),
        }
    }
}
