use crate::types::FileLinkTarget;
use crate::*;
#[cfg(feature = "server")]
use crate::models::FileLink;
#[cfg(feature = "server")]
use common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FileLinkInfo {
    pub file_url: String,
    pub link_target: FileLinkTarget,
}

#[get("/api/spaces/{space_pk}/file-links", _role: SpaceUserRole)]
pub async fn list_file_links(space_pk: SpacePartition) -> Result<Vec<FileLinkInfo>> {
    let cli = crate::config::get().common.dynamodb();

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
