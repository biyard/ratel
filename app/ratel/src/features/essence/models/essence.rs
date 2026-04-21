use crate::features::essence::types::*;
#[cfg(feature = "server")]
use crate::features::essence::models::UserEssenceStats;
use crate::*;

/// Row stored under `USER#{user_id}` for every source the user has
/// contributed to their Essence House.
///
/// Indexes (all sharing `pk = USER#{uid}` — each GSI just re-sorts the
/// user's rows on a different dimension, so listing can paginate
/// correctly regardless of the chosen sort column):
/// - **Primary** — `sk = ESSENCE#{encoded}`. Deterministic sk from the
///   source's pk+sk, so writes are idempotent (upsert == put).
/// - **GSI1 (`find_by_user_recent`)** — `sk = updated_at` for "last
///   edited" pagination (DynamoEntity encodes the i64 lexicographically).
/// - **GSI2 (`find_by_user_by_words`)** — `sk = word_count` for
///   "word count desc" pagination.
/// - **GSI3 (`find_by_user_by_title`)** — `sk = title` for "title A–Z"
///   pagination. Raw string sort, same convention as `Team.display_name`.
#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Essence {
    /// Primary pk AND every GSI's pk — the same `USER#{user_id}` serves all
    /// indexes, they just differ in their sort key.
    #[dynamo(name = "find_by_user_recent", index = "gsi1", pk)]
    #[dynamo(name = "find_by_user_by_words", index = "gsi2", pk)]
    #[dynamo(name = "find_by_user_by_title", index = "gsi3", pk)]
    pub pk: Partition,

    /// `ESSENCE#{deterministic_id}` where `deterministic_id` encodes the
    /// referenced source's pk+sk.
    pub sk: EntityType,

    pub source_kind: EssenceSourceKind,

    #[dynamo(index = "gsi3", sk)]
    pub title: String,

    pub source_path: String,

    /// Raw pk/sk strings of the referenced entity — stored as `String` (not
    /// `Partition`/`EntityType`) so we can reference arbitrary source
    /// systems (Notion, future integrations) without forcing every foreign
    /// id into the Partition enum.
    pub source_pk: String,
    pub source_sk: String,

    /// Space this essence belongs to, if any. Stored as a raw partition
    /// string (`SPACE#{uuid}`) so the client can navigate to the space for
    /// poll/quiz/discussion-comment rows whose `source_pk` doesn't contain
    /// the space id directly (e.g. a DiscussionComment's source_pk is the
    /// parent SpacePost, not the space).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_pk: Option<String>,

    /// `i64` (not `u32`) so DynamoEntity's numeric lexicographic encoder
    /// accepts it as the GSI2 sort key — same pattern as
    /// `common/models/auth/user.rs` using `updated_at: i64` as a gsi sk.
    #[dynamo(index = "gsi2", sk)]
    #[serde(default)]
    pub word_count: i64,

    pub created_at: i64,

    #[dynamo(index = "gsi1", sk)]
    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl Essence {
    /// Build a deterministic Essence id from the referenced source's pk+sk.
    /// We replace `#` with `_` (the delimiter DynamoEntity itself uses) so
    /// the encoded id has a single top-level `~` separator and survives a
    /// round-trip through key parsing.
    fn encode_source_id(source_pk: &str, source_sk: &str) -> String {
        let pk = source_pk.replace('#', "_");
        let sk = source_sk.replace('#', "_");
        format!("{pk}~{sk}")
    }

    pub fn essence_sk_for(source_pk: &Partition, source_sk: &EntityType) -> EntityType {
        let id = Self::encode_source_id(&source_pk.to_string(), &source_sk.to_string());
        EntityType::Essence(id)
    }

    /// Construct (without persisting) an Essence row for the given source.
    pub fn new(
        user_pk: Partition,
        source_pk: Partition,
        source_sk: EntityType,
        source_kind: EssenceSourceKind,
        title: String,
        source_path: String,
        word_count: i64,
        space_pk: Option<Partition>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        let source_pk_str = source_pk.to_string();
        let source_sk_str = source_sk.to_string();

        Self {
            pk: user_pk,
            sk: Self::essence_sk_for(&source_pk, &source_sk),
            source_kind,
            title,
            source_path,
            source_pk: source_pk_str,
            source_sk: source_sk_str,
            space_pk: space_pk.map(|p| p.to_string()),
            word_count,
            created_at: now,
            updated_at: now,
        }
    }

    /// Idempotent write — deterministic sk means calling this twice for the
    /// same source overwrites the existing row rather than duplicating it.
    /// Preserves `created_at` from the existing row if one is present.
    /// Atomically updates the per-user counter row so hero stats stay in
    /// sync with the table.
    ///
    /// Named `put` rather than `upsert` because DynamoEntity's derive
    /// already generates an `upsert` method on every entity.
    pub async fn put(&self, cli: &aws_sdk_dynamodb::Client) -> Result<()> {
        let mut row = self.clone();
        let existing = Self::get(cli, row.pk.clone(), Some(row.sk.clone()))
            .await
            .ok()
            .flatten();
        let (source_delta, words_delta) = match &existing {
            Some(old) => {
                row.created_at = old.created_at;
                (0i64, row.word_count - old.word_count)
            }
            None => (1i64, row.word_count),
        };
        row.upsert(cli).await.map_err(|e| {
            crate::error!("essence upsert failed: {e}");
            EssenceError::UpsertFailed
        })?;
        Self::bump_stats(cli, &row.pk, source_delta, words_delta).await;
        Ok(())
    }

    /// Atomic counter update via DynamoDB `ADD` — creates the stats row if
    /// it doesn't exist, else increments in place. Intentionally swallows
    /// failures (logged only) so an Essence write never fails because the
    /// counter is behind; counters can be re-computed offline if they
    /// drift.
    async fn bump_stats(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
        source_delta: i64,
        words_delta: i64,
    ) {
        if source_delta == 0 && words_delta == 0 {
            return;
        }
        let res = UserEssenceStats::updater(user_pk, EntityType::UserEssenceStats)
            .increase_total_sources(source_delta)
            .increase_total_words(words_delta)
            .execute(cli)
            .await;
        if let Err(e) = res {
            crate::error!("essence stats bump failed: {e}");
        }
    }

    /// Convenience wrapper: construct + upsert in one call so call sites in
    /// Post/Poll/Quiz/Comment handlers stay a single line.
    pub async fn upsert_for_source(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: Partition,
        source_pk: Partition,
        source_sk: EntityType,
        source_kind: EssenceSourceKind,
        title: String,
        source_path: String,
        word_count: i64,
        space_pk: Option<Partition>,
    ) -> Result<()> {
        let row = Self::new(
            user_pk,
            source_pk,
            source_sk,
            source_kind,
            title,
            source_path,
            word_count,
            space_pk,
        );
        row.put(cli).await
    }

    /// Remove the Essence row mirroring the given source, if any. Silently
    /// succeeds when no row exists — useful for cascade deletes where the
    /// source may never have been indexed (e.g. legacy pre-migration rows).
    /// Decrements the per-user counter row by the removed row's `chunks`
    /// and one source.
    pub async fn delete_for_source(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: Partition,
        source_pk: &Partition,
        source_sk: &EntityType,
    ) -> Result<()> {
        let sk = Self::essence_sk_for(source_pk, source_sk);
        Self::detach_by_sk(cli, user_pk, sk).await
    }

    /// Shared delete path for both cascade (delete_for_source) and the
    /// user-facing `DELETE /api/essences/:id` endpoint. Looks up the row
    /// to capture its `chunks` value before deleting so the counter
    /// decrement is accurate.
    pub async fn detach_by_sk(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: Partition,
        sk: EntityType,
    ) -> Result<()> {
        let row = Self::get(cli, user_pk.clone(), Some(sk.clone()))
            .await
            .ok()
            .flatten();
        let Some(row) = row else {
            return Ok(());
        };
        let words = row.word_count;
        Self::delete(cli, user_pk.clone(), Some(sk))
            .await
            .map_err(|e| {
                crate::error!("essence cascade delete failed: {e}");
                EssenceError::DeleteFailed
            })?;
        Self::bump_stats(cli, &user_pk, -1, -words).await;
        Ok(())
    }

    /// List Essence rows for a user, paginated through the GSI matching
    /// `sort`. DynamoDB returns rows already in sorted order — no
    /// in-memory re-sort, so pagination is correct at any data size.
    pub async fn list_for_user(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: Partition,
        sort: EssenceSort,
        bookmark: Option<String>,
    ) -> Result<(Vec<Self>, Option<String>)> {
        let opts = Self::opt_with_bookmark(bookmark).limit(50);
        let result = match sort {
            EssenceSort::LastEditedDesc => {
                Self::find_by_user_recent(cli, user_pk, opts.scan_index_forward(false)).await
            }
            EssenceSort::WordCountDesc => {
                Self::find_by_user_by_words(cli, user_pk, opts.scan_index_forward(false)).await
            }
            EssenceSort::TitleAsc => {
                Self::find_by_user_by_title(cli, user_pk, opts.scan_index_forward(true)).await
            }
        };
        result.map_err(|e| {
            crate::error!("essence list failed: {e}");
            EssenceError::ReadFailed.into()
        })
    }
}

impl From<Essence> for EssenceResponse {
    fn from(e: Essence) -> Self {
        let id = match &e.sk {
            EntityType::Essence(id) => id.clone(),
            other => other.to_string(),
        };
        Self {
            id,
            source_kind: e.source_kind,
            title: e.title,
            source_path: e.source_path,
            source_pk: e.source_pk,
            source_sk: e.source_sk,
            space_pk: e.space_pk,
            word_count: e.word_count,
            updated_at: e.updated_at,
        }
    }
}
