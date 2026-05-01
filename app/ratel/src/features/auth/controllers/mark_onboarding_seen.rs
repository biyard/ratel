use crate::features::auth::*;

/// Mark the cross-posting onboarding interstitial as seen for the current user.
///
/// Called by the OnboardingConnections page on Continue / Skip / connection-success
/// (FR-2 #13). Idempotent — flipping `interstitial_seen` from `true` to `true`
/// is a no-op write but doesn't error. Returns the updated `User` so the
/// caller can refresh `UserContext` without a separate `get_me` round-trip.
#[post("/api/auth/onboarding-seen", user: User)]
pub async fn mark_onboarding_seen_handler() -> Result<User> {
    let conf = crate::features::auth::config::get();
    let cli = conf.dynamodb();

    User::updater(user.pk.clone(), user.sk.clone())
        .with_interstitial_seen(true)
        .execute(cli)
        .await?;

    // The updater above succeeded against `user.pk`, so the row must exist;
    // a `None` re-read here is a should-never-happen DB inconsistency.
    let updated = User::get(cli, user.pk.clone(), Some(EntityType::User))
        .await?
        .ok_or(Error::Internal)?;
    Ok(updated)
}
