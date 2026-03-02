use crate::*;

#[get("/api/spaces/{space_pk}/discussions?category&bookmark", role: SpaceUserRole)]
pub async fn list_discussions(
    space_pk: SpacePartition,
    category: Option<String>,
    bookmark: Option<String>,
) -> Result<ListDiscussionsResponse> {
    SpacePost::can_view(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();

    let (posts, next_bookmark) = if let Some(category_name) = category {
        let opt = SpacePost::opt_with_bookmark(bookmark)
            .sk(EntityType::SpacePost(String::default()).to_string())
            .scan_forward(false)
            .limit(20);
        let pk = format!("{}#{}", space_pk, category_name);
        SpacePost::find_by_category(cli, pk, opt).await?
    } else {
        let opt = SpacePost::opt_with_bookmark(bookmark)
            .sk(EntityType::SpacePost(String::default()).to_string())
            .scan_forward(false)
            .limit(20);
        SpacePost::find_by_space_ordered(cli, space_pk, opt).await?
    };

    let responses: Vec<DiscussionResponse> = posts.into_iter().map(|p| p.into()).collect();

    Ok((responses, next_bookmark).into())
}
