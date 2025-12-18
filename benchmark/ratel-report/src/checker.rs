use anyhow::Result;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
struct StatsRes {
    current: usize,
    peak: usize,
    window_seconds: u64,
    page_key: String,
}

pub async fn run() -> Result<()> {
    let base = std::env::var("PRESENCE_BASE").unwrap_or_else(|_| "http://localhost:3000".into());
    let required_peak: usize = std::env::var("REQUIRED_PEAK").ok().and_then(|v| v.parse().ok()).unwrap_or(2000);
    let duration_secs: u64 = std::env::var("CHECK_DURATION_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(300);

    let client = reqwest::Client::new();

    let _ = client
        .get(format!("{}/presence/stats?reset=true", base))
        .send().await?
        .error_for_status()?;

    let end = tokio::time::Instant::now() + Duration::from_secs(duration_secs);

    let mut observed_peak = 0usize;
    while tokio::time::Instant::now() < end {
        let s: StatsRes = client
            .get(format!("{}/presence/stats", base))
            .send().await?
            .error_for_status()?
            .json().await?;

        observed_peak = observed_peak.max(s.peak);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    println!("observed_peak={} required_peak={} PASS={}",
        observed_peak, required_peak, observed_peak >= required_peak);

    Ok(())
}
