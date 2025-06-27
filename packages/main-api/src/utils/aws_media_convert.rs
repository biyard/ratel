#![allow(unused)]
use dto::*;

pub async fn merge_recording_chunks(
    meeting_id: &str,
    media_pipeline_arn: String,
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
            return Some(format!("s3://{}/concatenated/{}/", bucket_name, meeting_id));
        }
        Err(err) => {
            tracing::error!("Failed to start concatenation pipeline: {:?}", err);
            return None;
        }
    }
}
