use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderChaptersRequest {
    pub ordered_ids: Vec<String>,
}

#[mcp_tool(name = "reorder_chapters", description = "Reorder chapters in a space by providing the full ordered list of chapter IDs. Requires creator role.")]
#[post("/api/spaces/{space_id}/chapters/reorder", role: SpaceUserRole)]
pub async fn reorder_chapters(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Reorder data: ordered_ids array with all chapter IDs in desired order")]
    req: ReorderChaptersRequest,
) -> Result<String> {
    if role != SpaceUserRole::Creator {
        return Err(Error::Unauthorized(
            "Only creators can reorder chapters".into(),
        ));
    }

    let config = crate::common::CommonConfig::default();
    let cli = config.dynamodb();
    let space_pk: Partition = space_id.into();

    // Load existing chapters
    let chapter_sk_prefix = EntityType::SpaceChapter(String::new()).to_string();
    let (existing_chapters, _) = SpaceChapter::query(
        cli,
        &space_pk,
        SpaceChapter::opt()
            .sk(chapter_sk_prefix)
            .limit(1_000_000)
            .scan_index_forward(true),
    )
    .await
    .map_err(|e| {
        crate::error!("reorder_chapters: failed to load chapters: {e}");
        Error::InternalServerError("failed to load chapters".into())
    })?;

    // Validate length matches
    if req.ordered_ids.len() != existing_chapters.len() {
        return Err(Error::BadRequest(format!(
            "Expected {} chapter ids but got {}",
            existing_chapters.len(),
            req.ordered_ids.len()
        )));
    }

    // Build a lookup map of existing chapter ids for validation
    let existing_ids: std::collections::HashSet<String> = existing_chapters
        .iter()
        .filter_map(|c| match &c.sk {
            EntityType::SpaceChapter(id) => Some(id.clone()),
            _ => None,
        })
        .collect();

    // Validate all provided ids exist
    for id in &req.ordered_ids {
        if !existing_ids.contains(id) {
            return Err(Error::BadRequest(format!("Chapter id not found: {id}")));
        }
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();

    // Update each chapter's order field
    for (new_order, chapter_id) in req.ordered_ids.iter().enumerate() {
        let (pk, sk) = SpaceChapter::keys(&space_pk, chapter_id);
        SpaceChapter::updater(&pk, &sk)
            .with_order(new_order as u32)
            .with_updated_at(now)
            .execute(cli)
            .await
            .map_err(|e| {
                crate::error!("reorder_chapters: failed to update chapter {chapter_id}: {e}");
                Error::InternalServerError("failed to reorder chapters".into())
            })?;
    }

    Ok("Chapters reordered".to_string())
}
