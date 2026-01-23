use crate::{
    controllers::v3::teams::{
        create_team::CreateTeamResponse,
        delete_team::DeleteTeamResponse,
        get_team::GetTeamResponse,
        groups::{create_group::CreateGroupResponse, delete_group::DeleteGroupResponse},
        list_members::TeamMember,
        update_team::UpdateTeamResponse,
    },
    tests::v3_setup::TestContextV3,
    types::{EntityType, Partition, TeamGroupPermission, list_items_response::ListItemsResponse},
};

use crate::*;

#[tokio::test]
async fn test_update_team_without_permission() {
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

    // Try to update with another user (should fail)
    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/teams/{}", team.team_pk),
        headers: user2.1.clone(),
        body: {
            "nickname": "Updated Squad",
            "description": "This is an updated squad"
        }
    };

    assert_eq!(status, 401, "Update should fail without permission");

    // Try to update without auth (should fail)
    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/teams/{}", team.team_pk),
        body: {
            "nickname": "Updated Squad"
        }
    };

    assert_eq!(status, 401, "Update should fail without auth");
}

#[tokio::test]
async fn test_update_team() {
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

    // Update team
    let new_nickname = "Updated Squad Name";
    let new_description = "Updated squad description";
    let new_profile_url = "https://example.com/updated_profile.png";

    let (status, _headers, _updated_team) = patch! {
        app: app,
        path: format!("/v3/teams/{}", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "nickname": new_nickname,
            "description": new_description,
            "profile_url": new_profile_url
        },
        response_type: UpdateTeamResponse
    };

    assert_eq!(status, 200, "Failed to update team");

    // Get team to verify updates
    let (status, _headers, team_response) = get! {
        app: app,
        path: format!("/v3/teams/{}", team.team_pk),
        headers: test_user.1.clone(),
        response_type: GetTeamResponse
    };

    assert_eq!(status, 200, "Failed to get team");
    assert_eq!(team_response.team.nickname, new_nickname);
    assert_eq!(team_response.team.html_contents, new_description);
    assert_eq!(team_response.team.profile_url.unwrap(), new_profile_url);
}

#[tokio::test]
async fn test_get_team() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());
    let team_nickname = format!("{}'s Squad", team_username);

    // Create team
    let (status, _headers, team) = post! {
        app: app,
        path: "/v3/teams",
        headers: test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": team_nickname,
            "profile_url": "https://metadata.ratel.foundation/ratel/default-profile.png",
            "description": "This is a squad for verification"
        },
        response_type: CreateTeamResponse
    };

    assert_eq!(status, 200, "Failed to create team");

    // Get team
    let (status, _headers, team_response) = get! {
        app: app,
        path: format!("/v3/teams/{}", team.team_pk),
        headers: test_user.1.clone(),
        response_type: GetTeamResponse
    };

    assert_eq!(status, 200, "Failed to get team");
    assert_eq!(team_response.team.nickname, team_nickname);
    assert_eq!(team_response.team.username, team_username);

    let owner = team_response.owner.expect("Owner should exist");
    // owner.id now contains just the UUID (not USER#uuid format)
    let user_uuid = match &test_user.0.pk {
        Partition::User(uuid) => uuid.clone(),
        _ => test_user.0.pk.to_string(),
    };
    assert_eq!(owner.id, user_uuid);
}

#[tokio::test]
async fn test_list_members() {
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

    // Create a group
    let (status, _headers, group) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "name": "Group for Verification",
            "description": "A group for verification purposes",
            "image_url": "https://example.com/image.png",
            "permissions": [TeamGroupPermission::PostWrite]
        },
        response_type: CreateGroupResponse
    };

    assert_eq!(status, 200, "Failed to create group");

    // Extract UUID from EntityType for path parameter
    let group_id = match group.group_sk {
        EntityType::TeamGroup(ref id) => id.clone(),
        _ => panic!("Expected TeamGroup EntityType"),
    };

    // Add user2 as member
    let (status, _headers, _add_result) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups/{}/member", team.team_pk, group_id),
        headers: test_user.1.clone(),
        body: {
            "user_pks": [user2.0.pk.to_string()]
        }
    };

    assert_eq!(status, 200, "Failed to add member");

    // List members - using team username in path
    let (status, _headers, members_response) = get! {
        app: app,
        path: format!("/v3/teams/{}/members", team_username),
        headers: test_user.1.clone(),
        response_type: ListItemsResponse<TeamMember>
    };

    assert_eq!(status, 200, "Failed to list members");
    assert_eq!(
        members_response.items.len(),
        2,
        "Should have 2 members (owner + added)"
    );

    let owner_member = members_response
        .items
        .iter()
        .find(|m| m.is_owner)
        .expect("Should have owner");
    assert_eq!(owner_member.user_id, test_user.0.pk.to_string());
}

