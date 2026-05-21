use super::*;
use crate::common::models::space::SpaceParticipant;
use crate::common::types::{EntityType, Partition};
use crate::features::spaces::space_common::services::bump_participant_activity;

/// Regression test for the sub-team broadcast "Something went wrong" bug.
///
/// Before the fix `bump_participant_activity` issued a bare
/// `SpaceParticipant::updater(pk, sk).with_last_activity_at(now).execute(cli)`
/// against the DDB row. When the row did not exist (always the case for
/// a team-owned space whose Creator is the parent-team admin — they're a
/// Creator role but never a participant), DynamoDB's `UpdateItem`
/// happily created a brand-new item containing ONLY the keys plus
/// `last_activity_at`, missing every required field of
/// `SpaceParticipant` (`created_at`, `display_name`, ...). The next
/// `SpaceParticipant::get` issued from `get_space` then panicked with
/// `serde_dynamo: missing field 'created_at'`, and the entire space
/// arena rendered "Something went wrong" on the next page load.
///
/// The fix makes the helper a no-op when no participant row exists by
/// conditioning the update on `attribute_exists(created_at)`. This test
/// covers both shapes:
///   1. No participant row → bump is a silent no-op, no partial row is
///      written.
///   2. Real participant row → bump updates `last_activity_at` without
///      clobbering other fields.
#[tokio::test]
async fn test_bump_participant_activity_no_partial_row_when_user_is_not_participant() {
    let ctx = TestContext::setup().await;

    // Pick arbitrary keys. We don't even need a real SpaceCommon row —
    // the helper only writes against SpaceParticipant.
    let space_pk = Partition::Space(uuid::Uuid::new_v4().to_string());
    let user_pk = ctx.test_user.0.pk.clone();

    // Sanity: row doesn't exist before the bump.
    let (lookup_pk, lookup_sk) = SpaceParticipant::keys(space_pk.clone(), user_pk.clone());
    let pre = SpaceParticipant::get(&ctx.ddb, &lookup_pk, Some(&lookup_sk))
        .await
        .expect("pre-bump get must not error");
    assert!(pre.is_none(), "precondition: no SpaceParticipant row");

    // Bump must not panic, must not propagate an error, and must NOT
    // create a partial row.
    bump_participant_activity(&ctx.ddb, &space_pk, &user_pk).await;

    let post = SpaceParticipant::get(&ctx.ddb, &lookup_pk, Some(&lookup_sk))
        .await
        .expect(
            "post-bump get must succeed — if this fails with `missing field`, \
             the helper wrote a partial row again",
        );
    assert!(
        post.is_none(),
        "no SpaceParticipant row should exist after bumping a non-participant — \
         got {post:?}"
    );
}

#[tokio::test]
async fn test_bump_participant_activity_updates_last_activity_when_participant_exists() {
    let ctx = TestContext::setup().await;

    let space_pk = Partition::Space(uuid::Uuid::new_v4().to_string());
    let user = ctx.test_user.0.clone();
    let user_pk = user.pk.clone();

    // Seed a real, complete participant row.
    let participant = SpaceParticipant::new_non_anonymous(space_pk.clone(), user);
    participant
        .create(&ctx.ddb)
        .await
        .expect("seed participant");

    let (lookup_pk, lookup_sk) = SpaceParticipant::keys(space_pk.clone(), user_pk.clone());
    let before = SpaceParticipant::get(&ctx.ddb, &lookup_pk, Some(&lookup_sk))
        .await
        .expect("get before")
        .expect("seeded row");
    assert!(
        before.last_activity_at.is_none(),
        "seeded row should have no last_activity_at yet"
    );

    bump_participant_activity(&ctx.ddb, &space_pk, &user_pk).await;

    let after = SpaceParticipant::get(&ctx.ddb, &lookup_pk, Some(&lookup_sk))
        .await
        .expect("get after")
        .expect("row should still exist");
    assert!(
        after.last_activity_at.is_some(),
        "bump must populate last_activity_at on an existing participant"
    );
    // Other required fields must remain intact — proves the update did
    // not clobber the row.
    assert_eq!(after.created_at, before.created_at);
    assert_eq!(after.username, before.username);
    assert_eq!(after.display_name, before.display_name);
    assert_eq!(after.sk, EntityType::SpaceParticipant);
}
