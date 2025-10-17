use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, schemars::JsonSchema,
)]
pub struct SpaceSurveyAnswer {
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

impl SpaceSurveyAnswer {
    pub fn new(
        space_pk: Partition,
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
        let survey_id = match survey_pk.clone() {
            Partition::Survey(v) => v,
            _ => "".to_string(),
        };

        let user_id = match pk.clone() {
            Partition::User(v) | Partition::Team(v) => v,
            _ => "".to_string(),
        };

        let sk = EntityType::SurveyResponse(survey_id, user_id);

        Self {
            pk: space_pk,
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
pub struct SpaceSurveyAnswerResponse {
    pub pk: Partition,
    pub sk: EntityType,

    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub survey_type: SurveyType,
    pub answers: Vec<SurveyAnswer>,
}

impl From<SpaceSurveyAnswer> for SpaceSurveyAnswerResponse {
    fn from(responses: SpaceSurveyAnswer) -> Self {
        Self {
            pk: responses.clone().pk,
            sk: responses.clone().sk,
            user_pk: responses.clone().user_pk,
            author_display_name: responses.clone().author_display_name,
            author_profile_url: responses.clone().author_profile_url,
            author_username: responses.clone().author_username,

            survey_type: responses.clone().survey_type,
            answers: responses.answers,
        }
    }
}
