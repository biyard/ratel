use super::*;

#[mcp_tool(name = "list_chapters", description = "List all chapters in a space, sorted by order.")]
#[get("/api/spaces/{space_id}/chapters", role: SpaceUserRole)]
pub async fn list_chapters(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
) -> Result<Vec<SpaceChapter>> {
    let config = crate::common::CommonConfig::default();
    let cli = config.dynamodb();
    let space_pk: Partition = space_id.into();

    let chapter_sk_prefix = EntityType::SpaceChapter(String::new()).to_string();
    let (mut chapters, _) = SpaceChapter::query(
        cli,
        &space_pk,
        SpaceChapter::opt()
            .sk(chapter_sk_prefix)
            .limit(1_000_000)
            .scan_index_forward(true),
    )
    .await
    .map_err(|e| {
        crate::error!("list_chapters: failed to load chapters: {e}");
        Error::InternalServerError("failed to load chapters".into())
    })?;

    chapters.sort_by_key(|c| c.order);
    Ok(chapters)
}
