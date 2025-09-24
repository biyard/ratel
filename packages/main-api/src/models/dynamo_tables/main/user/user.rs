use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct User {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub display_name: String,
    pub profile_url: String,
    #[dynamo(prefix = "EMAIL", name = "find_by_email", index = "gsi1", pk)]
    pub email: String,
    // NOTE: username is linked with gsi2-index of team model.
    #[dynamo(prefix = "USERNAME", name = "find_by_username", index = "gsi2", pk)]
    pub username: String,

    pub term_agreed: bool,
    pub informed_agreed: bool,

    pub user_type: UserType,
    pub parent_id: Option<String>,

    pub followers_count: i64,
    pub followings_count: i64,

    // profile contents
    pub html_contents: String,
    pub password: String,

    pub membership: Membership,
    pub membership_info: MembershipInfo,
    pub theme: Theme,
    pub points: i64,
}

impl User {
    pub fn new(
        nickname: String,
        email: String,
        profile_url: String,
        term_agreed: bool,
        informed_agreed: bool,
        user_type: UserType,
        parent_id: Option<String>,
        username: String,
        password: String,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let pk = Partition::User(uid);
        let sk = EntityType::User;

        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            display_name: nickname,
            email,
            profile_url,
            term_agreed,
            informed_agreed,
            user_type,
            parent_id,
            username,
            password,
            membership_info: MembershipInfo::new_free(),
            ..Default::default()
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct UserUpdater {
//     k: HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
//     m: HashMap<String, aws_sdk_dynamodb::types::AttributeValueUpdate>,
// }

// impl User {
//     pub fn updater(pk: Partition, sk: EntityType) -> UserUpdater {
//         let k = HashMap::from([
//             (
//                 "pk".to_string(),
//                 serde_dynamo::to_attribute_value(&pk).expect("failed to serialize `pk`"),
//             ),
//             (
//                 "sk".to_string(),
//                 serde_dynamo::to_attribute_value(&sk).expect("failed to serialize `sk`"),
//             ),
//         ]);

//         UserUpdater {
//             m: HashMap::new(),
//             k,
//         }
//     }
// }

// impl UserUpdater {
//     pub fn with_display_name(mut self, display_name: String) -> Self {
//         let v = serde_dynamo::to_attribute_value(display_name)
//             .expect("failed to serialize `display_name`");
//         let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
//             .value(v)
//             .action(aws_sdk_dynamodb::types::AttributeAction::Put)
//             .build();
//         self.m.insert("display_name".to_string(), v);
//         self
//     }

//     pub fn increase_followers_count(mut self, by: i64) -> Self {
//         let v =
//             serde_dynamo::to_attribute_value(by).expect("failed to serialize `followers_count`");
//         let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
//             .value(v)
//             .action(aws_sdk_dynamodb::types::AttributeAction::Add)
//             .build();
//         self.m.insert("followers_count".to_string(), v);
//         self
//     }

//     pub fn decrease_follwers_count(mut self, by: i64) -> Self {
//         let v =
//             serde_dynamo::to_attribute_value(-by).expect("failed to serialize `followers_count`");
//         let v = aws_sdk_dynamodb::types::AttributeValueUpdate::builder()
//             .value(v)
//             .action(aws_sdk_dynamodb::types::AttributeAction::Add)
//             .build();
//         self.m.insert("followers_count".to_string(), v);
//         self
//     }

//     pub async fn execute(self, cli: &aws_sdk_dynamodb::Client) -> Result<(), crate::Error2> {
//         cli.update_item()
//             .table_name("ratel-local-main")
//             .set_key(Some(self.k))
//             .set_attribute_updates(Some(self.m))
//             .send()
//             .await
//             .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

//         Ok(())
//     }
// }
