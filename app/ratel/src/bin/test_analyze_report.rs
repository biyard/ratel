//! Local test driver for the auto-analysis (Phase B) Lambda handler.
//! Bypasses EventBridge entirely — calls
//! `process_analyze_report(cli, &report)` directly against whichever
//! DynamoDB table the env points at (set DYNAMO_TABLE_PREFIX +
//! AWS_PROFILE/AWS_REGION before running).
//!
//! Usage:
//!   cargo run --bin test_analyze_report --features server -- <space_uuid> <report_id>
//!
//! Effect: idempotently re-runs the Phase-B aggregation for the given
//! report and overwrites its `SpaceAnalyzeReportResult` row + flips
//! the report's status to Finish.

#[cfg(not(feature = "server"))]
fn main() {
    eprintln!("test_analyze_report requires --features server");
    std::process::exit(1);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use app_shell::common::CommonConfig;
    use app_shell::common::types::{EntityType, Partition};
    use app_shell::features::spaces::pages::apps::apps::analyzes::SpaceAnalyzeReport;
    use app_shell::features::spaces::pages::apps::apps::analyzes::services::auto_analysis;
    use bdk::prelude::DynamoEntity;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,app_shell=debug")),
        )
        .init();

    let mut args = std::env::args().skip(1);
    let space_uuid = args
        .next()
        .ok_or("usage: test_analyze_report <space_uuid> <report_id>")?;
    let report_id = args
        .next()
        .ok_or("usage: test_analyze_report <space_uuid> <report_id>")?;

    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let space_pk = Partition::Space(space_uuid.clone());
    let sk = EntityType::SpaceAnalyzeReport(report_id.clone());

    println!("Loading report space={space_uuid} report={report_id} …");
    let report = SpaceAnalyzeReport::get(cli, &space_pk, Some(sk))
        .await?
        .ok_or("report not found at the given (space, report_id)")?;
    println!(
        "  loaded report: status={:?} filters={}",
        report.status,
        report.filters.len()
    );

    println!("Running auto_analysis::process_analyze_report …");
    auto_analysis::process_analyze_report(cli, &report).await?;
    println!("Done. Check the SpaceAnalyzeReportResult row + report.status.");

    Ok(())
}
