use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, schemars::JsonSchema,
)]
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
    pub answers: Vec<SurveyAnswer>,
}

impl DeliberationSpaceResponse {
    pub fn new(
        deliberation_pk: Partition,
        survey_pk: Partition,
        survey_type: SurveyType,
        answers: Vec<SurveyAnswer>,

        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        // FIXME:
        // PK + SK should be unique.
        // and, we know, one user can respond only once per survey.
        // So, We can make USER_PK + SURVEY_SK is unique.

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
            answers,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct SurveyResponseResponse {
    pub pk: Partition,

    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub survey_type: SurveyType,
    pub answers: Vec<SurveyAnswer>,
}

impl From<DeliberationSpaceResponse> for SurveyResponseResponse {
    fn from(responses: DeliberationSpaceResponse) -> Self {
        let pk = match responses.clone().sk {
            EntityType::DeliberationSpaceResponse(v) => v,
            _ => "".to_string(),
        };

        Self {
            pk: Partition::SurveyResponse(pk),
            user_pk: responses.clone().user_pk,
            author_display_name: responses.clone().author_display_name,
            author_profile_url: responses.clone().author_profile_url,
            author_username: responses.clone().author_username,

            survey_type: responses.clone().survey_type,
            answers: responses.answers,
        }
    }
}
