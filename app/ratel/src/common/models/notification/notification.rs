use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Notification {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub status: EventStatus,

    pub data: NotificationData,
}

#[cfg(feature = "server")]
impl Notification {
    pub fn new(data: NotificationData) -> Self {
        let uid = uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
        let now = crate::common::utils::time::get_now_timestamp_millis();

        Self {
            pk: Partition::Notification(uid.clone()),
            sk: EntityType::Notification(uid),
            created_at: now,
            status: EventStatus::Requested,
            data,
        }
    }

    pub async fn process(&self) -> Result<()> {
        tracing::info!(
            "Notification send: pk={}, status={:?}, data={:?}",
            self.pk,
            self.status,
            self.data
        );

        self.data.send().await?;

        // Update notification status to Completed
        let cfg = crate::common::CommonConfig::default();
        let cli = cfg.dynamodb();
        Notification::updater(self.pk.clone(), self.sk.clone())
            .with_status(EventStatus::Completed)
            .execute(cli)
            .await?;

        Ok(())
    }
}
