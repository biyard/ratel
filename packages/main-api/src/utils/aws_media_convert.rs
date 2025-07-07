#![allow(unused)]
use dto::*;

pub async fn merge_recording_chunks(
    meeting_id: &str,
    media_pipeline_arn: String,
    record: Option<String>,
) -> Option<String> {
    use aws_config::{BehaviorVersion, Region, defaults};
    use aws_sdk_chimesdkmediapipelines::Client as ChimePipelinesClient;
    use aws_sdk_chimesdkmediapipelines::types::{
        ArtifactsConcatenationConfiguration, ArtifactsConcatenationState,
        AudioArtifactsConcatenationState, AudioConcatenationConfiguration,
        ChimeSdkMeetingConcatenationConfiguration, CompositedVideoConcatenationConfiguration,
        ConcatenationSink, ConcatenationSinkType, ConcatenationSource, ConcatenationSourceType,
        ContentConcatenationConfiguration, DataChannelConcatenationConfiguration,
        MediaCapturePipelineSourceConfiguration, MeetingEventsConcatenationConfiguration,
        S3BucketSinkConfiguration, TranscriptionMessagesConcatenationConfiguration,
        VideoConcatenationConfiguration,
    };
    use aws_sdk_s3::config::Credentials;

    let config = crate::config::get();

    let aws_config = defaults(BehaviorVersion::latest())
        .region(Region::new(config.aws.region))
        .credentials_provider(Credentials::new(
            config.aws.access_key_id,
            config.aws.secret_access_key,
            None,
            None,
            "credential",
        ))
        .load()
        .await;

    let bucket_name = config.chime_bucket_name.to_string();
    let destination_arn = format!("arn:aws:s3:::{}", bucket_name);

    if let Some(record) = record {
        if record.contains("video") {
            return Some(record);
        }

        let trimmed = record
            .trim_start_matches("https://")
            .trim_start_matches("http://");

        let parts: Vec<&str> = trimmed.split('/').collect();

        tracing::debug!("record parts: {:?}", parts);

        let media_pipeline_id = match parts.get(1) {
            Some(id) => *id,
            None => {
                tracing::warn!("Invalid record format: {}", record);
                return None;
            }
        };

        let prefix = format!("{}/video/", media_pipeline_id);

        let s3 = aws_sdk_s3::Client::new(&aws_config);
        let resp = s3
            .list_objects_v2()
            .bucket(&bucket_name)
            .prefix(&prefix)
            .send()
            .await
            .unwrap();

        let contents = resp.contents();

        if let Some(object) = contents
            .iter()
            .find(|obj| obj.key().map(|k| k.ends_with(".mp4")).unwrap_or(false))
        {
            let file_key = object.key().unwrap();

            let video_url = format!("https://{}/{}", bucket_name, file_key);

            return Some(video_url);
        }

        tracing::warn!("No mp4 file found in {}/video/", media_pipeline_id);
        return None;
    }

    let client = ChimePipelinesClient::new(&aws_config);

    let artifacts_config = ArtifactsConcatenationConfiguration::builder()
        .audio(
            AudioConcatenationConfiguration::builder()
                .state(AudioArtifactsConcatenationState::Enabled)
                .build()
                .unwrap(),
        )
        .video(
            VideoConcatenationConfiguration::builder()
                .state(ArtifactsConcatenationState::Enabled)
                .build()
                .unwrap(),
        )
        .content(
            ContentConcatenationConfiguration::builder()
                .state(ArtifactsConcatenationState::Enabled)
                .build()
                .unwrap(),
        )
        .composited_video(
            CompositedVideoConcatenationConfiguration::builder()
                .state(ArtifactsConcatenationState::Enabled)
                .build()
                .unwrap(),
        )
        .data_channel(
            DataChannelConcatenationConfiguration::builder()
                .state(ArtifactsConcatenationState::Disabled)
                .build()
                .unwrap(),
        )
        .transcription_messages(
            TranscriptionMessagesConcatenationConfiguration::builder()
                .state(ArtifactsConcatenationState::Disabled)
                .build()
                .unwrap(),
        )
        .meeting_events(
            MeetingEventsConcatenationConfiguration::builder()
                .state(ArtifactsConcatenationState::Disabled)
                .build()
                .unwrap(),
        )
        .build();

    let chime_config = ChimeSdkMeetingConcatenationConfiguration::builder()
        .artifacts_configuration(artifacts_config)
        .build();

    let source_config = MediaCapturePipelineSourceConfiguration::builder()
        .media_pipeline_arn(media_pipeline_arn)
        .chime_sdk_meeting_configuration(chime_config)
        .build()
        .unwrap();

    let request = client
        .create_media_concatenation_pipeline()
        .sources(
            ConcatenationSource::builder()
                .r#type(ConcatenationSourceType::MediaCapturePipeline)
                .media_capture_pipeline_source_configuration(source_config)
                .build()
                .unwrap(),
        )
        .sinks(
            ConcatenationSink::builder()
                .r#type(ConcatenationSinkType::S3Bucket)
                .s3_bucket_sink_configuration(
                    S3BucketSinkConfiguration::builder()
                        .destination(destination_arn.clone())
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );

    match request.send().await {
        Ok(resp) => {
            tracing::info!("Concatenation pipeline started: {:?}", resp);

            if resp.media_concatenation_pipeline.is_none() {
                return None;
            }

            match resp.media_concatenation_pipeline.unwrap().media_pipeline_id {
                Some(v) => Some(format!("https://{}/{}", bucket_name, v)),
                None => None,
            }
        }
        Err(err) => {
            tracing::error!("Failed to start concatenation pipeline: {:?}", err);
            return None;
        }
    }
}
