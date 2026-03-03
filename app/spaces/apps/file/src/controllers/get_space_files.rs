use crate::*;
#[cfg(feature = "server")]
use crate::models::SpaceFile;
#[cfg(feature = "server")]
use common::SpaceUserRole;

#[get("/api/spaces/{space_pk}/files", _role: SpaceUserRole)]
pub async fn get_space_files(space_pk: SpacePartition) -> Result<Vec<File>> {
    let cli = crate::config::get().common.dynamodb();

    let space_pk: Partition = space_pk.into();
    let (pk, sk) = SpaceFile::keys(&space_pk);

    let mut space_file = SpaceFile::get(cli, &pk, Some(sk.clone()))
        .await?
        .unwrap_or_default();

    // Lazy migration: auto-generate IDs for files that don't have them
    let mut needs_update = false;
    for file in &mut space_file.files {
        if file.id.is_empty() {
            file.id = common::uuid::Uuid::now_v7().to_string();
            needs_update = true;
        }
    }

    if needs_update {
        SpaceFile::updater(&pk, sk)
            .with_files(space_file.files.clone())
            .execute(cli)
            .await?;
    }

    Ok(space_file.files)
}
