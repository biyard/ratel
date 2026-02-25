use crate::*;
use common::models::space::SpaceCommon;

mod get_space_apps;
mod install_space_app;
mod uninstall_space_app;

pub use get_space_apps::*;
pub use install_space_app::*;
pub use uninstall_space_app::*;

//FIXME : Use Extension Instead. (Future)
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
