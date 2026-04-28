use crate::features::spaces::pages::apps::apps::analyzes::*;

/// One saved cross-filter analysis on a space.
///
/// Status starts at `InProgress` on submit. A DynamoDB stream pipeline
/// (added in a follow-up stage) reads the stored `filters`, runs the
/// LDA / TF-IDF / poll-quiz aggregation, writes the result fields
/// (`lda_topics`, `tf_idf`, `network`, …) onto this same row, and flips
/// `status` to `Finish`. Until then the row carries only the inputs the
/// stream needs.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, schemars::JsonSchema, aide::OperationIo)
)]
pub struct SpaceAnalyzeReport {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub status: AnalyzeReportStatus,

    /// The cross-filter chips chosen at submit time. Stored verbatim so
    /// the stream worker and the detail page can re-derive the matching
    /// respondent set without going back to the picker.
    #[serde(default)]
    pub filters: Vec<AnalyzeReportFilter>,

    /// Cached count of distinct respondents that match every chip in
    /// `filters` (AND across sources). Set by the stream worker when it
    /// finishes; `0` while `status == InProgress`.
    #[serde(default)]
    pub respondent_count: i64,
}

#[cfg(feature = "server")]
impl SpaceAnalyzeReport {
    pub fn new(
        space_pk: SpacePartition,
        name: String,
        filters: Vec<AnalyzeReportFilter>,
    ) -> Self {
        use crate::common::utils::time::get_now_timestamp_millis;

        let now = get_now_timestamp_millis();
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceAnalyzeReport(uuid::Uuid::now_v7().to_string());

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            name,
            status: AnalyzeReportStatus::InProgress,
            filters,
            respondent_count: 0,
        }
    }

    pub fn can_view(_role: SpaceUserRole) -> Result<()> {
        Ok(())
    }

    pub fn can_edit(role: SpaceUserRole) -> Result<()> {
        if role != SpaceUserRole::Creator {
            return Err(Error::NoPermission);
        }
        Ok(())
    }
}