#[tokio::test]
async fn test_delete_team() {
    let ctx = TestContextV3::setup().await;
    let user2 = ctx.create_another_user().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    let team_username = format!("testteam{}", uuid::Uuid::new_v4());

    // Create team
    let (status, _headers, _team) = post! {
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

    // Try to delete with non-owner (should fail)
    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/teams/{}", team_username),
        headers: user2.1.clone()
    };

    assert_eq!(status, 401, "Delete should fail for non-owner");

    // Delete with owner (should succeed)
    let (status, _headers, delete_response) = delete! {
        app: app,
        path: format!("/v3/teams/{}", team_username),
        headers: test_user.1.clone(),
        response_type: DeleteTeamResponse
    };

    assert_eq!(status, 200, "Failed to delete team");
    assert!(
        delete_response.deleted_count > 0,
        "Should have deleted entities"
    );
}

#[tokio::test]
async fn test_delete_group() {
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

    // Create a group
    let (status, _headers, group) = post! {
        app: app,
        path: format!("/v3/teams/{}/groups", team.team_pk),
        headers: test_user.1.clone(),
        body: {
            "name": "Group for Verification",
            "description": "A group for verification purposes",
            "image_url": "https://example.com/image.png",
            "permissions": [TeamGroupPermission::PostWrite]
        },
        response_type: CreateGroupResponse
    };

    assert_eq!(status, 200, "Failed to create group");

    // Extract UUID from EntityType for path parameter
    let group_id = match group.group_sk {
        EntityType::TeamGroup(ref id) => id.clone(),
        _ => panic!("Expected TeamGroup EntityType"),
    };

    // Delete group
    let (status, _headers, delete_response) = delete! {
        app: app,
        path: format!("/v3/teams/{}/groups/{}", team_username, group_id),
        headers: test_user.1.clone(),
        response_type: DeleteGroupResponse
    };

    assert_eq!(status, 200, "Failed to delete group");
    assert!(delete_response.message.contains("successfully deleted"));

    // Verify group is deleted - get team should not include the group
    let (status, _headers, team_response) = get! {
        app: app,
        path: format!("/v3/teams/{}", team.team_pk),
        headers: test_user.1.clone(),
        response_type: GetTeamResponse
    };

    assert_eq!(status, 200);
    let groups = team_response.groups.unwrap_or_default();
    // group.id now contains just the UUID (not TEAM_GROUP#uuid format)
    let group_uuid = match &group.group_sk {
        EntityType::TeamGroup(uuid) => uuid.to_string(),
        _ => group.group_sk.to_string(),
    };
    assert!(
        !groups.iter().any(|g| g.id == group_uuid),
        "Group should be deleted"
    );
}

#[tokio::test]
async fn test_list_team_posts() {
    use crate::controllers::v3::posts::create_post::CreatePostResponse;
    use crate::controllers::v3::posts::post_response::PostResponse;
    use crate::types::list_items_response::ListItemsResponse;

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

    // Create a post for the team
    let (status, _headers, post) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        body: {
            "team_pk": team.team_pk.to_string()
        },
        response_type: CreatePostResponse
    };

    assert_eq!(status, 200, "Failed to create post");

    // Update the post to publish it
    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post.post_pk),
        headers: test_user.1.clone(),
        body: {
            "title": "Test Post",
            "content": "<p>Test Content</p>",
            "publish": true
        }
    };

    assert_eq!(status, 200, "Failed to publish post");

    // List team posts with status filter for published
    let (status, _headers, posts_response) = get! {
        app: app,
        path: format!("/v3/teams/{}/posts?status=2", team.team_pk),
        headers: test_user.1.clone(),
        response_type: ListItemsResponse<PostResponse>
    };

    assert_eq!(status, 200, "Failed to list team posts");
    assert_eq!(
        posts_response.items.len(),
        1,
        "Should have 1 published post"
    );

    let post_item = &posts_response.items[0];
    assert_eq!(post_item.auth_pk, team.team_pk);
}
