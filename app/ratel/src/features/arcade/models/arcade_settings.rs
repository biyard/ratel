use crate::common::*;
use crate::features::arcade::types::ArcadeSettingsResponse;

#[allow(unused_imports)]
use rmcp::schemars;

/// Singleton settings row for the whole arcade. Lives at
/// `Partition::ArcadeSettings + EntityType::ArcadeSettings`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, rmcp::schemars::JsonSchema)
)]
pub struct ArcadeSettings {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub rp_to_chip_ratio_bps: i32,
    #[serde(default)]
    pub default_buy_in_chips: i64,
    #[serde(default)]
    pub min_convert_rp: i64,
    #[serde(default)]
    pub redeem_enabled: bool,
}

#[cfg(feature = "server")]
impl ArcadeSettings {
    pub fn keys() -> (Partition, EntityType) {
        (Partition::ArcadeSettings, EntityType::ArcadeSettings)
    }

    pub async fn get_or_default(
        cli: &aws_sdk_dynamodb::Client,
    ) -> crate::common::Result<ArcadeSettingsResponse> {
        let (pk, sk) = Self::keys();
        let row = ArcadeSettings::get(cli, &pk, Some(sk)).await?;
        Ok(row.map(ArcadeSettingsResponse::from).unwrap_or_default())
    }
}

impl From<ArcadeSettings> for ArcadeSettingsResponse {
    fn from(v: ArcadeSettings) -> Self {
        Self {
            rp_to_chip_ratio_bps: v.rp_to_chip_ratio_bps,
            default_buy_in_chips: v.default_buy_in_chips,
            min_convert_rp: v.min_convert_rp,
            redeem_enabled: v.redeem_enabled,
        }
    }
}
