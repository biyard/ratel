use crate::common::models::auth::User;
use crate::common::models::space::SpaceCommon;
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::models::SpaceApp;
use crate::features::spaces::pages::apps::types::SpaceAppError;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct CreateAnalyzeReportRequest {
    pub name: String,
    pub filters: Vec<AnalyzeReportFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct CreateAnalyzeReportResponse {
    pub report_id: String,
}

/// Persist a new analyze report with `status = InProgress`. The actual
/// LDA / TF-IDF / poll-quiz aggregation pipeline (next stage) reads the
/// stored `filters` off this row, writes its result fields back onto
/// the same row, and flips `status` to `Finish`. The response carries
/// just the new id so the client can navigate to the detail view (or
/// stay on the list and watch the badge update).
///
/// Quota gate: non-Enterprise spaces are capped at
/// `AnalyzeQuotaConfig::non_enterprise_limit` reports per space (default
/// 2). Enterprise tier is unlimited. The Enterprise check attaches to
/// the **space owner** (the user or team that created the space), not
/// the calling user — otherwise a non-paying team member acting on a
/// team's Enterprise space would be wrongly capped, and an Enterprise
/// user would bypass a non-Enterprise team's quota. Limit is
/// admin-tunable through `PUT /api/admin/analyze-quota`.
#[post("/api/spaces/{space_id}/apps/analyzes/reports", user: User, role: SpaceUserRole, space: SpaceCommon)]
pub async fn create_analyze_report(
    space_id: SpacePartition,
    req: CreateAnalyzeReportRequest,
) -> Result<CreateAnalyzeReportResponse> {
    SpaceApp::can_edit(role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();

    let trimmed_name = req.name.trim();
    if trimmed_name.is_empty() {
        return Err(Error::InvalidFormat);
    }

    if !is_enterprise_owner(cli, &space.user_pk).await? {
        let limit = AnalyzeQuotaConfig::get_limit(cli).await.map_err(|e| {
            crate::error!("create_analyze_report: failed to read quota config: {e}");
            Error::Internal
        })?;
        let used = count_reports_in_space(cli, &space_pk, limit).await?;
        if used >= limit {
            return Err(SpaceAppError::AnalyzeQuotaExceeded.into());
        }
    }

    let report = SpaceAnalyzeReport::new(space_id, trimmed_name.to_string(), req.filters);
    let report_id = match &report.sk {
        EntityType::SpaceAnalyzeReport(id) => id.clone(),
        _ => String::new(),
    };

    report.create(cli).await.map_err(|e| {
        crate::error!("failed to create analyze report: {e}");
        Error::Internal
    })?;

    Ok(CreateAnalyzeReportResponse { report_id })
}

/// Treats the space owner as Enterprise when the matching membership
/// row's `membership_pk` stringifies to anything containing
/// `Enterprise`. Same cheap string check `UserMembershipResponse::is_paid`
/// uses for the "is paying" predicate — avoids loading the parent
/// `Membership` entity just to read the tier.
///
/// Routes the lookup by `Partition` variant: `User(_)` reads
/// `UserMembership`, `Team(_)` reads `TeamMembership`. Anything else
/// (defensive — `SpaceCommon.user_pk` should only ever be one of those
/// two) returns `false` so the quota still applies.
#[cfg(feature = "server")]
async fn is_enterprise_owner(
    cli: &aws_sdk_dynamodb::Client,
    owner_pk: &Partition,
) -> Result<bool> {
    use crate::features::membership::models::{TeamMembership, UserMembership};
    match owner_pk {
        Partition::User(_) => {
            let row = UserMembership::get(cli, owner_pk, Some(EntityType::UserMembership)).await?;
            Ok(row
                .map(|m| m.membership_pk.0.contains("Enterprise"))
                .unwrap_or(false))
        }
        Partition::Team(_) => {
            let row = TeamMembership::get(cli, owner_pk, Some(EntityType::TeamMembership)).await?;
            Ok(row
                .map(|m| m.membership_pk.0.contains("Enterprise"))
                .unwrap_or(false))
        }
        _ => Ok(false),
    }
}

/// Walk the space's analyze-report rows and count, stopping as soon as
/// we've seen `limit + 1` (we only need to know whether the user is at
/// or over the quota — exact counts above that don't matter).
#[cfg(feature = "server")]
async fn count_reports_in_space(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    limit: i64,
) -> Result<i64> {
    let prefix = EntityType::SpaceAnalyzeReport(String::default()).to_string();
    let mut bookmark: Option<String> = None;
    let mut count: i64 = 0;
    loop {
        let mut opt = SpaceAnalyzeReport::opt().sk(prefix.clone()).limit(50);
        if let Some(b) = bookmark.clone() {
            opt = opt.bookmark(b);
        }
        let (rows, next) = SpaceAnalyzeReport::query(cli, space_pk.clone(), opt).await?;
        count += rows.len() as i64;
        if count > limit {
            return Ok(count);
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => return Ok(count),
        }
    }
}
