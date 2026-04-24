use crate::common::*;
use crate::common::models::space::SpaceParticipant;
use crate::common::utils::time::get_now_timestamp_millis;

/// Best-effort: failures must not block the user's underlying action.
pub async fn bump_participant_activity(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    user_pk: &Partition,
) {
    let (pk, sk) = SpaceParticipant::keys(space_pk.clone(), user_pk.clone());
    let now = get_now_timestamp_millis();

    if let Err(e) = SpaceParticipant::updater(pk, sk)
        .with_last_activity_at(now)
        .execute(cli)
        .await
    {
        crate::error!(
            "bump_participant_activity failed (space={}, user={}): {e}",
            space_pk,
            user_pk
        );
    }
}
