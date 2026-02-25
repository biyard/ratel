use crate::models::SpaceApp;
use crate::*;
use common::types::Partition;
use common::types::SpacePartition;
use ratel_auth::models::user::OptionalUser;

#[get("/api/spaces/{space_id}/apps", user: OptionalUser)]
pub async fn get_space_apps(space_id: SpacePartition) -> Result<Vec<SpaceApp>> {
    let dynamo = crate::config::get().common.dynamodb();
    let space_pk: Partition = space_id.clone().into();

    #[cfg(feature = "server")]
    {
        let user: Option<ratel_auth::User> = user.into();
        super::get_apps_access::ensure_space_admin(space_id, user).await?;
    }

    let (apps, _) = SpaceApp::query(
        dynamo,
        &space_pk,
        SpaceApp::opt_all()
            .sk(SpaceApp::sk_prefix())
            .scan_index_forward(true),
    )
    .await?;

    Ok(apps)
}
