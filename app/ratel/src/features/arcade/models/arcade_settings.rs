use crate::common::*;

#[allow(unused_imports)]
use rmcp::schemars;

/// Operator-tunable response surface mirrored from
/// `ArcadeSettings`. Defaults match v1's "1 RP = 1 chip" decision.
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArcadeSettingsResponse {
    /// Chips received per 1 RP converted, in basis points (10_000 =
    /// 1.0×). v1 default is 10_000 so 1 RP → 1 chip. Operator can
    /// tune to make chips cheaper / more expensive in future seasons.
    pub rp_to_chip_ratio_bps: i32,

    /// Default chip amount auto-bought-in at lobby join when the
    /// caller doesn't pick one. Games can override per-round.
    pub default_buy_in_chips: i64,

    /// Minimum convert amount in RP. Prevents dust conversions.
    pub min_convert_rp: i64,

    /// Whether chip → RP redemption is enabled. v1 = false.
    pub redeem_enabled: bool,
}

impl Default for ArcadeSettingsResponse {
    fn default() -> Self {
        Self {
            rp_to_chip_ratio_bps: 10_000, // 1:1
            default_buy_in_chips: 100,
            min_convert_rp: 100,
            redeem_enabled: false,
        }
    }
}

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
