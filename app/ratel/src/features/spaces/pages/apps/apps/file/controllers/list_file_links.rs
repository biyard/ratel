use crate::features::spaces::pages::apps::apps::file::types::FileLinkTarget;
use crate::features::spaces::pages::apps::apps::file::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::apps::apps::file::models::FileLink;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FileLinkInfo {
    pub file_url: String,
    pub link_target: FileLinkTarget,
}

#[get("/api/spaces/{space_pk}/file-links", _role: SpaceUserRole)]
pub async fn list_file_links(space_pk: SpacePartition) -> Result<Vec<FileLinkInfo>> {
    let cli = crate::features::spaces::pages::apps::apps::file::config::get().common.dynamodb();

    let space_pk: Partition = space_pk.into();

    let links = FileLink::list_by_space(cli, &space_pk).await?;

    Ok(links
        .into_iter()
        .map(|link| FileLinkInfo {
            file_url: link.file_url,
            link_target: link.link_target,
        })
        .collect())
}
