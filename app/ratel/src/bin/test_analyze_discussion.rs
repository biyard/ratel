//! Local test driver for the discussion-analysis (Phase C) Lambda
//! handler. Bypasses EventBridge — calls
//! `process_discussion_analysis(cli, &row)` directly.
//!
//! Usage:
//!   cargo run --bin test_analyze_discussion --features server -- <space_uuid> <result_sk>
//!
//! `<result_sk>` is the full `SpaceAnalyzeDiscussionResult` sort key.
//! Easiest way to get one: trigger an analyze run from the UI (which
//! INSERTs the row), look it up in the DDB console, then re-run it
//! locally with this binary to iterate on the analysis logic.

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
        .ok_or("usage: test_analyze_discussion <space_uuid> <result_sk>")?;
    let result_sk = args
        .next()
        .ok_or("usage: test_analyze_discussion <space_uuid> <result_sk>")?;

    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();

    let space_pk = Partition::Space(space_uuid.clone());
    // `result_sk` is the canonical EntityType variant value (everything
    // after the prefix). `from_str_with_prefix`-style helpers vary per
    // codebase; the simplest path is constructing the variant manually
    // since this binary only handles one entity type.
    let stripped = result_sk
        .strip_prefix("SpaceAnalyzeDiscussionResult#")
        .unwrap_or(&result_sk)
        .to_string();
    let sk = EntityType::SpaceAnalyzeDiscussionResult(stripped);

    println!("Loading discussion-analysis row space={space_uuid} sk={result_sk} …");
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
