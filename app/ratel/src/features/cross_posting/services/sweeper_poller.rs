//! Local-dev poller for the Stage 3 retry sweep.
//!
//! In production the sweep is driven by a CloudWatch / EventBridge
//! schedule rule that fires once a minute (see
//! `cdk/lib/dynamo-stream-event.ts` § Cross-posting Stage 3). Lambda
//! lights the same `EventBridgeEnvelope::proc` path, lands on
//! `DetailType::SyndicationRetrySweep`, and calls
//! `sweeper::run_retry_sweep`.
//!
//! In `local-dev` there is no schedule rule, so we spawn a tokio task
//! on a dedicated OS thread (the same pattern as `stream_poller`) that
//! sleeps for 60 seconds between calls.
//!
//! Gated behind `#[cfg(all(feature = "server", feature = "local-dev"))]`.

#[cfg(all(feature = "server", feature = "local-dev"))]
pub fn spawn_retry_sweeper() {
    std::thread::Builder::new()
        .name("cross-posting-sweeper".into())
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("retry-sweeper runtime");
            rt.block_on(sweep_loop());
        })
        .expect("failed to spawn cross-posting sweeper thread");
}

#[cfg(all(feature = "server", feature = "local-dev"))]
async fn sweep_loop() {
    use std::time::Duration;

    tracing::info!("local-dev: cross-posting retry sweeper started (1 min interval)");

    // Stagger the first run so it doesn't race with server startup
    // (the table may not be ready on first boot).
    tokio::time::sleep(Duration::from_secs(15)).await;

    loop {
        if let Err(e) =
            crate::features::cross_posting::services::sweeper::run_retry_sweep().await
        {
            tracing::error!(error = %e, "local-dev: retry sweep failed");
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
