use crate::features::spaces::pages::apps::apps::file::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::apps::apps::file::models::{FileLink, SpaceFile};
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateFileLinkRequest {
    pub file_url: String,
    pub file_name: Option<String>,
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
        pk: space_pk.clone(),
        sk: EntityType::FileLink(link_id),
        file_url: req.file_url.clone(),
        link_target: req.link_target,
        created_at: now,
        updated_at: now,
    };

    file_link.create(cli).await?;

    let (pk, sk) = SpaceFile::keys(&space_pk);
    let name = req
        .file_name
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| extract_filename_from_url(&req.file_url));
    let ext = FileExtension::from_name_or_url(&name, &req.file_url);
    let new_file = File {
        id: crate::common::uuid::Uuid::now_v7().to_string(),
        name,
        ext,
        url: Some(req.file_url),
        ..Default::default()
    };

    let existing = SpaceFile::get(cli, &pk, Some(sk.clone())).await?;
    if let Some(mut space_file) = existing {
        space_file.files.push(new_file);
        SpaceFile::updater(&pk, sk)
            .with_files(space_file.files)
            .execute(cli)
            .await?;
    } else {
        SpaceFile::new(space_pk, vec![new_file]).create(cli).await?;
    }

    Ok(())
}
