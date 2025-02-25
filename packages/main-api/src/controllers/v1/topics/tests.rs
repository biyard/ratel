use super::*;
use crate::tests::{setup, TestContext};
use dto::*;

#[tokio::test]
async fn test_topic() {
    let context = TestContext {
        pool,
        user,
        endpoint,
        ..
    } = setup().await.unwrap();

    let cli_topic = Topic::get_client(&endpoint);

    test_create_topic(context.clone(), cli_topic.clone());
}

async fn test_create_topic(context: TestContext, cli: TopicClient) {
    let ended_at = context.now + 3600;
    let topic = cli
        .create(
            ended_at,
            "test title".to_string(),
            "test content".to_string(),
            TopicStatus::Scheduled,
            "https://test.com".to_string(),
            "test solutions".to_string(),
            vec![
                "test discussions1".to_string(),
                "test discussions2".to_string(),
            ],
            vec![
                AdditionalResource {
                    filename: "test additional_resources1".to_string(),
                    extension: "pdf".to_string(),
                    link: "https://test.com".to_string(),
                },
                AdditionalResource {
                    filename: "test additional_resources2".to_string(),
                    extension: "pdf".to_string(),
                    link: "https://test.com".to_string(),
                },
            ],
        )
        .await
        .unwrap();

    assert_eq!(topic.title, "test title");
    assert_eq!(topic.content, "test content");
    assert_eq!(topic.status, TopicStatus::Scheduled);
    assert_eq!(topic.legislation_link, "https://test.com");
    assert_eq!(topic.solutions, "test solutions");
    assert_eq!(topic.discussions.len(), 2);
    assert_eq!(topic.additional_resources.len(), 2);
}
