use super::*;

#[mcp_tool(name = "delete_chapter", description = "Delete an empty chapter from a space. Fails if the chapter has actions assigned. Requires creator role.")]
#[delete("/api/spaces/{space_id}/chapters/{chapter_id}", role: SpaceUserRole)]
pub async fn delete_chapter(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Chapter ID to delete")]
    chapter_id: String,
) -> Result<String> {
    if role != SpaceUserRole::Creator {
        return Err(Error::Unauthorized(
            "Only creators can delete chapters".into(),
        ));
    }

    let config = crate::common::CommonConfig::default();
    let cli = config.dynamodb();
    let space_pk: Partition = space_id.into();

    let (pk, sk) = SpaceChapter::keys(&space_pk, &chapter_id);
    let _chapter = SpaceChapter::get(cli, &pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("delete_chapter: failed to get chapter: {e}");
            Error::InternalServerError("failed to get chapter".into())
        })?
        .ok_or(GamificationError::ChapterNotFound)?;

    // Reject if chapter has actions assigned
    let (space_actions, _) =
        SpaceAction::find_by_space(cli, &space_pk, SpaceAction::opt()).await.map_err(|e| {
            crate::error!("delete_chapter: failed to load actions: {e}");
            Error::InternalServerError("failed to load actions".into())
        })?;

    let has_actions = space_actions.iter().any(|a| {
        a.chapter_id
            .as_ref()
            .map(|cid| cid.0 == chapter_id)
            .unwrap_or(false)
    });

    if has_actions {
        return Err(GamificationError::ChapterNotEmpty.into());
    }

    SpaceChapter::delete(cli, pk, sk).await.map_err(|e| {
        crate::error!("delete_chapter: failed to delete chapter: {e}");
        Error::InternalServerError("failed to delete chapter".into())
    })?;

    Ok("Chapter deleted".to_string())
}
