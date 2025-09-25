use dto::File;

use super::*;
use crate::{
    models::{space::SpaceCommon, user::User},
    tests::{get_test_aws_config, get_test_user},
    types::CheckboxQuestion,
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
    let deliberation_member = DeliberationSpaceMember::new(
        deliberation.pk.clone(),
        deliberation_discussion.pk.clone(),
        user.clone(),
    );
    let res = deliberation_member.create(&cli).await;
    assert!(res.is_ok());
    let uid = uuid::Uuid::new_v4().to_string();
    let deliberation_participant = DeliberationSpaceParticipant::new(
        deliberation.pk.clone(),
        deliberation_discussion.pk.clone(),
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

    let deliberation_question_1 = DeliberationSpaceQuestion::new(
        deliberation.pk.clone(),
        deliberation_survey.pk.clone(),
        crate::types::SurveyQuestion::Checkbox(CheckboxQuestion {
            title: "question 1".to_string(),
            description: Some("question description".to_string()),
            image_url: None,
            options: vec!["option 1".to_string(), "option 2".to_string()],
            is_multi: false,
            is_required: Some(false),
        }),
    );
    let res = deliberation_question_1.create(&cli).await;
    assert!(res.is_ok());

    let deliberation_question_2 = DeliberationSpaceQuestion::new(
        deliberation.pk.clone(),
        deliberation_survey.pk.clone(),
        crate::types::SurveyQuestion::Checkbox(CheckboxQuestion {
            title: "question 2".to_string(),
            description: Some("question description 2".to_string()),
            image_url: None,
            options: vec!["option 1".to_string(), "option 2".to_string()],
            is_multi: false,
            is_required: Some(false),
        }),
    );
    let res = deliberation_question_2.create(&cli).await;
    assert!(res.is_ok());

    let deliberation_response = DeliberationSpaceResponse::new(
        deliberation.pk.clone(),
        deliberation_survey.pk.clone(),
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
}
