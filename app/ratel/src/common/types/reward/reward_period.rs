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
    Translate,
)]
pub enum RewardPeriod {
    #[default]
    #[translate(en = "Once", ko = "1회")]
    Once, // Permanently, one-time reward
    #[translate(en = "Hourly", ko = "매시간")]
    Hourly,
    #[translate(en = "Daily", ko = "매일")]
    Daily,
    #[translate(en = "Weekly", ko = "매주")]
    Weekly,
    #[translate(en = "Monthly", ko = "매월")]
    Monthly,
    #[translate(en = "Yearly", ko = "매년")]
    Yearly,
    #[translate(en = "Unlimited", ko = "무제한")]
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
