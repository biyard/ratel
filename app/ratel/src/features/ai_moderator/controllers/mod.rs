pub mod delete_material;
pub mod get_ai_moderator_config;
pub mod list_materials;
pub mod update_ai_moderator_config;
pub mod upload_material;

pub use delete_material::*;
pub use get_ai_moderator_config::*;
pub use list_materials::*;
pub use update_ai_moderator_config::*;
pub use upload_material::*;

use super::*;

#[cfg(feature = "server")]
pub(crate) async fn require_premium(
    cli: &aws_sdk_dynamodb::Client,
    user: &crate::features::auth::User,
) -> Result<()> {
    use crate::features::membership::models::UserMembership;

    let membership =
        UserMembership::get(cli, user.pk.clone(), Some(EntityType::UserMembership)).await?;

    let is_paid = membership
        .as_ref()
        .map_or(false, |m| !m.membership_pk.0.contains("Free"));

    if !is_paid {
        return Err(AiModeratorError::PremiumRequired.into());
    }
    Ok(())
}
