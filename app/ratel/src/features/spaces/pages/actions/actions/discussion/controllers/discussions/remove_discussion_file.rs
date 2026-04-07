use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::apps::apps::file::{
    delete_file_link, DeleteFileLinkRequest, FileLinkTarget,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RemoveDiscussionFileRequest {
    pub file_url: String,
}

#[delete("/api/spaces/{space_id}/discussions/{discussion_id}/files", role: SpaceUserRole)]
pub async fn remove_discussion_file(
    space_id: SpacePartition,
    discussion_id: SpacePostEntityType,
    req: RemoveDiscussionFileRequest,
) -> Result<()> {
    SpacePost::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();
    let link_target = FileLinkTarget::Board(discussion_id.to_string());
    let discussion_sk: EntityType = discussion_id.into();

    let post = SpacePost::get(cli, &space_pk, Some(discussion_sk.clone()))
        .await?
        .ok_or(Error::NotFound("Discussion not found".into()))?;

    let updated_files: Vec<File> = post
        .files
        .into_iter()
        .filter(|f| f.url.as_ref() != Some(&req.file_url))
        .collect();

    SpacePost::updater(&space_pk, &discussion_sk)
        .with_files(updated_files)
        .execute(cli)
        .await?;

    delete_file_link(
        space_id,
        DeleteFileLinkRequest {
            file_url: req.file_url,
            link_target,
        },
    )
    .await?;

    Ok(())
}
