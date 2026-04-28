//! Local test driver for the discussion-analysis (Phase C) Lambda
//! handler. Bypasses EventBridge — calls
//! `process_discussion_analysis(cli, &row)` directly.
//!
//! Usage:
//!   cargo run --bin test_analyze_discussion --features server -- \
//!     <space_uuid> <report_id> <discussion_id_and_request_uuid>
//!
//! The `discussion_id_and_request_uuid` is the second tuple field of
//! `EntityType::SpaceAnalyzeDiscussionResult(report_id, "{discussion_id}#{request_uuid}")`.
//! In the DDB console it's everything after `SpaceAnalyzeDiscussionResult#{report_id}#`.
//! Easiest way to get one: trigger an analyze run from the UI (which
//! INSERTs the row), copy the row's sk, then re-run it locally with
//! this binary to iterate on the analysis logic.

#[cfg(not(feature = "server"))]
fn main() {
    eprintln!("test_analyze_discussion requires --features server");
    std::process::exit(1);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use app_shell::common::CommonConfig;
    use app_shell::common::types::{EntityType, Partition};
    use app_shell::features::spaces::pages::apps::apps::analyzes::SpaceAnalyzeDiscussionResult;
    use app_shell::features::spaces::pages::apps::apps::analyzes::services::discussion_analysis;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,app_shell=debug")),
        )
        .init();

    let usage = "usage: test_analyze_discussion <space_uuid> <report_id> <discussion_id_and_request_uuid>";
    let mut args = std::env::args().skip(1);
    let space_uuid = args.next().ok_or(usage)?;
    let report_id = args.next().ok_or(usage)?;
    let discussion_request = args.next().ok_or(usage)?;

    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let space_pk = Partition::Space(space_uuid.clone());
    let sk = EntityType::SpaceAnalyzeDiscussionResult(report_id.clone(), discussion_request.clone());

    println!("Loading discussion-analysis row space={space_uuid} report={report_id} disc#req={discussion_request} …");
    let row = SpaceAnalyzeDiscussionResult::get(cli, &space_pk, Some(sk))
        .await?
        .ok_or("discussion-analysis row not found")?;
    println!(
        "  loaded: status={:?} report_id={} discussion_id={}",
        row.status, row.report_id, row.discussion_id
    );

    println!("Running discussion_analysis::process_discussion_analysis …");
    discussion_analysis::process_discussion_analysis(cli, &row).await?;
    println!("Done. Check the row's status / topics / tfidf_terms / network_*.");

    Ok(())
}
