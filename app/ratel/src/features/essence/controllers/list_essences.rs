use crate::common::*;
#[cfg(feature = "server")]
use crate::features::essence::models::Essence;
use crate::features::auth::User;
use crate::features::essence::types::*;
#[cfg(feature = "server")]
use aws_sdk_dynamodb::types::AttributeValue;
#[cfg(feature = "server")]
use std::collections::HashMap;

/// Paginated list of the current user's Essence rows. `sort` selects which
/// GSI to query so ordering stays correct across pages; the default (when
/// the client omits the param) is `last_edited` — newest `updated_at` first.
///
/// Accepted `sort` values: `last_edited`, `word_count`, `title`.
///
/// When `kind` is present and non-empty, only rows of that kind are
/// returned. Because `Essence` has no GSI keyed by kind, filtering happens
/// in-memory after the GSI scan — the server keeps scanning ahead until it
/// has `limit` matching rows or exhausts the data. Per-kind totals are
/// authoritative via the `UserEssenceStats` counter (see
/// `EssenceStatsResponse.total_{kind}`), so the client can render the
/// correct "Page N of M" without reading every row.
///
/// `limit` defaults to 10, matching the sources-table page size. Capped at
/// 50 so a single request can't scan a large fraction of a user's rows.
#[get("/api/essences?sort&bookmark&kind&limit", user: User)]
pub async fn list_essences_handler(
    sort: Option<String>,
    bookmark: Option<String>,
    kind: Option<String>,
    limit: Option<u32>,
) -> Result<ListResponse<EssenceResponse>> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let sort = parse_sort(sort.as_deref());
    let kind_filter = parse_kind(kind.as_deref());
    let requested_limit = limit.unwrap_or(10).clamp(1, 50) as usize;

    let (items, next) = collect_filtered_page(
        cli,
        user.pk.clone(),
        sort,
        bookmark,
        kind_filter,
        requested_limit,
    )
    .await?;
    let items: Vec<EssenceResponse> = items.into_iter().map(EssenceResponse::from).collect();
    Ok((items, next).into())
}

#[cfg(feature = "server")]
fn parse_sort(raw: Option<&str>) -> EssenceSort {
    match raw {
        Some("word_count") => EssenceSort::WordCountDesc,
        Some("title") => EssenceSort::TitleAsc,
        // Empty string, unknown value, or missing param → default.
        _ => EssenceSort::LastEditedDesc,
    }
}

#[cfg(feature = "server")]
fn parse_kind(raw: Option<&str>) -> KindFilter {
    match raw {
        Some("notion") => KindFilter::Notion,
        Some("post") => KindFilter::Post,
        Some("comment") => KindFilter::Comment,
        Some("poll") => KindFilter::Poll,
        Some("quiz") => KindFilter::Quiz,
        // `all`, empty, unknown, or missing → no filter.
        _ => KindFilter::All,
    }
}

/// Scan pages until `wanted` matching rows accumulate or the GSI is
/// exhausted. Each DynamoDB scan fetches up to 50 rows; capped at 5 scans
/// per call so a single request can't read hundreds of rows even if the
/// kind is rare. `KindFilter::All` short-circuits to a single scan at the
/// requested limit.
///
/// When we hit the page target via filter, we synthesize a bookmark
/// pointing to the LAST RETURNED ROW (not DynamoDB's `LastEvaluatedKey`,
/// which would skip ahead to the end of the underlying scan and lose any
/// matches between the page target and the scan boundary). This guarantees
/// next-page semantics even when the user's full row set fits inside a
/// single scan batch (in which case the underlying LEK is `None`).
#[cfg(feature = "server")]
async fn collect_filtered_page(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: Partition,
    sort: EssenceSort,
    bookmark: Option<String>,
    kind: KindFilter,
    wanted: usize,
) -> Result<(Vec<Essence>, Option<String>)> {
    // Even `KindFilter::All` must go through the batching loop: the GSI
    // query uses `filter_sk_prefix("ESSENCE#")`, which DynamoDB applies
    // AFTER `Limit`, so a single scan at `wanted` rows returns fewer than
    // requested whenever other entity types share the user's gsi_pk.
    const SCAN_BATCH: i32 = 50;
    const MAX_SCANS: u8 = 5;

    let mut out = Vec::with_capacity(wanted);
    let mut cursor = bookmark;
    let mut scans: u8 = 0;

    loop {
        let (batch, next) =
            Essence::list_for_user(cli, user_pk.clone(), sort, cursor.clone(), SCAN_BATCH).await?;
        for row in batch {
            if kind.matches(row.source_kind) {
                out.push(row);
                if out.len() >= wanted {
                    let bm = build_essence_bookmark(out.last().unwrap(), sort)?;
                    return Ok((out, Some(bm)));
                }
            }
        }
        cursor = next;
        scans += 1;
        if cursor.is_none() || scans >= MAX_SCANS {
            break;
        }
    }
    // Either the underlying scan is exhausted (cursor=None — no more pages)
    // or we hit MAX_SCANS without filling the page. In either case we hand
    // back what we have plus whatever cursor the underlying scan gave us.
    Ok((out, cursor))
}

/// Synthesize a `LastEvaluatedKey` (encoded via `Essence::encode_lek_all`)
/// pointing at the given essence row, so the client can request the page
/// AFTER it. The shape of the LEK depends on which GSI we're paginating —
/// each GSI has a different `(pk, sk)` pair on top of the primary table key.
///
/// Numeric GSI sort keys are zero-padded to match the `numeric_lex_encoding`
/// used by `DynamoEntity`'s derive (i64 → 20-digit shifted by `i64::MIN`),
/// so the bookmark string matches what DynamoDB stores in the index.
#[cfg(feature = "server")]
fn build_essence_bookmark(e: &Essence, sort: EssenceSort) -> Result<String> {
    let pk_str = e.pk.to_string();
    let sk_str = e.sk.to_string();
    let mut lek: HashMap<String, AttributeValue> = HashMap::new();
    lek.insert("pk".to_string(), AttributeValue::S(pk_str.clone()));
    lek.insert("sk".to_string(), AttributeValue::S(sk_str));

    match sort {
        EssenceSort::LastEditedDesc => {
            // gsi1: pk = primary pk, sk = updated_at (i64) padded as 20-digit
            // shifted-by-i64::MIN string. Matches the GSI projection emitted
            // by DynamoEntity for `find_by_user_recent`.
            let shifted = (e.updated_at as i128) - (i64::MIN as i128);
            let sk_padded = format!("{:020}", shifted);
            lek.insert("gsi1_pk".to_string(), AttributeValue::S(pk_str));
            lek.insert("gsi1_sk".to_string(), AttributeValue::S(sk_padded));
        }
        EssenceSort::WordCountDesc => {
            // gsi2: pk = primary pk, sk = word_count (i64) padded same way.
            let shifted = (e.word_count as i128) - (i64::MIN as i128);
            let sk_padded = format!("{:020}", shifted);
            lek.insert("gsi2_pk".to_string(), AttributeValue::S(pk_str));
            lek.insert("gsi2_sk".to_string(), AttributeValue::S(sk_padded));
        }
        EssenceSort::TitleAsc => {
            // gsi3: pk = primary pk, sk = raw title.
            lek.insert("gsi3_pk".to_string(), AttributeValue::S(pk_str));
            lek.insert("gsi3_sk".to_string(), AttributeValue::S(e.title.clone()));
        }
    }

    Essence::encode_lek_all(&lek).map_err(|err| {
        crate::error!("essence bookmark synth failed: {err}");
        EssenceError::ReadFailed.into()
    })
}
