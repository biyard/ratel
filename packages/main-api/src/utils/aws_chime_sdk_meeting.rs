use crate::{models::folder_type::folder_type::FolderType, types::meeting_info::MeetingInfo, *};
use aws_config::{BehaviorVersion, load_defaults};
use aws_sdk_chimesdkmediapipelines::{
    Client as MediaPipelinesClient,
    types::{MediaPipelineSinkType, MediaPipelineSourceType},
};
use aws_sdk_chimesdkmeetings::{
    Client as MeetingsClient,
    types::{Attendee, Meeting},
};
use aws_sdk_s3::Client as S3Client;
use tokio::time::{Duration, sleep};

#[derive(Debug)]
pub struct AttendeeInfo {
    pub attendee_id: String,
    pub external_user_id: String,
    pub join_token: String,
}

pub struct ChimeMeetingService {
    client: MeetingsClient,
    pipeline: MediaPipelinesClient,
    s3: S3Client,
}

impl ChimeMeetingService {
    pub async fn new() -> Self {
        let config = load_defaults(BehaviorVersion::latest()).await;
        let client = MeetingsClient::new(&config);
        let pipeline = MediaPipelinesClient::new(&config);
        let s3 = S3Client::new(&config);
        Self {
            client,
            pipeline,
            s3,
        }
    }

    pub async fn get_meeting_info(&self, meeting_id: &str) -> Option<Meeting> {
        let meeting = match self
            .client
            .get_meeting()
            .meeting_id(meeting_id)
            .send()
            .await
        {
            Ok(v) => Some(v.meeting.unwrap()),
            Err(e) => {
                tracing::error!("get_meeting error: {:?}", e);
                None
            }
        };

        meeting
    }

    pub async fn get_attendee_info(&self, meeting_id: &str, attendee_id: &str) -> Option<Attendee> {
        let attendee = match self
            .client
            .get_attendee()
            .meeting_id(meeting_id)
            .attendee_id(attendee_id)
            .send()
            .await
        {
            Ok(v) => Some(v.attendee.unwrap()),
            Err(e) => {
                tracing::error!("get_attendee error: {:?}", e);
                None
            }
        };

        attendee
    }

