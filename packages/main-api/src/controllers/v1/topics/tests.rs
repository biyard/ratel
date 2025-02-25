use super::*;
use crate::tests::setup;
// use dto::*;

#[tokio::test]
async fn test_topic() {
    let context = setup().await.unwrap();

    let _cli_topic = Topic::get_client(&context.endpoint);

    // let _id = test_create_topic(context.now, cli_topic.clone()).await;
}

async fn test_create_topic(now: i64, cli: TopicClient) -> i64 {
    let ended_at = now + 3600;
    let res = cli
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
                    extension: FileType::Pdf,
                    link: "https://test.com".to_string(),
                },
                AdditionalResource {
                    filename: "test additional_resources2".to_string(),
                    extension: FileType::Image,
                    link: "https://test.com".to_string(),
                },
            ],
        )
        .await;

    tracing::debug!("test_create_topic: {:?}", res);
    assert!(res.is_ok(), "create topic failed: {:?}", res);
    res.unwrap().id
}
