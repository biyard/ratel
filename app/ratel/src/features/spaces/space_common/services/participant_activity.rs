use crate::common::*;
use crate::common::models::space::SpaceParticipant;
use crate::common::utils::time::get_now_timestamp_millis;

/// Best-effort: failures must not block the user's underlying action.
///
/// Conditional on `attribute_exists(created_at)` so we never write a
/// partial SpaceParticipant row. The naive
/// `SpaceParticipant::updater(pk, sk).with_last_activity_at(now).execute(cli)`
/// path used to fail closed: DynamoDB's `UpdateItem` creates a brand-new
/// item containing ONLY the keys plus `last_activity_at` when the row
/// didn't exist, which then made every subsequent
/// `SpaceParticipant::get` (e.g. inside `get_space`) blow up with
/// `serde_dynamo: missing field 'created_at'`. The most common trigger
/// was a team-owned space's parent-team admin (Creator role, never a
/// Participant) running `create_discussion` — they have no
/// SpaceParticipant row to bump, so the partial row would get created
/// here and the next page load would render "Something went wrong".
///
/// Same condition also covers the "row exists but is partial because of
/// the old bug" case in already-deployed environments: those rows lack
/// `created_at`, so the condition fails and we skip them too.
pub async fn bump_participant_activity(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    user_pk: &Partition,
) {
    use aws_sdk_dynamodb::operation::update_item::UpdateItemError;
    use aws_sdk_dynamodb::types::AttributeValue as AV;

    let (pk, sk) = SpaceParticipant::keys(space_pk.clone(), user_pk.clone());
    let now = get_now_timestamp_millis();

    let resp = cli
        .update_item()
        .table_name(SpaceParticipant::table_name())
        .key("pk", AV::S(pk.to_string()))
        .key("sk", AV::S(sk.to_string()))
        .update_expression("SET last_activity_at = :now")
        .condition_expression("attribute_exists(created_at)")
        .expression_attribute_values(":now", AV::N(now.to_string()))
        .send()
        .await;

    if let Err(e) = resp {
        let svc = e.into_service_error();
        // ConditionalCheckFailedException → row missing or partial; not a
        // participant we care about. Silent skip is intentional.
        if matches!(svc, UpdateItemError::ConditionalCheckFailedException(_)) {
            return;
        }
        crate::error!(
            "bump_participant_activity failed (space={}, user={}): {svc}",
            space_pk,
            user_pk
        );
    }
}
