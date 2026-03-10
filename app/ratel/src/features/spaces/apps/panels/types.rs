use super::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::str::FromStr;

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpacePanelAttributeEntityType(pub String);

impl std::fmt::Display for SpacePanelAttributeEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SpacePanelAttributeEntityType {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = if s.starts_with("SPACE_PANEL_ATTRIBUTE#") {
            s.replacen("SPACE_PANEL_ATTRIBUTE#", "", 1)
        } else {
            s.to_string()
        };
        Ok(Self(s))
    }
}

impl From<EntityType> for SpacePanelAttributeEntityType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::SpacePanelAttribute(label, value) if value.is_empty() => Self(label),
            EntityType::SpacePanelAttribute(label, value) => Self(format!("{label}#{value}")),
            _ => Self::default(),
        }
    }
}

impl From<SpacePanelAttributeEntityType> for EntityType {
    fn from(value: SpacePanelAttributeEntityType) -> Self {
        let mut parts = value.0.splitn(2, '#');
        let label = parts.next().unwrap_or_default().to_string();
        let value = parts.next().unwrap_or_default().to_string();
        EntityType::SpacePanelAttribute(label, value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpacePanelQuotaResponse {
    pub panel_id: SpacePanelAttributeEntityType,
    pub quotas: i64,
    pub remains: i64,
    pub attributes: PanelAttribute,
}

impl From<SpacePanelQuota> for SpacePanelQuotaResponse {
    fn from(panel: SpacePanelQuota) -> Self {
        Self {
            panel_id: panel.sk.into(),
            quotas: panel.quotas,
            remains: panel.remains,
            attributes: panel.attributes,
        }
    }
}
