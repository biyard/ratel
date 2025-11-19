use crate::{
    types::{email_operation::EmailOperation, notification_status::NotificationStatus},
    utils::time::get_now_timestamp_millis,
    *,
};
use aws_sdk_dynamodb::types::AttributeValue;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, DynamoEntity, Default)]
pub struct Notification {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", pk, name = "find_by_user_notifications", order = 0)]
    #[dynamo(
        index = "gsi2",
        pk,
        prefix = "NOTI_USER",
        name = "find_by_user_notifications_by_status",
        order = 0
    )]
    pub user_pk: Partition,

    #[dynamo(index = "gsi1", sk, prefix = "TS", order = 0)]
    #[dynamo(index = "gsi2", sk, order = 1)]
    pub created_at: i64,
    pub readed_at: Option<i64>,

    #[dynamo(index = "gsi2", sk, order = 0)]
    pub status: NotificationStatus,
    pub operation: EmailOperation,
}

impl Notification {
    pub fn new(operation: EmailOperation, User { pk: user_pk, .. }: User) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let now = get_now_timestamp_millis();

        Self {
            pk: Partition::Notification(user_pk.to_string()),
            sk: EntityType::Notification(uid),
            user_pk,
            created_at: now,
            readed_at: None,

            status: NotificationStatus::Unread,
            operation,
        }
    }

    pub async fn list_by_user_since(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
        since: i64,
        opt: NotificationQueryOption,
    ) -> std::result::Result<(Vec<Self>, Option<String>), crate::Error> {
        let mut req = cli
            .query()
            .table_name(Self::table_name())
            .index_name("gsi1-index")
            .key_condition_expression("#pk = :pk AND #sk >= :sk")
            .expression_attribute_names("#pk", "gsi1_pk")
            .expression_attribute_names("#sk", "gsi1_sk")
            .expression_attribute_values(
                ":pk",
                AttributeValue::S(Self::compose_gsi1_pk(user_pk.to_string())),
            )
            .expression_attribute_values(":sk", AttributeValue::S(Self::compose_gsi1_sk(since)))
            .limit(opt.limit)
            .scan_index_forward(opt.scan_index_forward);

        if let Some(ref bookmark) = opt.bookmark {
            let lek = Self::decode_bookmark_all(bookmark)?;
            req = req.set_exclusive_start_key(Some(lek));
        }

        let resp = req
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

        let items: Vec<Self> = resp
            .items
            .unwrap_or_default()
            .into_iter()
            .map(serde_dynamo::from_item)
            .collect::<std::result::Result<_, _>>()?;

        let bookmark = if let Some(ref last_lek) = resp.last_evaluated_key {
            Some(Self::encode_lek_all(last_lek)?)
        } else {
            None
        };

        Ok((items, bookmark))
    }
}
