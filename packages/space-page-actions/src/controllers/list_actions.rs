use crate::*;

#[get("/api/actions")]
pub async fn list_actions(
    space_pk: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<SpaceAction>> {
    //TODO: Update Action List
    let cli = crate::config::get().common.dynamodb();

    let opt = SpaceAction::opt_with_bookmark(bookmark);
    let (actions, next_bookmark) = SpaceAction::query(cli, space_pk, opt).await?;
    Ok(ListResponse {
        items: actions,
        bookmark: next_bookmark,
    })
}
