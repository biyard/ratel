use dto::File;

use super::*;
use crate::{
    models::{space::SpaceCommon, user::User},
    tests::{get_test_aws_config, get_test_user},
    types::{CheckboxQuestion, EntityType},
    utils::aws::DynamoClient,
};

#[tokio::test]
async fn tests_create_deliberation() {
    let cli = DynamoClient::mock(get_test_aws_config()).client;
    let user = get_test_user(&cli).await;

    let fetched_user = User::get(&cli, user.clone().pk.clone(), Some(user.clone().sk)).await;
    assert!(fetched_user.is_ok());

    let deliberation = DeliberationSpace::new(user.clone());
    let res = deliberation.create(&cli).await;
    assert!(res.is_ok());

    //FIXME: fix to real post data when post is implemented
    let post_pk = uuid::Uuid::new_v4().to_string();

    let space_common = SpaceCommon::new(
        deliberation.pk.clone(),
        crate::types::Partition::Feed(post_pk),
    );
    let res = space_common.create(&cli).await;
    assert!(res.is_ok());

    let deliberation_summary = DeliberationSpaceSummary::new(
        deliberation.pk.clone(),
        "<div>deliberation space</div>".to_string(),
        [File {
            name: "excel file".to_string(),
            size: "15KB".to_string(),
            ext: dto::FileExtension::EXCEL,
            url: None,
        }]
        .to_vec(),
    );
    let res = deliberation_summary.create(&cli).await;
    assert!(res.is_ok());

    let now = chrono::Utc::now().timestamp();
    let deliberation_discussion = DeliberationSpaceDiscussion::new(
        deliberation.pk.clone(),
        "discussion title".to_string(),
        "discussion desc".to_string(),
        now,
        now,
        None,
        "".to_string(),
        None,
        None,
        user.clone(),
    );
    let res = deliberation_discussion.create(&cli).await;
    assert!(res.is_ok());

    let discussion_pk = match deliberation_discussion.sk {
        EntityType::DeliberationSpaceDiscussion(v) => v,
        _ => "".to_string(),
    };

    let deliberation_member = DeliberationSpaceMember::new(
        deliberation.pk.clone(),
        crate::types::Partition::Discussion(discussion_pk.clone()),
        user.clone(),
    );
    let res = deliberation_member.create(&cli).await;
    assert!(res.is_ok());
    let uid = uuid::Uuid::new_v4().to_string();

    let deliberation_participant = DeliberationSpaceParticipant::new(
        deliberation.pk.clone(),
        crate::types::Partition::Discussion(discussion_pk.clone()),
        uid.clone(),
        user.clone(),
    );
    let res = deliberation_participant.create(&cli).await;
    assert!(res.is_ok());

    let deliberation_elearning = DeliberationSpaceElearning::new(
        deliberation.pk.clone(),
        [File {
            name: "elearning file".to_string(),
            size: "50KB".to_string(),
            ext: dto::FileExtension::PDF,
            url: None,
        }]
        .to_vec(),
    );
    let res = deliberation_elearning.create(&cli).await;
    assert!(res.is_ok());

    let deliberation_survey = DeliberationSpaceSurvey::new(
        deliberation.pk.clone(),
        crate::types::SurveyStatus::Ready,
        now,
        now + 1000,
    );
    let res = deliberation_survey.create(&cli).await;
    assert!(res.is_ok());

    let survey_pk = match deliberation_survey.sk {
        EntityType::DeliberationSpaceSurvey(v) => v,
        _ => "".to_string(),
    };

    let deliberation_question_1 = DeliberationSpaceQuestion::new(
        deliberation.pk.clone(),
        crate::types::Partition::Survey(survey_pk.clone()),
        vec![
            crate::types::SurveyQuestion::Checkbox(CheckboxQuestion {
                title: "question 1".to_string(),
                description: Some("question description".to_string()),
                image_url: None,
                options: vec!["option 1".to_string(), "option 2".to_string()],
                is_multi: false,
                is_required: Some(false),
            }),
            crate::types::SurveyQuestion::Checkbox(CheckboxQuestion {
                title: "question 2".to_string(),
                description: Some("question description 2".to_string()),
                image_url: None,
                options: vec!["option 1".to_string(), "option 2".to_string()],
                is_multi: false,
                is_required: Some(false),
            }),
        ],
    );
    let res = deliberation_question_1.create(&cli).await;
    assert!(res.is_ok());

    let deliberation_response = DeliberationSpaceResponse::new(
        deliberation.pk.clone(),
        crate::types::Partition::Survey(survey_pk.clone()),
        crate::types::SurveyType::Sample,
        vec![
            crate::types::SurveyAnswer::Checkbox {
                answer: Some(vec![1]),
            },
            crate::types::SurveyAnswer::Checkbox {
                answer: Some(vec![1]),
            },
        ],
        user.clone(),
    );
    let res = deliberation_response.create(&cli).await;
    assert!(res.is_ok());

    let deliberation_recommendation = DeliberationSpaceRecommendation::new(
        deliberation.pk.clone(),
        "<div>deliberation space recommendation</div>".to_string(),
        [File {
            name: "excel file recommendation".to_string(),
            size: "15KB".to_string(),
            ext: dto::FileExtension::EXCEL,
            url: None,
        }]
        .to_vec(),
    );
    let res = deliberation_recommendation.create(&cli).await;
    assert!(res.is_ok());

    let metadata = DeliberationMetadata::query(&cli, deliberation.pk.clone()).await;
    assert!(
        metadata.is_ok(),
        "failed to query user metadata {:?}",
        metadata.err()
    );
    let metadatas = metadata.unwrap();

    assert_eq!(metadatas.len(), 11);

    let deliberation: DeliberationDetailResponse = metadatas.into();

    assert_eq!(deliberation.summary.files[0].name, "excel file".to_string());
    assert_eq!(deliberation.discussions[0].members.len(), 1);
    assert_eq!(deliberation.discussions[0].participants.len(), 1);
    assert_eq!(
        deliberation.elearnings.files[0].name,
        "elearning file".to_string()
    );
    assert_eq!(deliberation.surveys.questions.len(), 2);
    assert_eq!(deliberation.surveys.responses.len(), 1);
    //FIXME: remove this comment when metadata limit issue is fixed
    // assert_eq!(
    //     deliberation.recommendation.files[0].name,
    //     "excel file recommendation"
    // );
}
