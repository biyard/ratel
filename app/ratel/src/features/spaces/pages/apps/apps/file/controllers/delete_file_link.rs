use crate::features::spaces::pages::apps::apps::file::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::apps::apps::file::models::{FileLink, SpaceFile};
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

    for link in &links {
        if link.file_url == req.file_url && link.link_target == req.link_target {
            FileLink::delete(cli, &link.pk, Some(link.sk.clone())).await?;
            break;
        }
    }

    let remaining = links
        .iter()
        .filter(|l| l.file_url == req.file_url && l.link_target != req.link_target)
        .count();

    if remaining == 0 {
        let (pk, sk) = SpaceFile::keys(&space_pk);
        if let Some(mut space_file) = SpaceFile::get(cli, &pk, Some(sk.clone())).await? {
            space_file.files.retain(|f| f.url.as_deref() != Some(&req.file_url));
            SpaceFile::updater(&pk, sk)
                .with_files(space_file.files)
                .execute(cli)
                .await?;
        }
    }

    Ok(())
}
