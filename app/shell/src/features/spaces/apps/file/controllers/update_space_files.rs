use crate::features::spaces::apps::file::*;
#[cfg(feature = "server")]
use crate::features::spaces::apps::file::models::SpaceFile;
#[cfg(feature = "server")]
use common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSpaceFilesRequest {
    pub files: Vec<File>,
}

#[patch("/api/spaces/{space_pk}/files", role: SpaceUserRole)]
pub async fn update_space_files(
    space_pk: SpacePartition,
    req: UpdateSpaceFilesRequest,
) -> Result<Vec<File>> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let cli = crate::features::spaces::apps::file::config::get().common.dynamodb();

    let space_pk: Partition = space_pk.into();
    let (pk, sk) = SpaceFile::keys(&space_pk);

    // Ensure all files have IDs
    let mut files_with_ids = req.files;
    for file in &mut files_with_ids {
        if file.id.is_empty() {
            file.id = common::uuid::Uuid::now_v7().to_string();
        }
    }

    let existing = SpaceFile::get(cli, &pk, Some(sk.clone())).await?;

    if existing.is_some() {
        SpaceFile::updater(&pk, sk)
            .with_files(files_with_ids.clone())
            .execute(cli)
            .await?;
    } else {
        let space_file = SpaceFile::new(space_pk, files_with_ids.clone());
        space_file.create(cli).await?;
    }

    Ok(files_with_ids)
}
