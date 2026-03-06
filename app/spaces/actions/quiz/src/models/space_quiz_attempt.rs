use common::utils::time::get_now_timestamp_millis;

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceQuizAttempt {
    pub pk: Partition,  // user_pk
    pub sk: EntityType, // SpaceQuizAttempt#{quiz_id}#{attempt_id}

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "QUIZ_USER", index = "gsi1", name = "find_by_quiz_user", pk)]
    pub quiz_user: String, // {user_pk}#{quiz_id}

    pub answers: Vec<Answer>,
    pub score: i64,
}

#[cfg(feature = "server")]
impl SpaceQuizAttempt {
    pub fn new(
        quiz_id: SpaceQuizEntityType,
        user_pk: Partition,
        answers: Vec<Answer>,
        score: i64,
    ) -> Self {
        let created_at = get_now_timestamp_millis();
        let attempt_id = uuid::Uuid::now_v7().to_string();
        let (pk, sk) = Self::keys(&user_pk, &quiz_id, &attempt_id);
        let quiz_user = Self::quiz_user_key(&quiz_id, &user_pk);

        Self {
            pk,
            sk,
            created_at,
            quiz_user,
            answers,
            score,
        }
    }

    pub fn quiz_user_key(quiz_id: &SpaceQuizEntityType, user_pk: &Partition) -> String {
        format!("{user_pk}#{quiz_id}")
    }

    pub fn keys(
        user_pk: &Partition,
        quiz_id: &SpaceQuizEntityType,
        attempt_id: &str,
    ) -> (Partition, EntityType) {
        (
            Partition::SpaceQuizAttempt(user_pk.to_string()),
            EntityType::SpaceQuizAttempt(format!("{quiz_id}#{attempt_id}")),
        )
    }

    pub async fn find_latest_by_quiz_user(
        cli: &aws_sdk_dynamodb::Client,
        quiz_id: &SpaceQuizEntityType,
        user_pk: &Partition,
    ) -> crate::Result<Option<Self>> {
        let pk = Self::compose_gsi1_pk(Self::quiz_user_key(quiz_id, user_pk));
        let opt = SpaceQuizAttemptQueryOption::builder()
            .limit(1)
            .scan_index_forward(false);
        let (items, _) = Self::find_by_quiz_user(cli, &pk, opt).await?;
        Ok(items.into_iter().next())
    }

    pub async fn list_by_quiz_user(
        cli: &aws_sdk_dynamodb::Client,
        quiz_id: &SpaceQuizEntityType,
        user_pk: &Partition,
        limit: i32,
    ) -> crate::Result<Vec<Self>> {
        let pk = Self::compose_gsi1_pk(Self::quiz_user_key(quiz_id, user_pk));
        let opt = SpaceQuizAttemptQueryOption::builder()
            .limit(limit)
            .scan_index_forward(false);
        let (items, _) = Self::find_by_quiz_user(cli, &pk, opt).await?;
        Ok(items)
    }
}
