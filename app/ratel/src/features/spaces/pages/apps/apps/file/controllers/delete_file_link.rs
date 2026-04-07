use crate::features::spaces::pages::apps::apps::file::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::apps::apps::file::models::FileLink;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteFileLinkRequest {
    pub file_url: String,
    pub link_target: FileLinkTarget,
}

#[delete("/api/spaces/{space_pk}/file-links", _role: SpaceUserRole)]
pub async fn delete_file_link(
    space_pk: SpacePartition,
    req: DeleteFileLinkRequest,
) -> Result<()> {
    let cli = crate::features::spaces::pages::apps::apps::file::config::get().common.dynamodb();

    let space_pk: Partition = space_pk.into();

    let links = FileLink::list_by_space(cli, &space_pk).await?;

    for link in links {
        if link.file_url == req.file_url && link.link_target == req.link_target {
            FileLink::delete(cli, &link.pk, Some(link.sk)).await?;
            break;
        }
    }

    Ok(())
}
