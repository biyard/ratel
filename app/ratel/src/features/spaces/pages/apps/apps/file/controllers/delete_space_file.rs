use crate::features::spaces::pages::apps::apps::file::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::apps::apps::file::models::SpaceFile;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeleteSpaceFileRequest {
    pub file_url: String,
}

#[delete("/api/spaces/{space_pk}/files", role: SpaceUserRole)]
pub async fn delete_space_file(
    space_pk: SpacePartition,
    req: DeleteSpaceFileRequest,
) -> Result<bool> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let cli = crate::features::spaces::pages::apps::apps::file::config::get().common.dynamodb();

    let space_pk: Partition = space_pk.into();
    let (pk, sk) = SpaceFile::keys(&space_pk);

    if let Some(mut space_file) = SpaceFile::get(cli, &pk, Some(sk.clone())).await? {
        space_file
            .files
            .retain(|f| f.url.as_ref() != Some(&req.file_url));
        SpaceFile::updater(&pk, sk)
            .with_files(space_file.files)
            .execute(cli)
            .await?;
    }

    Ok(true)
}
