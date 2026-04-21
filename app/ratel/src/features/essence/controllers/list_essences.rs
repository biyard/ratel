use crate::common::*;
#[cfg(feature = "server")]
use crate::features::essence::models::Essence;
use crate::features::essence::types::*;
use crate::features::auth::User;

/// Paginated list of the current user's Essence rows. `sort` selects which
/// GSI to query so ordering stays correct across pages; the default (when
/// the client omits the param) is `last_edited` — newest `updated_at` first.
///
/// Accepted `sort` values: `last_edited`, `word_count`, `title`.
#[get("/api/essences?sort&bookmark", user: User)]
pub async fn list_essences_handler(
    sort: Option<String>,
    bookmark: Option<String>,
) -> Result<ListResponse<EssenceResponse>> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let sort = parse_sort(sort.as_deref());

    let (items, next) = Essence::list_for_user(cli, user.pk.clone(), sort, bookmark).await?;
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
