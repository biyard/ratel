use crate::features::spaces::pages::report::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SpaceReport {
    #[dynamo(index = "gsi1", name = "find_by_status", pk)]
    pub pk: Partition,
    pub sk: EntityType,
    #[dynamo(index = "gsi1", order = 2, sk)]
    pub created_at: i64,
    pub updated_at: i64,
    #[dynamo(index = "gsi1", order = 1, sk)]
    #[serde(default)]
    pub status: ReportStatus,

    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub html_contents: Option<String>,
}

#[cfg(feature = "server")]
impl SpaceReport {
    pub fn new(space_pk: SpacePartition, title: String, description: String) -> Self {
        use crate::common::utils::time::get_now_timestamp_millis;
        let now = get_now_timestamp_millis();
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceReport(uuid::Uuid::now_v7().to_string());
        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            status: ReportStatus::Draft,
            title,
            description,
            html_contents: None,
        }
    }

    pub fn can_view(_role: SpaceUserRole) -> Result<()> {
        Ok(())
    }

    pub fn can_edit(role: SpaceUserRole) -> Result<()> {
        match role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(Error::NoPermission),
        }
    }

    pub fn can_delete(role: SpaceUserRole) -> Result<()> {
        match role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(Error::NoPermission),
        }
    }
}
