use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::FactOrFoldSettingsResponse;

#[allow(unused_imports)]
use rmcp::schemars;

/// Singleton row carrying admin-tunable parameters for the
/// *Fact or Fold* game. Pairs with `EntityType::FactFoldSettings` at
/// the `Partition::FactFoldSettings` singleton key, mirroring the
/// `AnalyzeQuotaConfig` pattern.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct FactFoldSettings {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub round_capacity: i32,
    #[serde(default)]
    pub stage_news_reveal_sec: i32,
    #[serde(default)]
    pub stage_bet_sec: i32,
    #[serde(default)]
    pub stage_rationale_sec: i32,
    #[serde(default)]
    pub stage_reveal_sec: i32,
    #[serde(default)]
    pub stage_debate_sec: i32,
    #[serde(default)]
    pub min_bet_rp: i64,
    #[serde(default)]
    pub max_bet_rp: i64,
    #[serde(default)]
    pub correct_side_multiplier_bps: i32,
    #[serde(default)]
    pub insider_correct_bonus_bps: i32,
    #[serde(default)]
    pub influence_bonus_bps: i32,
    #[serde(default)]
    pub new_user_signup_rp: i64,
    #[serde(default)]
    pub reconnect_grace_sec: i32,
    #[serde(default)]
    pub queue_low_alert_days: i32,
}

#[cfg(feature = "server")]
impl FactFoldSettings {
    pub fn keys() -> (Partition, EntityType) {
        (Partition::FactFoldSettings, EntityType::FactFoldSettings)
    }

    /// Load the singleton row, returning `FactOrFoldSettingsResponse::default()`
    /// when the row has not been initialised yet. This lets the rest
    /// of the codebase rely on a populated value out of the box.
    pub async fn get_or_default(
        cli: &aws_sdk_dynamodb::Client,
    ) -> crate::common::Result<FactOrFoldSettingsResponse> {
        let (pk, sk) = Self::keys();
        let row = FactFoldSettings::get(cli, &pk, Some(sk)).await?;
        Ok(row
            .map(FactOrFoldSettingsResponse::from)
            .unwrap_or_default())
    }

    pub fn merge_response(mut self, r: FactOrFoldSettingsResponse) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        self.round_capacity = r.round_capacity;
        self.stage_news_reveal_sec = r.stage_news_reveal_sec;
        self.stage_bet_sec = r.stage_bet_sec;
        self.stage_rationale_sec = r.stage_rationale_sec;
        self.stage_reveal_sec = r.stage_reveal_sec;
        self.stage_debate_sec = r.stage_debate_sec;
        self.min_bet_rp = r.min_bet_rp;
        self.max_bet_rp = r.max_bet_rp;
        self.correct_side_multiplier_bps = r.correct_side_multiplier_bps;
        self.insider_correct_bonus_bps = r.insider_correct_bonus_bps;
        self.influence_bonus_bps = r.influence_bonus_bps;
        self.new_user_signup_rp = r.new_user_signup_rp;
        self.reconnect_grace_sec = r.reconnect_grace_sec;
        self.queue_low_alert_days = r.queue_low_alert_days;
        if self.created_at == 0 {
            self.created_at = now;
        }
        self.updated_at = now;
        let (pk, sk) = Self::keys();
        self.pk = pk;
        self.sk = sk;
        self
    }
}

impl From<FactFoldSettings> for FactOrFoldSettingsResponse {
    fn from(v: FactFoldSettings) -> Self {
        Self {
            round_capacity: v.round_capacity,
            stage_news_reveal_sec: v.stage_news_reveal_sec,
            stage_bet_sec: v.stage_bet_sec,
            stage_rationale_sec: v.stage_rationale_sec,
            stage_reveal_sec: v.stage_reveal_sec,
            stage_debate_sec: v.stage_debate_sec,
            min_bet_rp: v.min_bet_rp,
            max_bet_rp: v.max_bet_rp,
            correct_side_multiplier_bps: v.correct_side_multiplier_bps,
            insider_correct_bonus_bps: v.insider_correct_bonus_bps,
            influence_bonus_bps: v.influence_bonus_bps,
            new_user_signup_rp: v.new_user_signup_rp,
            reconnect_grace_sec: v.reconnect_grace_sec,
            queue_low_alert_days: v.queue_low_alert_days,
        }
    }
}
