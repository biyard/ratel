//! Drains every `SpaceAnalyzeDiscussionResult` row whose
//! `status == InProgress` and runs the discussion-analysis pipeline
//! against it. Same effect as the EventBridge → Lambda path, but
//! invocable locally without deploying CDK.
//!
//! Usage:
//!   cargo run --bin run_pending_discussions --features server -- <space_uuid>
//!     [--report <report_id>]
//!
//! With no `--report` arg: scans all discussion-analysis rows in the
//! space and processes every one with `status=InProgress`.
//! With `--report <id>`: only rows under that report.

#[cfg(not(feature = "server"))]
fn main() {
    eprintln!("run_pending_discussions requires --features server");
    std::process::exit(1);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use app_shell::common::CommonConfig;
    use app_shell::common::types::Partition;
    use app_shell::features::spaces::pages::apps::apps::analyzes::{
        AnalyzeReportStatus, SpaceAnalyzeDiscussionResult,
    };
    use app_shell::features::spaces::pages::apps::apps::analyzes::services::discussion_analysis;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,app_shell=debug")),
        )
        .init();

    let mut args = std::env::args().skip(1);
    let space_uuid = args
        .next()
        .ok_or("usage: run_pending_discussions <space_uuid> [--report <report_id>]")?;

    // Optional `--report <id>` filter.
    let mut report_filter: Option<String> = None;
    while let Some(flag) = args.next() {
        if flag == "--report" {
            report_filter = args.next();
        }
    }

    let cfg = CommonConfig::default();
    let cli = cfg.dynamodb();
    let space_pk = Partition::Space(space_uuid.clone());

    // Begin-with prefix on the discussion-result sk. `query` is fine
    // here because all discussion-analysis rows for a space share the
    // same pk. NOTE: the on-disk prefix is the SCREAMING_SNAKE form
    // emitted by the DynamoEnum derive — using the variant name
    // (`SpaceAnalyzeDiscussionResult`) verbatim silently misses every
    // row.
    let sk_prefix = match report_filter.as_deref() {
        Some(rid) => format!("SPACE_ANALYZE_DISCUSSION_RESULT#{}#", rid),
        None => "SPACE_ANALYZE_DISCUSSION_RESULT#".to_string(),
    };
    let opt = SpaceAnalyzeDiscussionResult::opt().sk(sk_prefix).limit(100);
    let (rows, _) = SpaceAnalyzeDiscussionResult::query(cli, space_pk.clone(), opt).await?;

    let pending: Vec<_> = rows
        .into_iter()
        .filter(|r| matches!(r.status, AnalyzeReportStatus::InProgress))
        .collect();

    if pending.is_empty() {
        println!("No InProgress discussion-analysis rows under space={space_uuid}");
        return Ok(());
    }

    println!("Found {} InProgress row(s). Processing…", pending.len());
    for (i, row) in pending.iter().enumerate() {
        println!(
            "[{}/{}] report={} discussion_id={} sk={}",
            i + 1,
            pending.len(),
            row.report_id,
            row.discussion_id,
            row.sk
        );
        match discussion_analysis::process_discussion_analysis(cli, row).await {
            Ok(()) => println!("  ✓ done"),
            Err(e) => println!("  ✗ failed: {e}"),
        }
    }

    Ok(())
}
