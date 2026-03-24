use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::models::{OptionalUser, User};
use crate::common::types::{FeedPartition, Partition, SpacePartition};
use crate::features::posts::models::Post;
use crate::features::posts::types::{BoosterType, SpaceType};
use crate::features::spaces::space_common::*;

#[get("/api/spaces/{space_id}", user: OptionalUser)]
pub async fn get_space(space_id: SpacePartition) -> Result<SpaceResponse> {
    let config = crate::features::spaces::space_common::config::get();
    let dynamo = config.common.dynamodb();

    let space_pk_partition: Partition = space_id.into();
    let space =
        SpaceCommon::get(dynamo, &space_pk_partition, Some(&EntityType::SpaceCommon)).await?;
    let space = space.ok_or_else(|| Error::NotFound("Space Not Found".to_string()))?;

    let post_pk = space.pk.clone().to_post_key()?;
    let post = Post::get(dynamo, &post_pk, Some(EntityType::Post)).await?;
    let post = post.ok_or_else(|| Error::NotFound("Post Not Found".to_string()))?;
    let user: Option<User> = user.into();

    let permissions = post.get_permissions(dynamo, user.clone()).await?;
    let liked = if let Some(ref user) = user {
        post.is_liked(dynamo, &user.pk).await?
    } else {
        false
    };

    let is_participation_open = space.is_participation_open();

    let (user_participant, can_participate) = if let Some(ref user) = user {
        let (participant_pk, participant_sk) =
            SpaceParticipant::keys(space.pk.clone(), user.pk.clone());
        let participant =
            SpaceParticipant::get(dynamo, &participant_pk, Some(&participant_sk)).await?;
        let invited = if space.visibility != SpaceVisibility::Public {
            let (invitation_pk, invitation_sk) = SpaceInvitationMember::keys(&space.pk, &user.pk);
            let invitation =
                SpaceInvitationMember::get(dynamo, &invitation_pk, Some(&invitation_sk)).await?;

            matches!(
                invitation.as_ref().map(|member| member.status),
                Some(InvitationStatus::Invited) | Some(InvitationStatus::Accepted)
            )
        } else {
            true
        };
        let can_participate = participant.is_none() && is_participation_open && invited;
        (participant, can_participate)
    } else {
        (None, false)
    };

    let (participated, participant_display_name, participant_profile_url, participant_username) =
        if let Some(participant) = user_participant {
            (
                true,
                Some(participant.display_name),
                Some(participant.profile_url),
                Some(participant.username),
            )
        } else {
            (false, None, None, None)
        };

    Ok(SpaceResponse {
        id: space.pk.clone().into(),
        post_id: post_pk.into(),
        sk: space.sk,
        title: post.title,
        content: if space.content.is_empty() {
            post.html_contents
        } else {
            space.content
        },
        created_at: space.created_at,
        updated_at: space.updated_at,
        urls: post.urls,
        space_type: post.space_type.unwrap_or_default(),
        features: vec![],
        status: space.status,
        permissions: permissions.into(),
        author_type: post.author_type,
        author_display_name: post.author_display_name,
        author_username: post.author_username,
        author_profile_url: post.author_profile_url,
        certified: false,
        likes: post.likes,
        comments: post.comments,
        shares: post.shares,
        liked,
        reports: space.reports,
        rewards: space.rewards,
        visibility: space.visibility,
        publish_state: space.publish_state,
        booster: post.booster.unwrap_or_default(),
        files: None,
        anonymous_participation: space.anonymous_participation,
        join_anytime: space.join_anytime,
        can_participate,
        participated,
        participant_display_name,
        participant_profile_url,
        participant_username,
        remains: space.remains,
        quota: space.quota,
        is_report: false,
        logo: space.logo,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpaceResponse {
    pub id: SpacePartition,
    pub post_id: FeedPartition,
    pub sk: EntityType,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub urls: Vec<String>,
    pub space_type: SpaceType,
    pub features: Vec<String>,
    pub status: Option<SpaceStatus>,
    pub permissions: i64,
    pub author_type: UserType,
    pub author_display_name: String,
    pub author_username: String,
    pub author_profile_url: String,
    pub certified: bool,
    pub likes: i64,
    pub comments: i64,
    pub shares: i64,
    pub liked: bool,
    pub reports: i64,
    pub rewards: Option<i64>,
    pub visibility: SpaceVisibility,
    pub publish_state: SpacePublishState,
    pub booster: BoosterType,
    pub files: Option<Vec<File>>,
    pub anonymous_participation: bool,
    #[serde(default)]
    pub join_anytime: bool,
    pub can_participate: bool,
    pub participated: bool,
    pub participant_display_name: Option<String>,
    pub participant_profile_url: Option<String>,
    pub participant_username: Option<String>,
    pub remains: i64,
    pub quota: i64,
    pub is_report: bool,
    #[serde(default)]
    pub logo: String,
}

impl SpaceResponse {
    pub fn description(&self) -> String {
        let re = regex::Regex::new(r"<[^>]*>").unwrap();
        let content = re.replace_all(&self.content, "");

        content.to_string()
    }
}
