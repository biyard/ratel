use crate::controllers::v3::spaces::dto::*;
use crate::types::{Partition, TeamGroupPermission, EntityType};
use crate::{AppState, Error, Permissions};
use aide::NoApi;
use crate::features::spaces::files::{FileLink, FileLinkTarget, SpaceFile};
use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::models::space::SpaceCommon;
use axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
pub struct DeleteFileQuery {
    #[schemars(description = "URL of the file to delete")]
    pub file_url: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct DeleteFileResponse {
    pub success: bool,
    pub message: String,
}

/// Delete a file from the space
/// When a file is deleted from the files tab, it cascades to remove it from Overview/Boards
pub async fn delete_files_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(query): Query<DeleteFileQuery>,
) -> Result<Json<DeleteFileResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let file_url = query.file_url;

    // Get the file link to determine origin
    let file_link = FileLink::find_by_url(&dynamo.client, &space_pk, &file_url).await?;

    if let Some(link) = file_link {
        let origin = link.link_target.clone();
        let mut transactions = Vec::new();
        
        // Delete the file link
        let delete_link_tx = FileLink::delete_transact_write_item(link.pk.clone(), link.sk.clone());
        transactions.push(delete_link_tx);

        // Always remove from SpaceFile (Files tab)
        let (pk, sk) = SpaceFile::keys(&space_pk);
        if let Some(space_file) = SpaceFile::get(&dynamo.client, &pk, Some(sk.clone())).await? {
            let mut updated_files = space_file.files.clone();
            updated_files.retain(|f| f.url.as_ref() != Some(&file_url));
            
            let update_files_tx = SpaceFile::updater(&pk, sk)
                .with_files(updated_files)
                .transact_write_item();
            transactions.push(update_files_tx);
        }

        // Cascade delete based on origin
        match origin {
            FileLinkTarget::Overview => {
                // Remove from SpaceCommon (Overview)
                if let Some(mut space_common) = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon)).await? {
                    if let Some(ref mut files) = space_common.files {
                        files.retain(|f| f.url.as_ref() != Some(&file_url));
                        
                        let update_overview_tx = SpaceCommon::updater(&space_pk, EntityType::SpaceCommon)
                            .with_files(files.clone())
                            .transact_write_item();
                        transactions.push(update_overview_tx);
                    }
                }
            },
            FileLinkTarget::Board(post_id) => {
                // Remove from board post
                let post_sk = EntityType::SpacePost(post_id);
                if let Some(post) = SpacePost::get(&dynamo.client, space_pk.clone(), Some(post_sk.clone())).await? {
                    if let Some(mut files) = post.files {
                        files.retain(|f| f.url.as_ref() != Some(&file_url));
                        
                        let update_post_tx = SpacePost::updater(&space_pk, post_sk)
                            .with_files(files)
                            .transact_write_item();
                        transactions.push(update_post_tx);
                    }
                }
            },
            FileLinkTarget::Files => {
                // Already removed from SpaceFile above
            }
        }

        // Execute all operations atomically
        if !transactions.is_empty() {
            dynamo
                .client
                .transact_write_items()
                .set_transact_items(Some(transactions))
                .send()
                .await
                .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        }

        Ok(Json(DeleteFileResponse {
            success: true,
            message: "File deleted successfully".to_string(),
        }))
    } else {
        Ok(Json(DeleteFileResponse {
            success: false,
            message: "File not found".to_string(),
        }))
    }
}
