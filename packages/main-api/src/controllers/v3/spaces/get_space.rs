use super::*;
use crate::features::report::ContentReport;
use crate::features::spaces::members::{SpaceEmailVerification, SpaceInvitationMember};
use crate::features::spaces::{
    SpaceDao, SpaceRequirement, SpaceRequirementDto, SpaceRequirementQueryOption,
    SpaceRequirementResponse,
};
use crate::models::user::User;
use crate::models::{Post, SpaceCommon};
use crate::types::*;
use crate::{AppState, Error};
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    aide::OperationIo,
    schemars::JsonSchema,
)]
pub struct GetSpaceResponse {
    pub pk: Partition,
    pub sk: EntityType,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub urls: Vec<String>,
    pub space_type: SpaceType,
    // TODO: implemente real features
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
    pub reports: i64,
    pub rewards: Option<i64>,
    pub visibility: SpaceVisibility,
    pub publish_state: SpacePublishState,
    pub booster: BoosterType,

    pub files: Option<Vec<File>>,

    pub anonymous_participation: bool,

    pub can_participate: bool,
    pub change_visibility: bool,
    pub participated: bool,
    pub participant_display_name: Option<String>,
    pub participant_profile_url: Option<String>,
    pub participant_username: Option<String>,

    pub requirements: Vec<SpaceRequirementDto>,
    pub remains: i64,
    pub quota: i64,

    pub is_report: bool,
    pub dao_address: Option<String>,
}

pub async fn get_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    NoApi(perms): NoApi<Permissions>,
    Extension(space): Extension<SpaceCommon>,
) -> Result<Json<GetSpaceResponse>> {
    perms.permitted(TeamGroupPermission::SpaceRead)?;
    let space_pk = space.pk.clone();

    let post_pk = space_pk.clone().to_post_key()?;
    let post = Post::get(&dynamo.client, &post_pk, Some(&EntityType::Post)).await?;

    let post = post.ok_or(Error::PostNotFound)?;

    let permissions = post.get_permissions(&dynamo.client, user.clone()).await?;

    let user_participant = if user.is_some() {
        let (pk, sk) = SpaceParticipant::keys(space_pk.clone(), user.as_ref().unwrap().pk.clone());
        SpaceParticipant::get(&dynamo.client, pk, Some(sk)).await?
    } else {
        None
    };

    let (req_pk, sk) = SpaceRequirement::keys(&space_pk, None);
    // NOTE: Currently, space requirement will be just one or zero.
    // If it is extended to lots of requirements, we need to implement pagination.
    let opt = SpaceRequirement::opt_all().sk(sk.to_string());

    let (mut requirements, _bookmark) =
        SpaceRequirement::query(&dynamo.client, req_pk, opt).await?;

    let dao = SpaceDao::get(&dynamo.client, &space_pk, Some(&EntityType::SpaceDao)).await?;

    let can_participate = if let Some(ref user) = user {
        let (pk, sk) = SpaceInvitationMember::keys(&space.pk, &user.pk);
        let invitation = SpaceInvitationMember::get(&dynamo.client, &pk, Some(&sk)).await?;

        invitation.is_some()
            && user_participant.is_none()
            && space.status == Some(SpaceStatus::InProgress)
    } else {
        false
    };

    let mut res = GetSpaceResponse::from((space.clone(), post, permissions, user_participant));
    res.dao_address = dao.map(|item| item.contract_address);
    requirements.sort_by(|a, b| a.order.cmp(&b.order));

    let (is_report, keys) = if let Some(ref user) = user {
        let is_report = ContentReport::is_reported_for_target_by_user(
            &dynamo.client,
            &space.clone().pk,
            Some(&space.clone().sk),
            &user.clone().pk,
        )
        .await?;
        let keys = requirements
            .iter()
            .map(|e| {
                e.get_respondent_keys(&user.pk)
                    .expect("failed to get respondent key")
            })
            .collect();
        (is_report, keys)
    } else {
        (false, vec![])
    };

    let resp = SpaceRequirementResponse::batch_get(&dynamo.client, keys).await?;

    res.requirements = requirements
        .into_iter()
        .map(|r| SpaceRequirementDto::new(r, &user, &resp))
        .collect();
    res.can_participate = can_participate;
    res.is_report = is_report;

    Ok(Json(res))
}

impl
    From<(
        SpaceCommon,
        Post,
        TeamGroupPermissions,
        Option<SpaceParticipant>,
    )> for GetSpaceResponse
{
    fn from(
        (space, post, permissions, user_participant): (
            SpaceCommon,
            Post,
            TeamGroupPermissions,
            Option<SpaceParticipant>,
        ),
    ) -> Self {
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

        Self {
            pk: space.pk,
            sk: space.sk,
            title: post.title,
            content: match space.content.is_empty() {
                true => post.html_contents,
                false => space.content,
            },
            created_at: space.created_at,
            updated_at: space.updated_at,
            urls: post.urls,
            space_type: space.space_type,
            features: vec![],
            status: space.status,
            permissions: permissions.into(),
            author_type: post.author_type,
            author_display_name: post.author_display_name,
            author_username: post.author_username,
            author_profile_url: post.author_profile_url,

            // TODO: implement real certification check
            certified: false,
            likes: post.likes,
            comments: post.comments,
            shares: post.shares,
            reports: space.reports,
            rewards: space.rewards,
            visibility: space.visibility,
            publish_state: space.publish_state,
            booster: space.booster,
            files: space.files,
            anonymous_participation: space.anonymous_participation,
            can_participate: false,
            change_visibility: space.change_visibility,
            participated,
            participant_display_name,
            participant_profile_url,
            participant_username,
            requirements: vec![],
            remains: space.remains,
            quota: space.quota,

            is_report: false,
            dao_address: None,
        }
    }
}
