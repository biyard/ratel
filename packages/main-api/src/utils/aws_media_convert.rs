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

            if let (Some(video_obj), Some(audio_obj)) = (
                contents
                    .iter()
                    .find(|obj| obj.key().map(|k| k.ends_with(".mp4")).unwrap_or(false)),
                contents_audio
                    .iter()
                    .find(|obj| obj.key().map(|k| k.ends_with(".mp4")).unwrap_or(false)),
            ) {
                use aws_sdk_s3::primitives::ByteStream;
                use std::fs::File;
                use std::io::Write;
                use tokio::process::Command;

                let video_key = video_obj.key().unwrap();
                let audio_key = audio_obj.key().unwrap();
                let media_output_key =
                    format!("{}/{}/video/merged.mp4", config.env, media_pipeline_id);

                let local_video_path = format!("/tmp/{}_video.mp4", media_pipeline_id);
                let local_audio_path = format!("/tmp/{}_audio.mp4", media_pipeline_id);
                let local_merged_path = format!("/tmp/{}_merged.mp4", media_pipeline_id);

                let video_bytes = match s3
                    .get_object()
                    .bucket(&bucket_name)
                    .key(video_key)
                    .send()
                    .await
                {
                    Ok(resp) => match resp.body.collect().await {
                        Ok(collected) => collected.into_bytes(),
                        Err(err) => {
                            tracing::error!("Failed to collect video body: {:?}", err);
                            return None;
                        }
                    },
                    Err(err) => {
                        tracing::error!("Failed to download video object: {:?}", err);
                        return None;
                    }
                };

                let audio_bytes = match s3
                    .get_object()
                    .bucket(&bucket_name)
                    .key(audio_key)
                    .send()
                    .await
                {
                    Ok(resp) => match resp.body.collect().await {
                        Ok(collected) => collected.into_bytes(),
                        Err(err) => {
                            tracing::error!("Failed to collect audio body: {:?}", err);
                            return None;
                        }
                    },
                    Err(err) => {
                        tracing::error!("Failed to download audio object: {:?}", err);
                        return None;
                    }
                };

                if let Err(err) =
                    File::create(&local_video_path).and_then(|mut f| f.write_all(&video_bytes))
                {
                    tracing::error!("Failed to write video file: {:?}", err);
                    return None;
                }

                if let Err(err) =
                    File::create(&local_audio_path).and_then(|mut f| f.write_all(&audio_bytes))
                {
                    tracing::error!("Failed to write audio file: {:?}", err);
                    return None;
                }

                let ffmpeg_status = match Command::new("ffmpeg")
                    .args(&[
                        "-y",
                        "-i",
                        &local_video_path,
                        "-ss",
                        "10",
                        "-i",
                        &local_audio_path,
                        "-map",
                        "0:v:0",
                        "-map",
                        "1:a:0",
                        "-c:v",
                        "libx264",
                        "-c:a",
                        "aac",
                        "-preset",
                        "fast",
                        "-crf",
                        "23",
                        "-shortest",
                        &local_merged_path,
                    ])
                    .status()
                    .await
                {
                    Ok(status) => status,
                    Err(err) => {
                        tracing::error!("Failed to spawn ffmpeg process: {:?}", err);
                        return None;
                    }
                };
                if !ffmpeg_status.success() {
                    tracing::error!("ffmpeg process exited with failure");
                    return None;
                }

                let merged_bytes = match tokio::fs::read(&local_merged_path).await {
                    Ok(bytes) => bytes,
                    Err(err) => {
                        tracing::error!("Failed to read merged file from disk: {:?}", err);
                        return None;
                    }
                };
                s3.put_object()
                    .bucket(&bucket_name)
                    .key(&media_output_key)
                    .body(ByteStream::from(merged_bytes))
                    .send()
                    .await
                    .map_err(|e| {
                        tracing::error!("Upload merged.mp4 failed: {:?}", e);
                        e
                    })
                    .ok()?;

                let new_url = format!("https://{}/{}", bucket_name, media_output_key);
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
