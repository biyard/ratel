use crate::*;

pub async fn merge_recording_chunks(
    _meeting_id: &str,
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
    use aws_sdk_dynamodb::error::ProvideErrorMetadata;
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
        if !record.contains("video") {
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
            let prefix_audio = format!("{}/audio/", media_pipeline_id);

            let s3 = aws_sdk_s3::Client::new(&aws_config);
            let resp = s3
                .list_objects_v2()
                .bucket(&bucket_name)
                .prefix(&prefix)
                .send()
                .await
                .unwrap();
            let resp_audio = s3
                .list_objects_v2()
                .bucket(&bucket_name)
                .prefix(&prefix_audio)
                .send()
                .await
                .unwrap();

            let contents = resp.contents();
            let contents_audio = resp_audio.contents();

            if let Some(object) = contents
                .iter()
                .find(|obj| obj.key().map(|k| k.ends_with(".mp4")).unwrap_or(false))
            {
                let file_key = object.key().unwrap();
                let filename = file_key.split('/').last().unwrap_or("video.mp4");

                let new_key = format!("{}/{}/video/{}", config.env, media_pipeline_id, filename);

                s3.copy_object()
                    .copy_source(format!("{}/{}", bucket_name, file_key))
                    .bucket(&bucket_name)
                    .key(&new_key)
                    .send()
                    .await
                    .map_err(|e| {
                        tracing::error!("S3 copy error: {:?}", e);
                        e
                    })
                    .ok()?;

                if let Some(object) = contents_audio
                    .iter()
                    .find(|obj| obj.key().map(|k| k.ends_with(".mp4")).unwrap_or(false))
                {
                    let file_key = object.key().unwrap();
                    let filename = file_key.split('/').last().unwrap_or("audio.mp4");

                    let new_key =
                        format!("{}/{}/audio/{}", config.env, media_pipeline_id, filename);

                    s3.copy_object()
                        .copy_source(format!("{}/{}", bucket_name, file_key))
                        .bucket(&bucket_name)
                        .key(&new_key)
                        .send()
                        .await
                        .map_err(|e| {
                            tracing::error!("S3 copy error: {:?}", e);
                            e
                        })
                        .ok()?;
                }

                let new_url = format!("https://{}/{}", bucket_name, new_key);

                let cleanup_prefix = format!("{}/", media_pipeline_id);
                let list_resp = s3
                    .list_objects_v2()
                    .bucket(&bucket_name)
                    .prefix(&cleanup_prefix)
                    .send()
                    .await
                    .ok();

                if let Some(objects) = list_resp.and_then(|r| r.contents) {
                    for obj in objects {
                        if let Some(key) = obj.key {
                            let _ = s3
                                .delete_object()
                                .bucket(&bucket_name)
                                .key(&key)
                                .send()
                                .await;
                        }
                    }
                    tracing::info!(
                        "Cleaned up media_pipeline_id directory: {}",
                        media_pipeline_id
                    );
                } else {
                    tracing::error!(
                        "No objects found under media_pipeline_id prefix: {}",
                        cleanup_prefix
                    );
                }

                return Some(new_url);
            }

            tracing::error!("No mp4 file found in {}/video/", media_pipeline_id);
            return None;
        } else {
            return Some(record);
        }
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
        .media_pipeline_arn(&media_pipeline_arn)
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

    if media_pipeline_arn.trim().is_empty() {
        tracing::error!("media_pipeline_arn is empty, cannot start concatenation pipeline");
        return None;
    }

    match request.send().await {
        Ok(resp) => {
            tracing::info!("Concatenation pipeline started: {:?}", resp);
            resp.media_concatenation_pipeline
                .and_then(|p| p.media_pipeline_id)
                .map(|v| format!("https://{}/{}", bucket_name, v))
        }
        Err(err) => {
            let meta = err.meta();
            tracing::error!(
                "Concatenation pipeline failed: code={:?}, message={:?}, request_id={:?}",
                meta.code(),
                meta.message(),
                meta.extra("aws_request_id")
            );

            None
        }
    }
}
