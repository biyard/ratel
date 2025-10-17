use super::{super::PollResponse, Poll, PollMetadata, PollQuestion, PollUserResponse};
use crate::{
    models::{feed::Post, space::SpaceCommon},
    tests::{create_test_user, get_test_aws_config},
    types::{Answer, ChoiceQuestion, EntityType, Question},
    utils::{aws::DynamoClient, time::get_now_timestamp_millis},
};

#[tokio::test]
async fn test_poll_space_creation() {
    let cli = DynamoClient::mock(get_test_aws_config()).client;

    let user = create_test_user(&cli).await;

    let post = Post::new(
        "This is a test post".to_string(),
        "Content of the post".to_string(),
        crate::types::PostType::Post,
        user.clone(),
    );

    let common = SpaceCommon::new(post.pk, user.clone());

    common
        .create(&cli)
        .await
        .expect("failed to create space common");

    let now = get_now_timestamp_millis();
    let poll = Poll::new(common.pk.clone(), false, now, now + 60).unwrap();

    poll.create(&cli).await.expect("failed to create poll");

    let questions = vec![
        Question::SingleChoice(ChoiceQuestion {
            title: "What is your favorite color?".to_string(),
            description: Some("Choose one color".to_string()),
            image_url: None,
            options: vec!["Red".to_string(), "Blue".to_string(), "Green".to_string()],
            is_required: Some(true),
        }),
        Question::MultipleChoice(ChoiceQuestion {
            title: "What is your favorite color?".to_string(),
            description: Some("Choose multiple colors".to_string()),
            image_url: None,
            options: vec!["Red".to_string(), "Blue".to_string(), "Green".to_string()],
            is_required: Some(true),
        }),
    ];

    let question = PollQuestion::new(poll.pk.clone(), questions);

    question
        .create(&cli)
        .await
        .expect("failed to create question");

    let metadata = PollMetadata::query(&cli, &poll.pk)
        .await
        .expect("failed to query poll space metadata");

    assert_eq!(metadata.len(), 3, "should have 3 entries");

    let response: PollResponse = metadata.into();

    assert_eq!(response.questions.len(), 2, "should have 2 questions");

    PollUserResponse::new(
        poll.pk.clone(),
        user.pk.clone(),
        vec![
            Answer::SingleChoice { answer: Some(0) },
            Answer::MultipleChoice {
                answer: Some(vec![1]),
            },
        ],
    )
    .create(&cli)
    .await
    .expect("failed to create user survey response");

    let (res, _) = PollUserResponse::find_by_space_pk(
        &cli,
        &EntityType::SpacePollUserResponse(poll.pk.to_string()),
        Default::default(),
    )
    .await
    .expect("failed to find spaces survey response");
    assert_eq!(res.len(), 1, "should have 1 response");

    // PollSpaceSurveyResponse::new(
    //     poll.pk.clone(),
    //     user.pk.clone(),
    //     vec![
    //         Answer::SingleChoice { answer: Some(0) },
    //         Answer::MultipleChoice {
    //             answer: Some(vec![1]),
    //         },
    //     ],
    // )
    // .create(&cli)
    // .await
    // .expect("failed to create user survey response");

    let (res, _) = PollUserResponse::find_by_space_pk(
        &cli,
        &EntityType::SpacePollUserResponse(poll.pk.to_string()),
        Default::default(),
    )
    .await
    .expect("failed to find spaces survey response");
    assert_eq!(res.len(), 1, "should have 1 response");

    let my_survey = PollUserResponse::find_one(&cli, &poll.pk, &user.pk)
        .await
        .expect("failed to get my survey response");

    assert!(my_survey.is_some(), "should have my survey response");
    let my_survey = my_survey.unwrap();
    assert_eq!(my_survey.answers.len(), 2, "should have 2 answers");
}
