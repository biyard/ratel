use super::*;
use crate::types::*;
use bdk::prelude::*;

fn create_test_config() -> aws_sdk_dynamodb::Config {
    aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build()
}

#[tokio::test]
async fn test_space_creation() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let space = Space::new(
        format!("test-space-{}", now),
        "Test Space".to_string(),
        "A test space for testing".to_string(),
        "user123".to_string(),
        true,
    );

    assert!(space.create(&cli).await.is_ok(), "failed to create space");

    let space_id = space.space_id().unwrap();
    let fetched = Space::get(&cli, Partition::Space(space_id), Some(EntityType::Space)).await;

    assert!(fetched.is_ok(), "failed to fetch space");
    let fetched = fetched.unwrap();
    assert!(fetched.is_some(), "space not found");
    let fetched = fetched.unwrap();

    assert_eq!(fetched.name, space.name);
    assert_eq!(fetched.display_name, space.display_name);
    assert_eq!(fetched.owner_id, space.owner_id);
    assert_eq!(fetched.is_public, space.is_public);
}

#[tokio::test]
async fn test_space_find_by_name() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let space_name = format!("test-space-name-{}", now);

    let space = Space::new(
        space_name.clone(),
        "Test Space by Name".to_string(),
        "A test space for name testing".to_string(),
        "user456".to_string(),
        true,
    );

    assert!(space.create(&cli).await.is_ok(), "failed to create space");

    let found_spaces = Space::find_by_name(
        &cli,
        format!("NAME#{}", space_name),
        SpaceQueryOption::builder().limit(10),
    ).await;

    assert!(found_spaces.is_ok(), "failed to find space by name");
    let (found_spaces, _) = found_spaces.unwrap();
    assert_eq!(found_spaces.len(), 1, "should find one space");
    assert_eq!(found_spaces[0].name, space_name);
}

#[tokio::test]
async fn test_space_find_by_owner() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let owner_id = format!("owner-{}", now);

    for i in 0..3 {
        let space = Space::new(
            format!("test-space-owner-{}-{}", now, i),
            format!("Test Space Owner {}", i),
            "A test space for owner testing".to_string(),
            owner_id.clone(),
            true,
        );
        assert!(space.create(&cli).await.is_ok(), "failed to create space");
    }

    let found_spaces = Space::find_by_owner(
        &cli,
        format!("OWNER#{}", owner_id),
        SpaceQueryOption::builder().limit(10),
    ).await;

    assert!(found_spaces.is_ok(), "failed to find spaces by owner");
    let (found_spaces, _) = found_spaces.unwrap();
    assert_eq!(found_spaces.len(), 3, "should find three spaces");

    for space in found_spaces {
        assert_eq!(space.owner_id, owner_id);
    }
}

#[tokio::test]
async fn test_space_member_creation() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let space_id = format!("space-{}", now);
    let user_id = format!("user-{}", now);

    let member = SpaceMember::new(
        space_id.clone(),
        user_id.clone(),
        SpaceMemberRole::Member,
    );

    assert!(member.create(&cli).await.is_ok(), "failed to create space member");

    let fetched = SpaceMember::get(&cli, Partition::Space(space_id), Some(EntityType::SpaceMember)).await;

    assert!(fetched.is_ok(), "failed to fetch space member");
    let fetched = fetched.unwrap();
    assert!(fetched.is_some(), "space member not found");
    let fetched = fetched.unwrap();

    assert_eq!(fetched.user_id, user_id);
    assert_eq!(fetched.space_id, member.space_id);
    assert!(!fetched.is_banned);
    assert!(fetched.permissions.can_post);
    assert!(!fetched.permissions.can_moderate);
}

#[tokio::test]
async fn test_space_member_ban_unban() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let space_id = format!("space-ban-{}", now);
    let user_id = format!("user-ban-{}", now);

    let mut member = SpaceMember::new(
        space_id.clone(),
        user_id.clone(),
        SpaceMemberRole::Member,
    );

    assert!(member.create(&cli).await.is_ok(), "failed to create space member");

    // Test ban
    member.ban("Spam posting".to_string(), "moderator123".to_string());
    assert!(member.update(&cli).await.is_ok(), "failed to update banned member");

    assert!(member.is_banned);
    assert_eq!(member.ban_reason, Some("Spam posting".to_string()));
    assert_eq!(member.banned_by, Some("moderator123".to_string()));

    // Test unban
    member.unban();
    assert!(member.update(&cli).await.is_ok(), "failed to update unbanned member");

    assert!(!member.is_banned);
    assert_eq!(member.ban_reason, None);
    assert_eq!(member.banned_by, None);
}

#[tokio::test]
async fn test_space_member_find_by_user() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let user_id = format!("user-spaces-{}", now);

    for i in 0..3 {
        let space_id = format!("space-{}-{}", now, i);
        let member = SpaceMember::new(
            space_id,
            user_id.clone(),
            SpaceMemberRole::Member,
        );
        assert!(member.create(&cli).await.is_ok(), "failed to create space member");
    }

    let found_members = SpaceMember::find_by_user(
        &cli,
        format!("USER#{}", user_id),
        SpaceMemberQueryOption::builder().limit(10),
    ).await;

    assert!(found_members.is_ok(), "failed to find members by user");
    let (found_members, _) = found_members.unwrap();
    assert_eq!(found_members.len(), 3, "should find three memberships");

    for member in found_members {
        assert_eq!(member.user_id, user_id);
    }
}

#[tokio::test]
async fn test_space_member_find_by_space() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let space_id = format!("space-members-{}", now);

    for i in 0..5 {
        let user_id = format!("user-{}-{}", now, i);
        let role = if i == 0 { SpaceMemberRole::Owner } else { SpaceMemberRole::Member };
        let member = SpaceMember::new(
            space_id.clone(),
            user_id,
            role,
        );
        assert!(member.create(&cli).await.is_ok(), "failed to create space member");
    }

    let found_members = SpaceMember::find_by_space(
        &cli,
        format!("SPACE#{}", space_id),
        SpaceMemberQueryOption::builder().limit(10),
    ).await;

    assert!(found_members.is_ok(), "failed to find members by space");
    let (found_members, _) = found_members.unwrap();
    assert_eq!(found_members.len(), 5, "should find five members");

    for member in &found_members {
        assert_eq!(member.space_id, space_id);
    }

    // Check that one is owner
    let owner_count = found_members.iter()
        .filter(|m| matches!(m.role, SpaceMemberRole::Owner))
        .count();
    assert_eq!(owner_count, 1, "should have exactly one owner");
}