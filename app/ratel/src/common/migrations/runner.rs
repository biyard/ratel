use crate::common::models::migration::LastBackfillVersion;

/// Run all pending migrations in version order. Gated by `MIGRATE=true`
/// env var: only executes when set, otherwise a no-op. Safe under
/// concurrent replicas — `LastBackfillVersion::advance_to` issues a
/// conditional update so only one replica wins each version bump.
///
/// Operational rule: set `MIGRATE=true` on **exactly one** instance per
/// release (typically a one-shot Lambda / ECS task). The conditional
/// version bump is a safety net, not the primary contention guard.
pub async fn run_migrations(cli: &aws_sdk_dynamodb::Client) -> crate::common::Result<()> {
    if std::env::var("MIGRATE").as_deref() != Ok("true") {
        tracing::info!("MIGRATE not set — skipping migrations");
        return Ok(());
    }

    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let stored = LastBackfillVersion::get(cli, &pk, Some(&sk))
        .await?
        .map(|r| r.version)
        .unwrap_or(0);

    tracing::info!(stored_version = stored, "migration runner starting");

    if stored < 1 {
        tracing::info!("running migration 001: backfill_character_xp");
        super::m001_backfill_character_xp::run(cli).await?;
        LastBackfillVersion::advance_to(cli, stored, 1).await?;
        tracing::info!("migration 001 complete; version advanced to 1");
    }

    if stored < 2 {
        tracing::info!("running migration 002: backfill_pending_rewards");
        super::m002_backfill_pending_rewards::run(cli).await?;
        LastBackfillVersion::advance_to(cli, 1, 2).await?;
        tracing::info!("migration 002 complete; version advanced to 2");
    }

    // Future migrations stack additively here:
    //   if stored < 3 { ... advance_to(cli, 2, 3) ... }

    tracing::info!("migration runner finished");
    Ok(())
}
