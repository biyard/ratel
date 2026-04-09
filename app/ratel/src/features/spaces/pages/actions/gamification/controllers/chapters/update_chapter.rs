use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChapterRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub actor_role: Option<SpaceUserRole>,
    #[serde(default)]
    pub completion_benefit: Option<ChapterBenefit>,
}

#[cfg(feature = "server")]
fn is_forward_role_upgrade(target: SpaceUserRole) -> bool {
    matches!(
        target,
        SpaceUserRole::Candidate | SpaceUserRole::Participant
    )
}

#[mcp_tool(name = "update_chapter", description = "Update a chapter's name, description, actor role, or completion benefit. Requires creator role.")]
#[patch("/api/spaces/{space_id}/chapters/{chapter_id}", role: SpaceUserRole)]
pub async fn update_chapter(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Chapter ID to update")]
    chapter_id: String,
    #[mcp(description = "Chapter update data: optional name, description, actor_role, completion_benefit")]
    req: UpdateChapterRequest,
) -> Result<SpaceChapter> {
    if role != SpaceUserRole::Creator {
        return Err(Error::Unauthorized("Only creators can update chapters".into()));
    }

    let config = crate::common::CommonConfig::default();
    let cli = config.dynamodb();
    let space_pk: Partition = space_id.into();

    let (pk, sk) = SpaceChapter::keys(&space_pk, &chapter_id);
    let mut chapter = SpaceChapter::get(cli, &pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("update_chapter: failed to get chapter: {e}");
            Error::InternalServerError("failed to get chapter".into())
        })?
        .ok_or(GamificationError::ChapterNotFound)?;

    // Validate role upgrade direction if benefit involves a role change
    if let Some(ref benefit) = req.completion_benefit {
        match benefit {
            ChapterBenefit::RoleUpgradeTo(target_role)
            | ChapterBenefit::RoleUpgradeAndXp(target_role) => {
                if !is_forward_role_upgrade(*target_role) {
                    return Err(Error::BadRequest(
                        "Role upgrade must be forward: Viewer -> Candidate -> Participant".into(),
                    ));
                }
            }
            ChapterBenefit::XpOnly => {}
        }
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();

    let mut updater = SpaceChapter::updater(&pk, &sk).with_updated_at(now);

    if let Some(name) = req.name.clone() {
        chapter.name = name.clone();
        updater = updater.with_name(name);
    }

    if let Some(description) = req.description.clone() {
        chapter.description = Some(description.clone());
        updater = updater.with_description(Some(description));
    }

    if let Some(actor_role) = req.actor_role {
        chapter.actor_role = actor_role;
        updater = updater.with_actor_role(actor_role);
    }

    if let Some(benefit) = req.completion_benefit.clone() {
        chapter.completion_benefit = benefit.clone();
        updater = updater.with_completion_benefit(benefit);
    }

    updater.execute(cli).await.map_err(|e| {
        crate::error!("update_chapter: failed to update chapter: {e}");
        Error::InternalServerError("failed to update chapter".into())
    })?;

    chapter.updated_at = now;
    Ok(chapter)
}