    pub async fn create_meeting(&self, meeting_name: &str) -> Result<Meeting> {
        let _ = meeting_name;
        let client_request_token = uuid::Uuid::new_v4().to_string();
        let conf = crate::config::get();

        let resp = match self
            .client
            .create_meeting()
            .client_request_token(client_request_token.clone())
            .external_meeting_id(client_request_token.clone())
            .media_region(conf.aws.region)
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("create_meeting error: {:?}", e);
                return Err(Error::AwsChimeError(e.to_string()));
            }
        };

        let meeting = match resp.meeting {
            Some(v) => v,
            None => {
                tracing::error!("create_meeting error: no meeting");
                return Err(Error::AwsChimeError("no meeting".to_string()));
            }
        };

        Ok(meeting)
    }

    pub async fn create_attendee(
        &self,
        meeting: &MeetingInfo,
        external_user_id: &str,
    ) -> Result<AttendeeInfo> {
        let user_id = format!("u-{:?}", external_user_id);
        let external_user_id = user_id.as_str();

        let resp = match self
            .client
            .create_attendee()
            .external_user_id(external_user_id)
            .meeting_id(meeting.meeting_id.clone())
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("create_attendee error: {:?}", e);
                return Err(Error::AwsChimeError(e.to_string()));
            }
        };

        let attendee = match resp.attendee {
            Some(v) => v,
            None => {
                tracing::error!("create_attendee error: no attendee");
                return Err(Error::AwsChimeError("no attendee".to_string()));
            }
        };

        Ok(AttendeeInfo {
            attendee_id: attendee.attendee_id.unwrap_or_default(),
            external_user_id: attendee.external_user_id.unwrap_or_default(),
            join_token: attendee.join_token.unwrap_or_default(),
        })
    }

    pub async fn end_meeting(&self, meeting: &Meeting) -> Result<()> {
        let resp = match self
            .client
            .delete_meeting()
            .meeting_id(meeting.meeting_id.clone().unwrap_or_default())
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("delete_meeting error: {:?}", e);
                return Err(Error::AwsChimeError(e.to_string()));
            }
        };

        tracing::debug!("delete_meeting response: {:?}", resp);

        Ok(())
    }

    pub async fn make_pipeline(
        &self,
        meeting: Meeting,
        _meeting_name: String,
    ) -> Result<(String, String)> {
        let bucket_name = crate::config::get().chime_bucket_name.to_string();

        let client_request_token = uuid::Uuid::new_v4().to_string();

        let artifacts_config =
            aws_sdk_chimesdkmediapipelines::types::ArtifactsConfiguration::builder()
                .audio(
                    aws_sdk_chimesdkmediapipelines::types::AudioArtifactsConfiguration::builder()
                        .mux_type(aws_sdk_chimesdkmediapipelines::types::AudioMuxType::AudioOnly)
                        .build()
                        .map_err(|e| {
                            tracing::error!("audio artifacts configuration error: {:?}", e);
                            Error::AwsMediaPipelinesError(e.to_string())
                        })?,
                )
                .video(
                    aws_sdk_chimesdkmediapipelines::types::VideoArtifactsConfiguration::builder()
                        .state(aws_sdk_chimesdkmediapipelines::types::ArtifactsState::Enabled)
                        .mux_type(aws_sdk_chimesdkmediapipelines::types::VideoMuxType::VideoOnly)
                        .build()
                        .map_err(|e| {
                            tracing::error!("video artifacts configuration error: {:?}", e);
                            Error::AwsMediaPipelinesError(e.to_string())
                        })?,
                )
                .content(
                    aws_sdk_chimesdkmediapipelines::types::ContentArtifactsConfiguration::builder()
                        .state(aws_sdk_chimesdkmediapipelines::types::ArtifactsState::Enabled)
                        .mux_type(
                            aws_sdk_chimesdkmediapipelines::types::ContentMuxType::ContentOnly,
                        )
                        .build()
                        .map_err(|e| {
                            tracing::error!("content artifacts configuration error: {:?}", e);
                            Error::AwsMediaPipelinesError(e.to_string())
                        })?,
                )
                .build();

        let sink_configuration =
            aws_sdk_chimesdkmediapipelines::types::ChimeSdkMeetingConfiguration::builder()
                .artifacts_configuration(artifacts_config)
                .build();

        let resp = match self
            .pipeline
            .create_media_capture_pipeline()
            .client_request_token(client_request_token)
            .source_type(MediaPipelineSourceType::ChimeSdkMeeting)
            .source_arn(meeting.meeting_arn.unwrap_or_default())
            .sink_type(MediaPipelineSinkType::S3Bucket)
            .sink_arn(format!("arn:aws:s3:::{}", bucket_name))
            .chime_sdk_meeting_configuration(sink_configuration)
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("create_media_capture_pipeline error: {:?}", e);
                return Err(Error::AwsChimeError(e.to_string()));
            }
        };

        tracing::debug!("create_media_capture_pipeline response: {:?}", resp);

        let (pipeline_id, pipeline_arn) = resp
            .media_capture_pipeline
            .as_ref()
            .and_then(|p| Some((p.media_pipeline_id.clone(), p.media_pipeline_arn.clone())))
            .unwrap_or_default();

        Ok((
            pipeline_id.unwrap_or_default(),
            pipeline_arn.unwrap_or_default(),
        ))
    }

    pub async fn end_pipeline(&self, pipeline_id: &str, _meeting_id: &str) -> Result<()> {
        let _bucket_name = crate::config::get().chime_bucket_name.to_string();

        let resp = self
            .pipeline
            .delete_media_capture_pipeline()
            .media_pipeline_id(pipeline_id)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("delete_media_capture_pipeline error: {:?}", e);
                Error::AwsChimeError(e.to_string())
            })?;

        tracing::debug!("delete_media_capture_pipeline response: {:?}", resp);

        Ok(())
    }

    pub async fn move_meeting_artifacts_with_retry(
        &self,
        bucket_name: &str,
        meeting_id: &str,
    ) -> Result<()> {
        let prefix = format!("{}/", meeting_id);
        let mut attempt = 0;
        let max_attempts = 10;
        let delay = Duration::from_secs(2);

        loop {
            attempt += 1;

            let list_result = self
                .s3
                .list_objects_v2()
                .bucket(bucket_name)
                .prefix(&prefix)
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("S3 list_objects_v2 failed on attempt {}: {:?}", attempt, e);
                    Error::AwsS3Error(e.to_string())
                })?;

            if let Some(objects) = list_result.contents {
                if !objects.is_empty() {
                    tracing::info!("S3 artifacts found after {} attempt(s)", attempt);
                    return self.move_meeting_artifacts(bucket_name, meeting_id).await;
                }
            }

            if attempt >= max_attempts {
                tracing::warn!(
                    "No artifacts found after {} attempts (~{}s). Skipping move.",
                    attempt,
                    attempt * delay.as_secs()
                );
                return Ok(());
            }

            tracing::debug!(
                "Waiting for artifacts... attempt {}/{}. Retrying in {}s",
                attempt,
                max_attempts,
                delay.as_secs()
            );
            sleep(delay).await;
        }
    }

    pub async fn move_meeting_artifacts(&self, bucket_name: &str, meeting_id: &str) -> Result<()> {
        let conf = crate::config::get();

        let list_result = self
            .s3
            .list_objects_v2()
            .bucket(bucket_name)
            .prefix(format!("{}/", meeting_id))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("S3 list_objects error: {:?}", e);
                Error::AwsS3Error(e.to_string())
            })?;

        if let Some(objects) = list_result.contents {
            for obj in objects {
                if let Some(key) = obj.key {
                    let filename = key.split('/').last().unwrap_or("artifact");

                    let folder: FolderType = key.parse().unwrap_or(FolderType::Etc);

                    let destination_key =
                        format!("{}/{}/{}/{}", conf.env, meeting_id, folder, filename);

                    tracing::debug!("Copying from {} to {}", key, destination_key);

                    self.s3
                        .copy_object()
                        .copy_source(format!("{}/{}", bucket_name, key))
                        .bucket(bucket_name)
                        .key(&destination_key)
                        .send()
                        .await
                        .map_err(|e| {
                            tracing::error!("S3 copy error for {}: {:?}", key, e);
                            Error::AwsS3Error(e.to_string())
                        })?;

                    self.s3
                        .delete_object()
                        .bucket(bucket_name)
                        .key(&key)
                        .send()
                        .await
                        .map_err(|e| {
                            tracing::warn!("S3 delete error for {}: {:?}", key, e);
                            Error::AwsS3Error(e.to_string())
                        })?;
                }
            }
        }

        Ok(())
    }
}
