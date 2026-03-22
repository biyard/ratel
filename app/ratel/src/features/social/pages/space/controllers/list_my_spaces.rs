use super::super::*;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MySpaceResponse {
    pub space_pk: Partition,
    pub post_pk: Partition,
    pub title: String,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub created_at: i64,
    pub visibility: SpaceVisibility,
    pub status: Option<SpaceStatus>,
    pub logo: String,
    pub participants: i64,
}

#[get("/api/me/spaces?bookmark", user: crate::features::auth::User)]
pub async fn list_my_spaces_handler(
    bookmark: Option<String>,
) -> Result<ListResponse<MySpaceResponse>> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let mut opt = SpaceParticipant::opt().limit(10);
    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (participants, bookmark) = SpaceParticipant::find_by_user(cli, &user.pk, opt).await?;

    let space_keys: Vec<(Partition, EntityType)> = participants
        .iter()
        .map(|sp| (sp.space_pk.clone(), EntityType::SpaceCommon))
        .collect();

    let spaces: Vec<SpaceCommon> = if space_keys.is_empty() {
        vec![]
    } else {
        SpaceCommon::batch_get(cli, space_keys.clone()).await?
    };

    let post_keys: Vec<(Partition, EntityType)> = spaces
        .iter()
        .filter_map(|s| s.pk.clone().to_post_key().ok())
        .map(|pk| (pk, EntityType::Post))
        .collect();

    let posts: Vec<crate::features::posts::models::Post> = if post_keys.is_empty() {
        vec![]
    } else {
        crate::features::posts::models::Post::batch_get(cli, post_keys).await?
    };

    // Filter server-side to only include active spaces (InProgress/Started)
    // to avoid pagination edge cases where the first page might contain only
    // inactive spaces, causing the timeline section to be hidden incorrectly.
    let items: Vec<MySpaceResponse> = spaces
        .into_iter()
        .filter(|space| space.is_active())
        .map(|space| {
            let title = space
                .pk
                .clone()
                .to_post_key()
                .ok()
                .and_then(|post_pk| {
                    posts
                        .iter()
                        .find(|p| p.pk == post_pk)
                        .map(|p| p.title.clone())
                })
                .unwrap_or_default();

            MySpaceResponse {
                space_pk: space.pk.clone(),
                post_pk: space.post_pk,
                title,
                author_display_name: space.author_display_name,
                author_profile_url: space.author_profile_url,
                created_at: space.created_at,
                visibility: space.visibility,
                status: space.status,
                logo: space.logo,
                participants: space.participants,
            }
        })
        .collect();

    Ok(ListResponse { items, bookmark })
}
