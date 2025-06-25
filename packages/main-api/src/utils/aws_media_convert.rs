use dto::*;

pub async fn merge_recording_chunks(meeting_id: &str) -> Option<String> {
    use aws_config::BehaviorVersion;
    use aws_config::{Region, defaults};
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

    let s3_cli = aws_sdk_s3::Client::new(&aws_config);

    let bucket_name = config.chime_bucket_name.to_string();
    let meeting_prefix = format!("{}/video/", meeting_id);

    // Check if merged file already exists
    let merged_key = format!("merged/{}_merged.mp4", meeting_id);
    match s3_cli
        .head_object()
        .bucket(&bucket_name)
        .key(&merged_key)
        .send()
        .await
    {
        Ok(_) => {
            // File already exists, return its URI
            return Some(format!("s3://{}/{}", bucket_name, merged_key));
        }
        Err(_) => {
            // File doesn't exist, continue with merge process
        }
    }

    let list_objects_output = match s3_cli
        .list_objects_v2()
        .bucket(&bucket_name)
        .prefix(&meeting_prefix)
        .send()
        .await
        .map_err(|e| Error::AwsS3Error(e.to_string()))
    {
        Ok(output) => output,
        Err(e) => {
            tracing::error!("Failed to list objects in S3: {:?}", e);
            return None;
        }
    };

    let objects = list_objects_output.contents();

    if objects.is_empty() {
        tracing::error!("No objects found in S3 bucket");
        return None;
    }

    let mediaconvert_cli = aws_sdk_mediaconvert::Client::new(&aws_config);

    let mut input_files: Vec<_> = objects
        .iter()
        .filter(|obj| obj.key().unwrap_or_default().ends_with(".mp4"))
        .collect();

    input_files.sort_by_key(|obj| obj.key().unwrap_or_default().to_string());

    let mut job_settings_builder = aws_sdk_mediaconvert::types::JobSettings::builder();

    for obj in &input_files {
        job_settings_builder = job_settings_builder.inputs(
            aws_sdk_mediaconvert::types::Input::builder()
                .file_input(format!("s3://{}/{}", bucket_name, obj.key().unwrap()))
                .build(),
        );
    }

    let job_settings = job_settings_builder
        .output_groups(
            aws_sdk_mediaconvert::types::OutputGroup::builder()
                .output_group_settings(
                    aws_sdk_mediaconvert::types::OutputGroupSettings::builder()
                        .file_group_settings(
                            aws_sdk_mediaconvert::types::FileGroupSettings::builder()
                                .destination(format!(
                                    "s3://{}/merged/{}_merged.mp4",
                                    bucket_name, meeting_id
                                ))
                                .build(),
                        )
                        .build(),
                )
                .outputs(
                    aws_sdk_mediaconvert::types::Output::builder()
                        .container_settings(
                            aws_sdk_mediaconvert::types::ContainerSettings::builder()
                                .container(aws_sdk_mediaconvert::types::ContainerType::Mp4)
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .build();

    let _job = match mediaconvert_cli
        .create_job()
        // .role(config.aws.media_convert_role)
        .settings(job_settings)
        .send()
        .await
    {
        Ok(job) => job,
        Err(e) => {
            tracing::error!("Failed to create MediaConvert job: {:?}", e);
            return None;
        }
    };

    Some(format!(
        "s3://{}/merged/{}_merged.mp4",
        bucket_name, meeting_id
    ))
}
