use crate::features::spaces::pages::apps::apps::file::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::apps::apps::file::models::FileLink;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateFileLinkRequest {
    pub file_url: String,
    pub link_target: FileLinkTarget,
}

#[post("/api/spaces/{space_pk}/file-links", _role: SpaceUserRole)]
pub async fn create_file_link(
    space_pk: SpacePartition,
    req: CreateFileLinkRequest,
) -> Result<()> {
    let cli = crate::features::spaces::pages::apps::apps::file::config::get().common.dynamodb();

    let space_pk: Partition = space_pk.into();
    let now = crate::common::utils::time::now();
    let link_id = crate::common::uuid::Uuid::now_v7().to_string();

    let file_link = FileLink {
        pk: space_pk,
        sk: EntityType::FileLink(link_id),
        file_url: req.file_url,
        link_target: req.link_target,
        created_at: now,
        updated_at: now,
    };

    file_link.create(cli).await?;

    Ok(())
}
