use crate::features::spaces::pages::actions::actions::discussion::*;

#[get("/api/spaces/{space_id}/discussions?category&bookmark", role: SpaceUserRole)]
pub async fn list_discussions(
    space_id: SpacePartition,
    category: Option<String>,
    bookmark: Option<String>,
) -> Result<ListResponse<SpacePost>> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let (posts, next_bookmark) = if let Some(category_name) = category {
        let opt = SpacePost::opt_with_bookmark(bookmark)
            .scan_index_forward(false)
            .limit(20);
        let pk = format!("{}#{}", space_pk, category_name);
        SpacePost::find_by_category(cli, pk, opt).await?
    } else {
        let opt = SpacePost::opt_with_bookmark(bookmark)
            .scan_index_forward(false)
            .limit(20);
        SpacePost::find_by_space_ordered(cli, space_pk, opt).await?
    };

    Ok(ListResponse {
        bookmark: next_bookmark,
        items: posts,
    })
}
