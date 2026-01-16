use crate::{
    controllers::v3::teams::create_team::CreateTeamResponse, features::membership::*,
    tests::v3_setup::TestContextV3, types::EntityType, *,
};

use super::get_team_membership_history::TeamPurchaseHistoryItem;
use crate::features::membership::ChangeTeamMembershipResponse;

#[tokio::test]
async fn test_get_team_membership_as_owner() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Squad", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a squad for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Get team membership as owner
    let (status, _headers, membership_response) = get! {
        app: app,
        path: format!("/v3/teams/{}/membership", team.team_pk),
        headers: test_user.1.clone(),
        response_type: TeamMembershipResponse
    };

    assert_eq!(status, 200, "Failed to get team membership");
    assert!(
        membership_response.is_owner,
        "User should be marked as owner"
    );
    // Default should be Free tier
    assert!(
        membership_response.tier.to_string().contains("FREE"),
        "Default membership should be Free, got: {}",
        membership_response.tier
    );
}

#[tokio::test]
async fn test_get_team_membership_as_member() {
    let ctx = TestContextV3::setup().await;
    let user2 = ctx.create_another_user().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Squad", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a squad for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Get team membership as non-owner
    let (status, _headers, membership_response) = get! {
        app: app,
        path: format!("/v3/teams/{}/membership", team.team_pk),
        headers: user2.1.clone(),
        response_type: TeamMembershipResponse
    };

    assert_eq!(status, 200, "Failed to get team membership");
    assert!(
        !membership_response.is_owner,
        "User should not be marked as owner"
    );
}

#[tokio::test]
async fn test_get_team_membership_not_authenticated() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Squad", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a squad for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Get team membership without authentication - should work but is_owner should be false
    let (status, _headers, membership_response) = get! {
        app: app,
        path: format!("/v3/teams/{}/membership", team.team_pk),
        response_type: TeamMembershipResponse
    };

    assert_eq!(status, 200, "Failed to get team membership");
    assert!(
        !membership_response.is_owner,
        "Anonymous user should not be marked as owner"
    );
}

#[tokio::test]
async fn test_change_team_membership_owner_only() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Squad", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a squad for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Upgrade team membership to Pro
    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/teams/{}/membership", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: serde_json::Value
    };

    if status != 200 {
        println!("Error response: {:?}", body);
    }
    assert_eq!(
        status, 200,
        "Failed to upgrade team membership. Response: {:?}",
        body
    );

    // Verify the membership was upgraded
    let team_membership =
        TeamMembership::get(&ddb, team.team_pk.clone(), Some(EntityType::TeamMembership))
            .await
            .unwrap()
            .expect("TeamMembership should exist");

    let membership_pk: Partition = team_membership.membership_pk.clone().into();
    let membership = Membership::get(&ddb, membership_pk, Some(EntityType::Membership))
        .await
        .unwrap()
        .expect("Membership should exist");

    assert_eq!(membership.tier, MembershipTier::Pro);
}

#[tokio::test]
async fn test_change_team_membership_denied_for_non_owner() {
    let ctx = TestContextV3::setup().await;
    let user2 = ctx.create_another_user().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Squad", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a squad for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Try to upgrade team membership as non-owner
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/teams/{}/membership", team.team_pk),
        headers: user2.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: serde_json::Value
    };

    assert_eq!(
        status, 401,
        "Non-owner should not be able to change team membership"
    );
}

#[tokio::test]
async fn test_change_team_membership_downgrade_schedules() {
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = TestContextV3::setup().await;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Squad", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a squad for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // First upgrade to Pro
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/teams/{}/membership", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: ChangeTeamMembershipResponse
    };
    assert_eq!(status, 200, "Failed to upgrade to Pro");

    // Now downgrade to Free
    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/teams/{}/membership", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "membership": "Free",
            "currency": "USD"
        },
        response_type: ChangeTeamMembershipResponse
    };
    assert_eq!(status, 200, "Failed to schedule downgrade: {:?}", body);

    // Verify downgrade was scheduled (not applied immediately)
    let team_membership =
        TeamMembership::get(&ddb, team.team_pk.clone(), Some(EntityType::TeamMembership))
            .await
            .unwrap()
            .expect("TeamMembership should exist");

    let membership_pk: Partition = team_membership.membership_pk.clone().into();
    let membership = Membership::get(&ddb, membership_pk, Some(EntityType::Membership))
        .await
        .unwrap()
        .expect("Membership should exist");

    // Current membership should still be Pro
    assert_eq!(membership.tier, MembershipTier::Pro);
    // But next_membership should be set to Free
    assert!(
        team_membership.next_membership.is_some(),
        "next_membership should be set to Free"
    );
}

#[tokio::test]
async fn test_team_purchase_history() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Squad", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a squad for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Upgrade to Pro to create a purchase record
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/teams/{}/membership", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: ChangeTeamMembershipResponse
    };
    assert_eq!(status, 200, "Failed to upgrade membership");

    // Get purchase history
    let (status, _headers, history_response) = get! {
        app: app,
        path: format!("/v3/teams/{}/membership/history", team.team_pk),
        headers: test_user.1.clone(),
        response_type: ListResponse<TeamPurchaseHistoryItem>
    };

    assert_eq!(status, 200, "Failed to get purchase history");
    assert!(
        !history_response.items.is_empty(),
        "Should have at least one purchase record"
    );
}
