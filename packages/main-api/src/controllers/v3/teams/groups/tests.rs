#![allow(warnings)]
use crate::{
    controllers::v3::teams::{
        create_team::CreateTeamResponse,
        get_team::GetTeamResponse,
        groups::{
            add_member::AddMemberResponse, create_group::CreateGroupResponse,
            delete_group::DeleteGroupResponse,
        },
    },
    tests::v3_setup::TestContextV3,
    types::{EntityType, TeamGroupPermission},
};

use crate::*;

#[tokio::test]
async fn test_update_group_handler() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create a team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Team", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a team for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Create a team group
    let (status, _headers, team_group) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "name": "Group for Verification",
            "description": "A group for verification purposes",
            "image_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "permissions": [TeamGroupPermission::GroupEdit]
        },
        response_type: CreateGroupResponse
    };

    assert_eq!(status, 200, "Failed to create team group");

    // Extract UUID from EntityType for path parameter
    let _group_id = match team_group.group_sk {
        EntityType::TeamGroup(ref id) => id.clone(),
        _ => panic!("Expected TeamGroup EntityType"),
    };

    // Update group
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups/{}", team.team_pk, team_group.group_sk ),
        headers: test_user.1.clone(),
        body: {
            "name": "Updated Group Name",
            "description": "Updated description",
            "permissions": [
                TeamGroupPermission::GroupEdit,
                TeamGroupPermission::TeamEdit
            ]
        }
    };

    assert_eq!(status, 200, "Failed to update group");
}

#[tokio::test]
async fn test_update_with_permissison() {
    let ctx = TestContextV3::setup().await;
    let user2 = ctx.create_another_user().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create a team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Team", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a team for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Create a team group
    let (status, _headers, team_group) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "name": "Group for Verification",
            "description": "A group for verification purposes",
            "image_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "permissions": [TeamGroupPermission::GroupEdit]
        },
        response_type: CreateGroupResponse
    };

    assert_eq!(status, 200, "Failed to create team group");

    // Extract UUID from EntityType for path parameter
    let group_id = match team_group.group_sk {
        EntityType::TeamGroup(ref id) => id.clone(),
        _ => panic!("Expected TeamGroup EntityType"),
    };

    // Add user2 as member to the group
    let (status, _headers, _add_result) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups/{}/member", team.team_pk, group_id),
        headers: test_user.1.clone(),
        body: {
            "user_pks": [user2.0.pk.to_string()]
        },
        response_type: AddMemberResponse
    };

    assert_eq!(status, 200, "Failed to add member to group");

    // Update group with user2 (who has GroupEdit permission)
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups/{}", team.team_pk, team_group.group_sk),
        headers: user2.1.clone(),
        body: {
            "name": "Updated by Member",
            "description": "Updated by a group member"
        }
    };

    assert_eq!(
        status, 200,
        "Member with GroupEdit permission should be able to update group"
    );
}

#[tokio::test]
async fn test_add_member_handler() {
    let ctx = TestContextV3::setup().await;
    let user2 = ctx.create_another_user().await;
    let TestContextV3 { app, test_user, .. } = ctx;
    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create a team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": format!("{}'s Team", team_username),
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a team for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Create a team group
    let (status, _headers, team_group) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "name": "Group for Verification",
            "description": "A group for verification purposes",
            "image_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "permissions": [TeamGroupPermission::PostWrite]
        },
        response_type: CreateGroupResponse
    };

    assert_eq!(status, 200, "Failed to create team group");

    // Extract UUID from EntityType for path parameter
    let group_id = match team_group.group_sk {
        EntityType::TeamGroup(ref id) => id.clone(),
        _ => panic!("Expected TeamGroup EntityType"),
    };

    // Add members to the group
    let (status, _headers, add_result) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups/{}/member", team.team_pk, group_id),
        headers: test_user.1.clone(),
        body: {
            "user_pks": [user2.0.pk.to_string()]
        },
        response_type: AddMemberResponse
    };

    assert_eq!(status, 200, "Failed to add members");
    assert_eq!(add_result.total_added, 1, "Should have added 1 member");
    assert_eq!(add_result.failed_pks.len(), 0, "Should have no failed adds");

    // Get team and verify group exists
    let (status, _headers, team_response) = get! {
        app: app,
        path: format!("/v3/teams/{}", team.team_pk),
        headers: test_user.1.clone(),
        response_type: GetTeamResponse
    };

    assert_eq!(status, 200, "Failed to get team");
    let groups = team_response.groups.unwrap_or_default();
    // group.id now contains just the UUID (not TEAM_GROUP#uuid format)
    let group_uuid = match &team_group.group_sk {
        EntityType::TeamGroup(uuid) => uuid.clone(),
        _ => team_group.group_sk.to_string(),
    };
    assert!(
        groups.iter().any(|g| g.id == group_uuid),
        "Team group should exist"
    );
}
