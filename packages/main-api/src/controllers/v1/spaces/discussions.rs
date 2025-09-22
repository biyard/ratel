use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::*;

use crate::utils::notifications::send_notification;
use crate::utils::users::{extract_user_with_allowing_anonymous, extract_user_with_options};
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct SpaceDiscussionController {
    repo: DiscussionRepository,
    participation_repo: DiscussionParticipantRepository,
    member_repo: DiscussionMemberRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl SpaceDiscussionController {
    async fn ensure_current_meeting(
        &self,
        client: &crate::utils::aws_chime_sdk_meeting::ChimeMeetingService,
        discussion: &Discussion,
    ) -> Result<String> {
        if let Some(ref mid) = discussion.meeting_id {
            if client.get_meeting_info(mid).await.is_some() {
                return Ok(mid.clone());
            }
        }

        let created = client.create_meeting(&discussion.name).await.map_err(|e| {
            tracing::error!("create_meeting failed: {:?}", e);
            Error::AwsChimeError(e.to_string())
        })?;

        let new_id = created.meeting_id().unwrap_or_default().to_string();

        self.repo
            .update(
                discussion.id,
                DiscussionRepositoryUpdateRequest {
                    meeting_id: Some(new_id.clone()),
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| {
                tracing::error!("update meeting_id failed: {:?}", e);
                Error::UpdateDiscussionError(e.to_string())
            })?;

        Ok(new_id)
    }

    async fn query(
        &self,
        space_id: i64,
        _auth: Option<Authorization>,
        param: DiscussionQuery,
    ) -> Result<QueryResponse<DiscussionSummary>> {
        let mut total_count = 0;
        let items: Vec<DiscussionSummary> = DiscussionSummary::query_builder()
            .limit(param.size())
            .page(param.page())
            .space_id_equals(space_id)
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn create(
        &self,
        space_id: i64,
        auth: Option<Authorization>,
        DiscussionCreateRequest {
            started_at,
            ended_at,
            name,
            description,
            participants,
            discussion_id,
        }: DiscussionCreateRequest,
    ) -> Result<Discussion> {
        let _discussion_id = discussion_id;
        let user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        let mut tx = self.pool.begin().await?;

        let res = self
            .repo
            .insert_with_tx(
                &mut *tx,
                space_id,
                started_at,
                ended_at,
                user.id,
                name.clone(),
                description,
                None,
                "".to_string(),
                None,
                None,
            )
            .await?;

        let id = res.clone().unwrap_or_default().id;

        let pts = DiscussionMember::query_builder()
            .discussion_id_equals(id)
            .query()
            .map(DiscussionMember::from)
            .fetch_all(&mut *tx)
            .await?;

        for pt in pts {
            let _ = self.member_repo.delete_with_tx(&mut *tx, pt.id).await;
        }

        for participant in participants {
            let _ = self
                .member_repo
                .insert_with_tx(&mut *tx, id, participant)
                .await;

            // Send InviteDiscussion notification to the participant
            let notification_data = NotificationData::InviteDiscussion {
                discussion_id: id,
                image_url: None,
                description: format!("You've been invited to join the discussion: {}", &name),
            };

            if let Err(e) =
                send_notification(&self.pool, &mut tx, participant, notification_data).await
            {
                tracing::error!(
                    "Failed to send InviteDiscussion notification to user {}: {:?}",
                    participant,
                    e
                );
                // Don't fail the entire operation if notification fails - just log the error
            }
        }

        tx.commit().await?;

        Ok(res.unwrap_or_default())
    }

    async fn update(
        &self,
        space_id: i64,
        id: i64,
        auth: Option<Authorization>,
        param: DiscussionUpdateRequest,
    ) -> Result<Discussion> {
        let _ = extract_user_with_allowing_anonymous(&self.pool, auth).await?;
        let _space_id = space_id;

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(
        &self,
        space_id: i64,
        id: i64,
        auth: Option<Authorization>,
    ) -> Result<Discussion> {
        let _ = extract_user_with_allowing_anonymous(&self.pool, auth).await?;
        let _space_id = space_id;

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    async fn start_meeting(&self, id: i64, _auth: Option<Authorization>) -> Result<Discussion> {
        let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;

        let discussion = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(Error::DiscussionNotFound)?;

        let _ = self.ensure_current_meeting(&client, &discussion).await?;

        let latest = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_one(&self.pool)
            .await?;

        Ok(latest)
    }

    async fn participant_meeting(
        &self,
        id: i64,
        auth: Option<Authorization>,
    ) -> Result<Discussion> {
        let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
        let pr = DiscussionParticipant::get_repository(self.pool.clone());

        let user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;
        let user_id = user.id;
        if user_id == 0 {
            return Err(Error::InvalidUser);
        }

        let discussion = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(Error::DiscussionNotFound)?;

        let exists = DiscussionParticipant::query_builder()
            .discussion_id_equals(discussion.id)
            .user_id_equals(user_id)
            .query()
            .map(DiscussionParticipant::from)
            .fetch_optional(&self.pool)
            .await?;
        if exists.is_some() {
            return Ok(discussion);
        }

        let meeting_id = self.ensure_current_meeting(&client, &discussion).await?;

        let m = client
            .get_meeting_info(&meeting_id)
            .await
            .ok_or_else(|| Error::AwsChimeError("Missing meeting from Chime".into()))?;

        let mp = m
            .media_placement()
            .ok_or_else(|| Error::AwsChimeError("Missing media_placement".into()))?;

        let meeting = MeetingInfo {
            meeting_id: meeting_id.clone(),
            media_region: m.media_region.clone().unwrap_or_default(),
            media_placement: MediaPlacementInfo {
                audio_host_url: mp.audio_host_url().unwrap_or_default().to_string(),
                audio_fallback_url: mp.audio_fallback_url().unwrap_or_default().to_string(),
                screen_data_url: mp.screen_data_url().unwrap_or_default().to_string(),
                screen_sharing_url: mp.screen_sharing_url().unwrap_or_default().to_string(),
                screen_viewing_url: mp.screen_viewing_url().unwrap_or_default().to_string(),
                signaling_url: mp.signaling_url().unwrap_or_default().to_string(),
                turn_control_url: mp.turn_control_url().unwrap_or_default().to_string(),
            },
        };

        let create_attendee_once = |_mid: &str| async {
            client
                .create_attendee(&meeting, user_id.to_string().as_str())
                .await
        };

        let attendee_res = match create_attendee_once(&meeting_id).await {
            Ok(r) => Ok(r),
            Err(e) => {
                let msg = e.to_string();
                let not_found = msg.contains("NotFound")
                    || msg.contains("NotFoundException")
                    || msg.to_ascii_lowercase().contains("not found");

                if not_found {
                    tracing::debug!(
                        "meeting not found on create_attendee; recreating and retrying"
                    );
                    let recreated_id = self.ensure_current_meeting(&client, &discussion).await?;
                    let m2 = client
                        .get_meeting_info(&recreated_id)
                        .await
                        .ok_or_else(|| Error::AwsChimeError("Recreated meeting missing".into()))?;
                    let mp2 = m2
                        .media_placement()
                        .ok_or_else(|| Error::AwsChimeError("Missing media_placement".into()))?;
                    let meeting = MeetingInfo {
                        meeting_id: recreated_id.clone(),
                        media_region: m2.media_region.clone().unwrap_or_default(),
                        media_placement: MediaPlacementInfo {
                            audio_host_url: mp2.audio_host_url().unwrap_or_default().to_string(),
                            audio_fallback_url: mp2
                                .audio_fallback_url()
                                .unwrap_or_default()
                                .to_string(),
                            screen_data_url: mp2.screen_data_url().unwrap_or_default().to_string(),
                            screen_sharing_url: mp2
                                .screen_sharing_url()
                                .unwrap_or_default()
                                .to_string(),
                            screen_viewing_url: mp2
                                .screen_viewing_url()
                                .unwrap_or_default()
                                .to_string(),
                            signaling_url: mp2.signaling_url().unwrap_or_default().to_string(),
                            turn_control_url: mp2
                                .turn_control_url()
                                .unwrap_or_default()
                                .to_string(),
                        },
                    };
                    client
                        .create_attendee(&meeting, user_id.to_string().as_str())
                        .await
                } else {
                    Err(e)
                }
            }
        }
        .map_err(|e| {
            tracing::error!("create_attendee error: {:?}", e);
            Error::AwsChimeError(e.to_string())
        })?;

        let olds = DiscussionParticipant::query_builder()
            .discussion_id_equals(discussion.id)
            .user_id_equals(user_id)
            .query()
            .map(DiscussionParticipant::from)
            .fetch_all(&self.pool)
            .await?;
        for p in olds {
            let _ = self.participation_repo.delete(p.id).await;
        }

        let _ = pr
            .insert(id, user_id, attendee_res.attendee_id)
            .await
            .map_err(|e| {
                tracing::error!("insert participant failed: {:?}", e);
                Error::DiscussionCreateUserFailed(e.to_string())
            })?;

        let latest = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_one(&self.pool)
            .await?;
        Ok(latest)
    }

    async fn exit_meeting(&self, id: i64, auth: Option<Authorization>) -> Result<Discussion> {
        let user = extract_user_with_options(&self.pool, auth, false).await?;
        let user_id = user.id;

        tracing::debug!("exit meeting user id: {:?}", user_id);

        if user_id == 0 {
            return Err(Error::InvalidUser);
        }

        let discussion = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(Error::DiscussionNotFound)?;

        if discussion.meeting_id.is_none() {
            return Err(Error::AwsChimeError("Not Found Meeting ID".to_string()));
        }

        let participants = DiscussionParticipant::query_builder()
            .discussion_id_equals(discussion.id)
            .user_id_equals(user_id)
            .query()
            .map(DiscussionParticipant::from)
            .fetch_all(&self.pool)
            .await?;

        for participant in participants {
            let _ = self.participation_repo.delete(participant.id).await?;
        }

        Ok(discussion)
    }

    async fn start_recording(&self, id: i64, _auth: Option<Authorization>) -> Result<Discussion> {
        let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;

        let discussion = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(Error::DiscussionNotFound)?;

        if discussion.meeting_id.is_none() {
            return Err(Error::DiscussionNotFound);
        }

        let meeting_id = discussion.meeting_id.unwrap();

        let m = client.get_meeting_info(&meeting_id).await;

        let meeting = if m.is_some() {
            m.unwrap()
        } else {
            let v = match client.create_meeting(&discussion.name).await {
                Ok(v) => Ok(v),
                Err(e) => {
                    tracing::error!("create meeting failed with error: {:?}", e);
                    Err(Error::AwsChimeError(e.to_string()))
                }
            }?;

            v
        };

        let (pipeline_id, pipeline_arn) =
            match client.make_pipeline(meeting.clone(), discussion.name).await {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("failed to create pipeline: {:?}", e);
                    return Err(Error::AwsChimeError(e.to_string()));
                }
            };

        let discussion = match self
            .repo
            .update(
                id,
                DiscussionRepositoryUpdateRequest {
                    pipeline_id: Some(pipeline_id),
                    media_pipeline_arn: Some(pipeline_arn),
                    meeting_id: Some(meeting.meeting_id().unwrap().to_string()),
                    ..Default::default()
                },
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("start recording {}", e);
                return Err(Error::UpdateDiscussionError(e.to_string()));
            }
        };
        Ok(discussion)
    }

    async fn end_recording(&self, id: i64, _auth: Option<Authorization>) -> Result<Discussion> {
        let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;

        let discussion = Discussion::query_builder()
            .id_equals(id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(Error::DiscussionNotFound)?;

        if discussion.meeting_id.is_none() {
            return Err(Error::DiscussionNotFound);
        }

        if discussion.pipeline_id == "" {
            return Err(Error::PipelineNotFound);
        }

        let _ = client
            .end_pipeline(
                &discussion.pipeline_id,
                &discussion.clone().meeting_id.unwrap_or_default(),
            )
            .await?;

        //FIXME: store s3 mp4 file to db

        Ok(discussion)
    }
}

impl SpaceDiscussionController {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Discussion::get_repository(pool.clone());
        let member_repo = DiscussionMember::get_repository(pool.clone());
        let participation_repo = DiscussionParticipant::get_repository(pool.clone());

        Self {
            repo,
            participation_repo,
            member_repo,
            pool,
        }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_discussion_by_id).post(Self::act_discussion_by_id),
            )
            .with_state(self.clone())
            .route("/", post(Self::act_discussion).get(Self::get_discussion))
            .with_state(self.clone())
    }

    pub async fn act_discussion_by_id(
        State(ctrl): State<SpaceDiscussionController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SpaceDiscussionPath { space_id, id }): Path<SpaceDiscussionPath>,
        Json(body): Json<DiscussionByIdAction>,
    ) -> Result<Json<Discussion>> {
        tracing::debug!("act_discussion_by_id {} {:?} {:?}", space_id, id, body);

        match body {
            DiscussionByIdAction::Update(param) => {
                let res = ctrl.update(space_id, id, auth, param).await?;
                Ok(Json(res))
            }
            DiscussionByIdAction::Delete(_) => {
                let res = ctrl.delete(space_id, id, auth).await?;
                Ok(Json(res))
            }

            DiscussionByIdAction::StartMeeting(_) => {
                let res = ctrl.start_meeting(id, auth).await?;
                Ok(Json(res))
            }

            DiscussionByIdAction::ParticipantMeeting(_) => {
                let res = ctrl.participant_meeting(id, auth).await?;
                Ok(Json(res))
            }

            DiscussionByIdAction::ExitMeeting(_) => {
                let res = ctrl.exit_meeting(id, auth).await?;
                Ok(Json(res))
            }

            DiscussionByIdAction::StartRecording(_) => {
                let res = ctrl.start_recording(id, auth).await?;
                Ok(Json(res))
            }

            DiscussionByIdAction::EndRecording(_) => {
                let res = ctrl.end_recording(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_discussion_by_id(
        State(ctrl): State<SpaceDiscussionController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(SpaceDiscussionPath { space_id, id }): Path<SpaceDiscussionPath>,
    ) -> Result<Json<Discussion>> {
        tracing::debug!("get_discussion {} {:?}", space_id, id);

        Ok(Json(
            Discussion::query_builder()
                .id_equals(id)
                .space_id_equals(space_id)
                .query()
                .map(Discussion::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_discussion(
        State(ctrl): State<SpaceDiscussionController>,
        Path(SpaceDiscussionParentPath { space_id }): Path<SpaceDiscussionParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DiscussionParam>,
    ) -> Result<Json<DiscussionGetResponse>> {
        tracing::debug!("list_discussion {} {:?}", space_id, q);

        match q {
            DiscussionParam::Query(param) => Ok(Json(DiscussionGetResponse::Query(
                ctrl.query(space_id, auth, param).await?,
            ))),
        }
    }

    pub async fn act_discussion(
        State(ctrl): State<SpaceDiscussionController>,
        Path(SpaceDiscussionParentPath { space_id }): Path<SpaceDiscussionParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<DiscussionAction>,
    ) -> Result<Json<Discussion>> {
        tracing::debug!("act_discussion {} {:?}", space_id, body);
        match body {
            DiscussionAction::Create(param) => {
                let res = ctrl.create(space_id, auth, param).await?;
                Ok(Json(res))
            }
        }
    }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceDiscussionPath {
    pub space_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceDiscussionParentPath {
    pub space_id: i64,
}
