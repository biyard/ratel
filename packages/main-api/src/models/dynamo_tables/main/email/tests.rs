use std::{thread::sleep, time::Duration};

use super::*;

#[tokio::test]
async fn test_email_verification_new() {
    let conf = aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build();

    let cli = aws_sdk_dynamodb::Client::from_conf(conf);
    let now = chrono::Utc::now().timestamp();
    let expired_at = now + 3600; // 1 hour later
    let email = format!("a+{}@example.com", now);

    let ev = EmailVerification::new(email.to_string(), "aaaa".to_string(), expired_at);

    assert_eq!(EmailVerification::table_name(), "ratel-local-main");
    assert_eq!(EmailVerification::pk_field(), "pk");
    assert_eq!(EmailVerification::sk_field(), Some("sk"));

    assert!(
        ev.create(&cli).await.is_ok(),
        "failed to create email verification"
    );

    let fetched_ev = EmailVerification::get(&cli, ev.pk.clone(), Some(ev.sk)).await;

    assert!(fetched_ev.is_ok(), "failed to fetch email verification");
    let fetched_ev = fetched_ev.unwrap();
    assert!(fetched_ev.is_some(), "email verification not found");
    let fetched_ev = fetched_ev.unwrap();
    assert_eq!(fetched_ev.email, ev.email);
    assert_eq!(fetched_ev.value, ev.value);
    assert_eq!(fetched_ev.expired_at, ev.expired_at);
}

#[tokio::test]
async fn test_email_verification_delete() {
    let conf = aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build();

    let cli = aws_sdk_dynamodb::Client::from_conf(conf);
    let now = chrono::Utc::now().timestamp();
    let expired_at = now + 3600; // 1 hour later
    let email = format!("d+{}@example.com", now);
    let ev = EmailVerification::new(email.to_string(), "aaaa".to_string(), expired_at);
    assert!(
        ev.create(&cli).await.is_ok(),
        "failed to create email verification"
    );
    let fetched_ev = EmailVerification::get(&cli, ev.pk.clone(), Some(ev.sk)).await;
    assert!(fetched_ev.is_ok(), "failed to fetch email verification");
    let fetched_ev = fetched_ev.unwrap();
    assert!(fetched_ev.is_some(), "email verification not found");
    let fetched_ev = fetched_ev.unwrap();
    assert_eq!(fetched_ev.email, ev.email);
    assert_eq!(fetched_ev.value, ev.value);
    assert_eq!(fetched_ev.expired_at, ev.expired_at);
    assert!(
        EmailVerification::delete(&cli, ev.pk.clone(), Some(ev.sk))
            .await
            .is_ok(),
        "failed to delete email verification"
    );
    let fetched_ev = EmailVerification::get(&cli, ev.pk.clone(), Some(ev.sk)).await;
    assert!(fetched_ev.is_ok(), "failed to fetch email verification");
    let fetched_ev = fetched_ev.unwrap();
    assert!(fetched_ev.is_none(), "email verification should be deleted");
}

#[tokio::test]
async fn test_email_verification_find_by_email_and_code() {
    let conf = aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build();

    let cli = aws_sdk_dynamodb::Client::from_conf(conf);
    let now = chrono::Utc::now().timestamp();
    let expired_at = now + 3600; // 1 hour later
    for i in 0..5 {
        let email = format!("l+{now}-{i}@example.com");

        let ev = EmailVerification::new(email.to_string(), "aaaa".to_string(), expired_at);
        assert!(
            ev.create(&cli).await.is_ok(),
            "failed to create email verification"
        );
    }

    let fetched_evs = EmailVerification::find_by_email_and_code(
        &cli,
        format!("EMAIL#l+{now}-0@example.com"),
        EmailVerificationQueryOption::builder()
            .limit(10)
            .sk("a".to_string()),
    )
    .await;
    assert!(fetched_evs.is_ok(), "failed to find email verification");
    let (fetched_evs, last_evaluated_key) = fetched_evs.unwrap();
    assert!(
        last_evaluated_key.is_none(),
        "last_evaluated_key should be empty"
    );
    assert_eq!(fetched_evs.len(), 1, "should find one email verification");
    assert_eq!(fetched_evs[0].email, format!("l+{now}-0@example.com"));
}

#[tokio::test]
async fn test_email_verification_find_by_code() {
    let conf = aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build();

    let cli = aws_sdk_dynamodb::Client::from_conf(conf);
    let now = chrono::Utc::now().timestamp();
    let expired_at = now + 3600; // 1 hour later
    for i in 0..5 {
        let email = format!("c+{now}-{i}@example.com");

        let ev = EmailVerification::new(email.to_string(), "aaaa".to_string(), expired_at);
        assert!(
            ev.create(&cli).await.is_ok(),
            "failed to create email verification"
        );
    }

    sleep(Duration::from_millis(500));

    let fetched_evs = EmailVerification::find_by_code(
        &cli,
        format!("aaaa"),
        EmailVerificationQueryOption::builder()
            .limit(4)
            .sk("TS".to_string()),
    )
    .await;
    assert!(fetched_evs.is_ok(), "failed to find email verification");
    let (fetched_evs, last_evaluated_key) = fetched_evs.unwrap();

    println!("fetched_evs: {:?}", fetched_evs.len());
    assert!(
        last_evaluated_key.is_some(),
        "last_evaluated_key should not be empty"
    );
    assert_eq!(fetched_evs.len(), 4, "should find one email verification");
    assert_eq!(fetched_evs[0].email, format!("c+{now}-4@example.com"));
    assert_eq!(fetched_evs[1].email, format!("c+{now}-3@example.com"));
    assert_eq!(fetched_evs[2].email, format!("c+{now}-2@example.com"));
    assert_eq!(fetched_evs[3].email, format!("c+{now}-1@example.com"));
}
