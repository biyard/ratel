use crate::DiscussionUser;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::get,
    },
};
use dto::*;

use crate::utils::aws_media_convert::merge_recording_chunks;
use crate::utils::users::extract_user_with_allowing_anonymous;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DiscussionPath {
    pub space_id: i64,
    pub discussion_id: i64,
}

#[derive(Clone, Debug)]
pub struct SpaceMeetingController {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: DiscussionRepository,
    participation_repo: DiscussionParticipantRepository,
}

impl SpaceMeetingController {
    async fn query(
        &self,
        _space_id: i64,
        auth: Option<Authorization>,
        discussion_id: i64,
    ) -> Result<MeetingData> {
        let client = crate::utils::aws_chime_sdk_meeting::ChimeMeetingService::new().await;
        let user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;
        let user_id = user.id;

        let discussion = Discussion::query_builder()
            .id_equals(discussion_id)
            .query()
            .map(Discussion::from)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(Error::DiscussionNotFound)?;

        let meeting_id = discussion.meeting_id.unwrap_or_default();
        let pipeline_arn = discussion.media_pipeline_arn.unwrap_or_default();

        let participant = DiscussionParticipant::query_builder()
            .discussion_id_equals(discussion.id)
            .user_id_equals(user_id)
            .query()
            .map(DiscussionParticipant::from)
            .fetch_optional(&self.pool)
            .await?;

        let attendee_id = participant.clone().unwrap_or_default().participant_id;
        let user_id = participant.clone().unwrap_or_default().user_id;

        // meeting checking and if meeting is expired, recreation
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

        let meeting_id = meeting.clone().meeting_id.unwrap_or_default();

        let mp = meeting
            .media_placement()
            .ok_or(Error::AwsChimeError("Missing media_placement".to_string()))?;

        let meeting_info = MeetingInfo {
            meeting_id: meeting_id.clone(),
            media_region: meeting.media_region.clone().unwrap_or_default(),
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

        // attendee checking and if get attendee is error, recreation.
        let v = client.get_attendee_info(&meeting_id, &attendee_id).await;

        let attendee = if v.is_some() {
            v.unwrap()
        } else {
            let v = match client
                .create_attendee(&meeting_info.clone(), user_id.to_string().as_str())
                .await
            {
                Ok(v) => Ok(v),
                Err(e) => {
                    tracing::error!("create meeting failed with error: {:?}", e);
                    Err(Error::AwsChimeError(e.to_string()))
                }
            }?;

            let v = client
                .get_attendee_info(meeting.clone().meeting_id().unwrap(), &v.attendee_id)
                .await
                .unwrap();

            let mut tx = self.pool.begin().await?;

            let _ = match self
                .repo
                .update_with_tx(
                    &mut *tx,
                    discussion_id,
                    DiscussionRepositoryUpdateRequest {
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

            let participants = DiscussionParticipant::query_builder()
                .discussion_id_equals(discussion.id)
                .user_id_equals(user_id)
                .query()
                .map(DiscussionParticipant::from)
                .fetch_all(&self.pool)
                .await?;

            for participant in participants {
                let _ = self
                    .participation_repo
                    .delete_with_tx(&mut *tx, participant.id)
                    .await;
            }

            match self
                .participation_repo
                .insert_with_tx(
                    &mut *tx,
                    discussion_id,
                    user_id,
                    v.clone().attendee_id.unwrap(),
                )
                .await
            {
                Ok(d) => d,
                Err(e) => {
                    tracing::error!("insert db failed after create participant {}", e);
                    return Err(Error::DiscussionCreateUserFailed(e.to_string()));
                }
            };

            tx.commit().await?;

            v
        };

        let attendee = AttendeeInfo {
            attendee_id,
            join_token: attendee.join_token.unwrap_or_default(),
            external_user_id: attendee.external_user_id.unwrap_or_default(),
        };

        // let record = merge_recording_chunks(&meeting_id).await;

        let discussion_participants = DiscussionParticipant::query_builder()
            .discussion_id_equals(discussion.id)
            .query()
            .map(DiscussionParticipant::from)
            .fetch_all(&self.pool)
            .await?;

        let user_ids: Vec<i64> = discussion_participants.iter().map(|p| p.user_id).collect();

        let participants = if user_ids.is_empty() {
            vec![]
        } else {
            let placeholders = (1..=user_ids.len())
                .map(|i| format!("${}", i))
                .collect::<Vec<_>>()
                .join(", ");
            let query = format!("SELECT * FROM users WHERE id IN ({})", placeholders);

            let mut q = sqlx::query(&query);

            for id in &user_ids {
                q = q.bind(id);
            }

            let rows = q.map(DiscussionUser::from).fetch_all(&self.pool).await?;

            rows
        };

        let record = merge_recording_chunks(&meeting_id, pipeline_arn).await;

        Ok(MeetingData {
            meeting: meeting_info,
            attendee,
            participants,
            record,
        })
    }
}

impl SpaceMeetingController {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Discussion::get_repository(pool.clone());
        let participation_repo = DiscussionParticipant::get_repository(pool.clone());
        Self {
            pool,
            repo,
            participation_repo,
        }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/:discussion-id", get(Self::get_meeting_by_id))
            .with_state(self.clone())
    }

    pub async fn get_meeting_by_id(
        State(ctrl): State<SpaceMeetingController>,
        Path(DiscussionPath {
            space_id,
            discussion_id,
        }): Path<DiscussionPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<MeetingDataParam>,
    ) -> Result<Json<MeetingDataGetResponse>> {
        tracing::debug!("get_meeting_by_id {:?}", q);

        match q {
            MeetingDataParam::Read(param)
                if param.action == Some(MeetingDataReadActionType::FindOne) =>
            {
                Ok(Json(MeetingDataGetResponse::Read(
                    ctrl.query(space_id, auth, discussion_id).await?,
                )))
            }
            _ => Err(Error::InvalidAction),
        }
    }
}
