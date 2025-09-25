use crate::{models::user::User, types::*};
use bdk::prelude::*;
use serde_json;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceResponse {
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,

    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    #[dynamo(prefix = "SURVEY_PK", name = "find_by_survey_pk", index = "gsi2", pk)]
    pub survey_pk: Partition,

    pub survey_type: SurveyType,

    // INFO: Serialize multiple answer vectors and save them in String format
    pub answer: String,
}

impl DeliberationSpaceResponse {
    pub fn new(
        deliberation_pk: Partition,
        survey_pk: Partition,
        survey_type: SurveyType,
        answer: SurveyAnswer,

        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let sk = EntityType::DeliberationSpaceResponse(uid);

        Self {
            pk: deliberation_pk,
            sk,
            user_pk: pk,
            author_display_name: display_name.into(),
            author_profile_url: profile_url.into(),
            author_username: username.into(),
            survey_pk,
            survey_type,
            answer: Self::serialize_answer(&answer),
        }
    }

    pub fn answer(&self) -> SurveyAnswer {
        serde_json::from_str(&self.answer).unwrap_or_default()
    }

    pub fn set_answer(&mut self, ans: SurveyAnswer) {
        self.answer = Self::serialize_answer(&ans);
    }

    pub fn try_answer(&self) -> Result<SurveyAnswer, serde_json::Error> {
        serde_json::from_str(&self.answer)
    }

    #[inline]
    fn serialize_answer(ans: &SurveyAnswer) -> String {
        serde_json::to_string(ans).unwrap_or_else(|_| "{}".to_string())
    }
}
