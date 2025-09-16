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
async fn test_team_creation() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let team = Team::new(
        format!("test-team-{}", now),
        "Test Team".to_string(),
        format!("space-{}", now),
        "user123".to_string(),
        false,
    );

    assert!(team.create(&cli).await.is_ok(), "failed to create team");

    let team_id = team.team_id().unwrap();
    let fetched = Team::get(&cli, Partition::Team(team_id), Some(EntityType::Team)).await;

    assert!(fetched.is_ok(), "failed to fetch team");
    let fetched = fetched.unwrap();
    assert!(fetched.is_some(), "team not found");
    let fetched = fetched.unwrap();

    assert_eq!(fetched.name, team.name);
    assert_eq!(fetched.display_name, team.display_name);
    assert_eq!(fetched.space_id, team.space_id);
    assert_eq!(fetched.created_by, team.created_by);
    assert_eq!(fetched.is_private, team.is_private);
    assert!(!fetched.is_archived);
}

#[tokio::test]
async fn test_team_archive_unarchive() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let mut team = Team::new(
        format!("test-team-archive-{}", now),
        "Test Team Archive".to_string(),
        format!("space-{}", now),
        "user123".to_string(),
        false,
    );

    assert!(team.create(&cli).await.is_ok(), "failed to create team");

    // Test archive
    team.archive();
    assert!(team.update(&cli).await.is_ok(), "failed to update archived team");
    assert!(team.is_archived);

    // Test unarchive
    team.unarchive();
    assert!(team.update(&cli).await.is_ok(), "failed to update unarchived team");
    assert!(!team.is_archived);
}

#[tokio::test]
async fn test_team_find_by_name() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let team_name = format!("test-team-name-{}", now);

    let team = Team::new(
        team_name.clone(),
        "Test Team by Name".to_string(),
        format!("space-{}", now),
        "user456".to_string(),
        false,
    );

    assert!(team.create(&cli).await.is_ok(), "failed to create team");

    let found_teams = Team::find_by_name(
        &cli,
        format!("NAME#{}", team_name),
        TeamQueryOption::builder().limit(10),
    ).await;

    assert!(found_teams.is_ok(), "failed to find team by name");
    let (found_teams, _) = found_teams.unwrap();
    assert_eq!(found_teams.len(), 1, "should find one team");
    assert_eq!(found_teams[0].name, team_name);
}

#[tokio::test]
async fn test_team_find_by_space() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let space_id = format!("space-teams-{}", now);

    for i in 0..3 {
        let team = Team::new(
            format!("test-team-space-{}-{}", now, i),
            format!("Test Team Space {}", i),
            space_id.clone(),
            "user789".to_string(),
            i % 2 == 0, // Alternate private/public
        );
        assert!(team.create(&cli).await.is_ok(), "failed to create team");
    }

    let found_teams = Team::find_by_space(
        &cli,
        format!("SPACE#{}", space_id),
        TeamQueryOption::builder().limit(10),
    ).await;

    assert!(found_teams.is_ok(), "failed to find teams by space");
    let (found_teams, _) = found_teams.unwrap();
    assert_eq!(found_teams.len(), 3, "should find three teams");

    for team in found_teams {
        assert_eq!(team.space_id, space_id);
    }
}

#[tokio::test]
async fn test_team_member_creation() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let team_id = format!("team-{}", now);
    let user_id = format!("user-{}", now);

    let member = TeamMember::new(
        team_id.clone(),
        user_id.clone(),
        "admin123".to_string(),
        TeamMemberRole::Member,
    );

    assert!(member.create(&cli).await.is_ok(), "failed to create team member");

    let fetched = TeamMember::get(&cli, Partition::Team(team_id), Some(EntityType::TeamMember)).await;

    assert!(fetched.is_ok(), "failed to fetch team member");
    let fetched = fetched.unwrap();
    assert!(fetched.is_some(), "team member not found");
    let fetched = fetched.unwrap();

    assert_eq!(fetched.user_id, user_id);
    assert_eq!(fetched.team_id, member.team_id);
    assert_eq!(fetched.joined_by, "admin123");
    assert!(fetched.is_active);
    assert!(matches!(fetched.role, TeamMemberRole::Member));
}

#[tokio::test]
async fn test_team_member_role_management() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let team_id = format!("team-roles-{}", now);
    let user_id = format!("user-roles-{}", now);

    let mut member = TeamMember::new(
        team_id.clone(),
        user_id.clone(),
        "admin123".to_string(),
        TeamMemberRole::Member,
    );

    assert!(member.create(&cli).await.is_ok(), "failed to create team member");

    // Test promotion to lead
    member.promote_to_lead();
    assert!(member.update(&cli).await.is_ok(), "failed to update to lead");
    assert!(matches!(member.role, TeamMemberRole::Lead));
    assert!(member.can_manage_team());
    assert!(member.can_invite_members());

    // Test promotion to admin
    member.promote_to_admin();
    assert!(member.update(&cli).await.is_ok(), "failed to update to admin");
    assert!(matches!(member.role, TeamMemberRole::Admin));
    assert!(member.can_manage_team());

    // Test demotion to member
    member.demote_to_member();
    assert!(member.update(&cli).await.is_ok(), "failed to demote to member");
    assert!(matches!(member.role, TeamMemberRole::Member));
    assert!(!member.can_manage_team());
    assert!(!member.can_invite_members());
}

