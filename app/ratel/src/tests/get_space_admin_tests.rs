//! Regression coverage for `get_space` reading-side robustness when the
//! caller is the space admin/Creator.
//!
//! Bug under test: in a team-owned space (e.g. a sub-team broadcast
//! space), the parent-team admin who pressed Publish is the space's
//! Creator but never has a `SpaceParticipant` row — the broadcast
//! publish flow does not create one (unlike `create_space_handler`,
//! which inserts a participant for the user atomically with the space).
//!
//! Independently, `bump_participant_activity` used to issue a bare
//! `UpdateItem` against the (missing) participant key. DynamoDB upserts
//! such updates into a partial row carrying only the keys plus
//! `last_activity_at`, missing every required field on
//! `SpaceParticipant`. The next `get_space` call would then try to
//! `SpaceParticipant::get(...)?` that row and surface
//! `serde_dynamo: missing field 'created_at'` → "Something went wrong".
//!
//! These tests pin the admin-skip path so that:
//!   1. An admin viewing a team-owned space with NO participant row
//!      gets a successful response with `participated=false`.
//!   2. The same admin still gets a successful response even when a
//!      buggy partial participant row already exists in the table.
//!
//! Both cases exercise the `permissions.contains(SpaceEdit)` branch in
//! `get_space` that short-circuits the participant load.
use super::*;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::types::{
    EntityType, Partition, SpacePublishState, SpaceStatus, SpaceVisibility, UserType,
};
use crate::features::auth::UserTeam;
use crate::features::posts::models::Team;
use crate::features::posts::types::{PostStatus, Visibility};
use crate::features::social::pages::member::dto::TeamRole;

/// Owner = `ctx.test_user`. Returns the team's pk.
async fn create_team_owned_by_test_user(ctx: &TestContext) -> Partition {
    let owner = &ctx.test_user.0;
    let (team_pk, _created_at) = Team::create_new_team(
        owner,
        &ctx.ddb,
        format!("team{}", uuid::Uuid::new_v4()),
        String::new(),
        format!("team-{}", uuid::Uuid::new_v4().simple()),
        "desc".to_string(),
    )
    .await
    .expect("create team");
    team_pk
}

/// Insert a team-owned, published, public space directly into DDB and
/// return both the SpaceCommon row and its raw id string (no prefix).
/// The Post is seeded with `user_pk = team_pk` and `author_type =
/// Team`, mirroring real space creation, so `Post::get_permissions`
/// routes into the `Team::get_permissions_by_team_pk` branch and
/// returns admin/owner permissions for team admins (i.e. the
/// `SpaceEdit` bit that `get_space` uses to detect admins).
async fn insert_team_owned_space(ctx: &TestContext, team_pk: Partition) -> (SpaceCommon, String) {
    let post_id = uuid::Uuid::new_v4().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let space_pk = Partition::Space(post_id.clone());
    let post_pk = Partition::Feed(post_id.clone());

    let mut space = SpaceCommon::default();
    space.pk = space_pk.clone();
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = Some(SpaceStatus::Open);
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk.clone();
    space.user_pk = team_pk.clone();
    space.author_display_name = "team".to_string();
    space.author_profile_url = String::new();
    space.author_username = "team".to_string();
    space.create(&ctx.ddb).await.expect("create space");

    let post = crate::features::posts::models::Post {
        pk: post_pk,
        sk: EntityType::Post,
        created_at: now,
        updated_at: now,
        title: "Admin Test Space".to_string(),
        status: PostStatus::Published,
        visibility: Some(Visibility::Public),
        user_pk: team_pk,
        author_type: UserType::Team,
        ..Default::default()
    };
    post.create(&ctx.ddb).await.expect("create post");

    (space, post_id)
}

#[tokio::test]
async fn test_get_space_as_team_admin_without_participant_row_succeeds() {
    let ctx = TestContext::setup().await;
    let team_pk = create_team_owned_by_test_user(&ctx).await;
    let (_space, space_id) = insert_team_owned_space(&ctx, team_pk).await;

    // Admin (team owner) hits get_space. Should succeed with
    // participated=false / can_participate=false, never touching
    // SpaceParticipant.
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "expected 200, body: {body:?}");
    assert_eq!(
        body["participated"],
        serde_json::json!(false),
        "admin is not a participant; body: {body:?}"
    );
    assert_eq!(
        body["can_participate"],
        serde_json::json!(false),
        "admin shouldn't get a Join CTA on their own space; body: {body:?}"
    );
}

#[tokio::test]
async fn test_get_space_as_team_admin_survives_partial_participant_row() {
    use aws_sdk_dynamodb::types::AttributeValue as AV;

    let ctx = TestContext::setup().await;
    let team_pk = create_team_owned_by_test_user(&ctx).await;
    let (space, space_id) = insert_team_owned_space(&ctx, team_pk).await;

    // Simulate what the legacy `bump_participant_activity` did to admins:
    // a bare UpdateItem against the (missing) SpaceParticipant key, which
    // DDB upserts into a partial row containing only the keys + the SET
    // field. `created_at`, `display_name`, etc. are all absent.
    let (pk, sk) = SpaceParticipant::keys(space.pk.clone(), ctx.test_user.0.pk.clone());
    let now = crate::common::utils::time::get_now_timestamp_millis();
    ctx.ddb
        .update_item()
        .table_name(SpaceParticipant::table_name())
        .key("pk", AV::S(pk.to_string()))
        .key("sk", AV::S(sk.to_string()))
        .update_expression("SET last_activity_at = :now")
        .expression_attribute_values(":now", AV::N(now.to_string()))
        .send()
        .await
        .expect("seed partial participant row");

    // Pre-`is_admin`-skip: this returned 500 because
    // `SpaceParticipant::get(...)?` blew up with
    // `serde_dynamo: missing field 'created_at'` before producing the
    // response. With the admin skip, the partial row is never read.
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(
        status, 200,
        "admin skip should bypass the partial row; body: {body:?}"
    );
    assert_eq!(body["participated"], serde_json::json!(false));
}

#[tokio::test]
async fn test_get_space_as_non_admin_still_loads_participant() {
    let ctx = TestContext::setup().await;
    let team_pk = create_team_owned_by_test_user(&ctx).await;
    let (space, space_id) = insert_team_owned_space(&ctx, team_pk.clone()).await;

    // Add a SECOND user as a Member of the team (NOT admin/owner). They
    // should fall through `permissions.contains(SpaceEdit) == false` and
    // hit the normal participant-loading path. With no SpaceParticipant
    // row, get_space should still succeed and report participated=false,
    // can_participate=true (space is Open & Public).
    let (member, member_headers) = ctx.create_another_user().await;
    let team = Team::get(&ctx.ddb, &team_pk, Some(EntityType::Team))
        .await
        .expect("get team")
        .expect("team exists");
    UserTeam::new(
        member.pk.clone(),
        team_pk.clone(),
        team.display_name.clone(),
        team.profile_url.clone(),
        team.username.clone(),
        team.dao_address.clone(),
        TeamRole::Member,
    )
    .create(&ctx.ddb)
    .await
    .expect("create UserTeam membership");

    let _ = space; // suppress unused-var when only id is needed

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id),
        headers: member_headers.clone(),
    };
    assert_eq!(status, 200, "member should be able to read: {body:?}");
    assert_eq!(body["participated"], serde_json::json!(false));
    assert_eq!(
        body["can_participate"],
        serde_json::json!(true),
        "Open + Public space should expose Join CTA to non-admin members"
    );
}
