#[cfg(feature = "server")]
use crate::*;
#[cfg(feature = "server")]
use common::models::space::SpaceCommon;

mod get_space_apps;
mod install_space_app;
mod uninstall_space_app;

pub use get_space_apps::*;
pub use install_space_app::*;
pub use uninstall_space_app::*;

#[cfg(feature = "server")]
fn parse_app_name(raw: &str) -> Result<SpaceAppName> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err(Error::BadRequest("app_name is empty".to_string()));
    }
    SpaceAppName::try_from(raw)
        .map_err(|_| Error::BadRequest(format!("invalid app_name: {raw}")))
}

#[cfg(feature = "server")]
async fn ensure_space_exists(
    dynamo: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<()> {
    let space: Option<SpaceCommon> =
        SpaceCommon::get(dynamo, space_pk, Some(&EntityType::SpaceCommon)).await?;
    if space.is_none() {
        return Err(Error::NotFound("Space not found".to_string()));
    }
    Ok(())
}