#[tokio::test]
async fn test_team_member_activation() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let team_id = format!("team-activation-{}", now);
    let user_id = format!("user-activation-{}", now);

    let mut member = TeamMember::new(
        team_id.clone(),
        user_id.clone(),
        "admin123".to_string(),
        TeamMemberRole::Member,
    );

    assert!(member.create(&cli).await.is_ok(), "failed to create team member");

    // Test deactivation
    member.deactivate();
    assert!(member.update(&cli).await.is_ok(), "failed to deactivate member");
    assert!(!member.is_active);

    // Test reactivation
    member.reactivate();
    assert!(member.update(&cli).await.is_ok(), "failed to reactivate member");
    assert!(member.is_active);
}

#[tokio::test]
async fn test_team_member_find_by_user() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let user_id = format!("user-teams-{}", now);

    for i in 0..3 {
        let team_id = format!("team-{}-{}", now, i);
        let role = if i == 0 { TeamMemberRole::Lead } else { TeamMemberRole::Member };
        let member = TeamMember::new(
            team_id,
            user_id.clone(),
            "admin123".to_string(),
            role,
        );
        assert!(member.create(&cli).await.is_ok(), "failed to create team member");
    }

    let found_members = TeamMember::find_by_user(
        &cli,
        format!("USER#{}", user_id),
        TeamMemberQueryOption::builder().limit(10),
    ).await;

    assert!(found_members.is_ok(), "failed to find members by user");
    let (found_members, _) = found_members.unwrap();
    assert_eq!(found_members.len(), 3, "should find three team memberships");

    for member in &found_members {
        assert_eq!(member.user_id, user_id);
    }

    // Check that one is lead
    let lead_count = found_members.iter()
        .filter(|m| matches!(m.role, TeamMemberRole::Lead))
        .count();
    assert_eq!(lead_count, 1, "should have exactly one lead role");
}

#[tokio::test]
async fn test_team_member_find_by_team() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let team_id = format!("team-members-{}", now);

    for i in 0..4 {
        let user_id = format!("user-{}-{}", now, i);
        let role = match i {
            0 => TeamMemberRole::Admin,
            1 => TeamMemberRole::Lead,
            _ => TeamMemberRole::Member,
        };
        let member = TeamMember::new(
            team_id.clone(),
            user_id,
            "admin123".to_string(),
            role,
        );
        assert!(member.create(&cli).await.is_ok(), "failed to create team member");
    }

    let found_members = TeamMember::find_by_team(
        &cli,
        format!("TEAM#{}", team_id),
        TeamMemberQueryOption::builder().limit(10),
    ).await;

    assert!(found_members.is_ok(), "failed to find members by team");
    let (found_members, _) = found_members.unwrap();
    assert_eq!(found_members.len(), 4, "should find four members");

    for member in &found_members {
        assert_eq!(member.team_id, team_id);
    }

    // Check role distribution
    let admin_count = found_members.iter()
        .filter(|m| matches!(m.role, TeamMemberRole::Admin))
        .count();
    let lead_count = found_members.iter()
        .filter(|m| matches!(m.role, TeamMemberRole::Lead))
        .count();
    let member_count = found_members.iter()
        .filter(|m| matches!(m.role, TeamMemberRole::Member))
        .count();

    assert_eq!(admin_count, 1, "should have exactly one admin");
    assert_eq!(lead_count, 1, "should have exactly one lead");
    assert_eq!(member_count, 2, "should have exactly two members");
}

#[tokio::test]
async fn test_team_member_custom_title() {
    let conf = create_test_config();
    let cli = aws_sdk_dynamodb::Client::from_conf(conf);

    let now = chrono::Utc::now().timestamp();
    let team_id = format!("team-title-{}", now);
    let user_id = format!("user-title-{}", now);

    let mut member = TeamMember::new(
        team_id.clone(),
        user_id.clone(),
        "admin123".to_string(),
        TeamMemberRole::Member,
    );

    assert!(member.create(&cli).await.is_ok(), "failed to create team member");

    // Test setting custom title
    member.update_custom_title(Some("Senior Developer".to_string()));
    assert!(member.update(&cli).await.is_ok(), "failed to update custom title");
    assert_eq!(member.custom_title, Some("Senior Developer".to_string()));

    // Test removing custom title
    member.update_custom_title(None);
    assert!(member.update(&cli).await.is_ok(), "failed to remove custom title");
    assert_eq!(member.custom_title, None);
}