use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChapterRequest {
    pub name: String,
    #[serde(default)]
    pub actor_role: Option<SpaceUserRole>,
    #[serde(default)]
    pub completion_benefit: Option<ChapterBenefit>,
}

#[mcp_tool(name = "create_chapter", description = "Create a new chapter in a space. Requires creator role.")]
#[post("/api/spaces/{space_id}/chapters", role: SpaceUserRole)]
pub async fn create_chapter(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Chapter creation data: name, optional actor_role, optional completion_benefit")]
    req: CreateChapterRequest,
) -> Result<String> {
    if role != SpaceUserRole::Creator {
        return Err(Error::Unauthorized("Only creators can create chapters".into()));
    }

    let config = crate::common::CommonConfig::default();
    let cli = config.dynamodb();
    let space_pk: Partition = space_id.clone().into();

    // Find max order among existing chapters to auto-assign order + 1
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
        crate::error!("create_chapter: failed to load chapters: {e}");
        Error::InternalServerError("failed to load chapters".into())
    })?;

    let max_order = existing_chapters.iter().map(|c| c.order).max().unwrap_or(0);
    let new_order = if existing_chapters.is_empty() {
        0
    } else {
        max_order + 1
    };

    let chapter_id = uuid::Uuid::new_v4().to_string();
    let actor_role = req.actor_role.unwrap_or(SpaceUserRole::Participant);
    let completion_benefit = req.completion_benefit.unwrap_or(ChapterBenefit::XpOnly);

    let chapter = SpaceChapter::new(
        space_id,
        chapter_id.clone(),
        new_order,
        req.name,
        actor_role,
        completion_benefit,
    );

    chapter.create(cli).await.map_err(|e| {
        crate::error!("create_chapter: failed to create chapter: {e}");
        Error::InternalServerError("failed to create chapter".into())
    })?;

    Ok(chapter_id)
}
