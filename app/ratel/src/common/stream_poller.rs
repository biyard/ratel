/// Local-dev DynamoDB Stream poller.
///
/// Spawns a background tokio task that polls the DynamoDB Stream for the main
/// table and dispatches records to `stream_handler::handle_stream_record`.
/// This replaces the EventBridge Pipes + Lambda architecture for local testing.
///
/// Gated behind `#[cfg(feature = "local-dev")]`.
#[cfg(all(feature = "server", feature = "local-dev"))]
pub fn spawn_stream_poller() {
    // Spawn on a dedicated OS thread with its own Tokio runtime.
    // `dioxus::serve()` hasn't started the reactor yet when this is called,
    // so `tokio::spawn()` would panic with "no reactor running".
    std::thread::Builder::new()
        .name("stream-poller".into())
        .spawn(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("stream-poller runtime");
            rt.block_on(poll_loop());
        })
        .expect("failed to spawn stream-poller thread");
}

#[cfg(all(feature = "server", feature = "local-dev"))]
async fn poll_loop() {
    use aws_sdk_dynamodbstreams::Client as StreamsClient;
    use std::collections::HashMap;

    let cfg = crate::common::CommonConfig::default();
    let dynamodb = cfg.dynamodb();

    let table_name = {
        let prefix = option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("ratel-local");
        format!("{}-main", prefix)
    };

    let stream_arn = match get_stream_arn(dynamodb, &table_name).await {
        Some(arn) => arn,
        None => {
            tracing::warn!(table = %table_name, "No DynamoDB stream found, stream poller disabled");
            return;
        }
    };

    tracing::info!(stream_arn = %stream_arn, "DynamoDB stream poller started");

    let aws_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let streams_client = StreamsClient::new(&aws_config);
    let mut shard_iterators: HashMap<String, String> = HashMap::new();

    loop {
        // Discover shards
        match streams_client.describe_stream().stream_arn(&stream_arn).send().await {
            Ok(output) => {
                if let Some(desc) = output.stream_description() {
                    for shard in desc.shards() {
                        let shard_id = shard.shard_id().unwrap_or_default().to_string();
                        if shard_iterators.contains_key(&shard_id) {
                            continue;
                        }
                        if let Ok(iter_output) = streams_client
                            .get_shard_iterator()
                            .stream_arn(&stream_arn)
                            .shard_id(&shard_id)
                            .shard_iterator_type(aws_sdk_dynamodbstreams::types::ShardIteratorType::Latest)
                            .send()
                            .await
                        {
                            if let Some(iterator) = iter_output.shard_iterator() {
                                shard_iterators.insert(shard_id.clone(), iterator.to_string());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!(error = %e, "Failed to describe stream");
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        }

        // Poll each shard
        let mut next_iterators: HashMap<String, String> = HashMap::new();
        for (shard_id, iterator) in &shard_iterators {
            match streams_client.get_records().shard_iterator(iterator).limit(100).send().await {
                Ok(output) => {
                    for record in output.records() {
                        let event_name = record.event_name().map(|e| e.as_str()).unwrap_or("UNKNOWN");
                        let new_image = record.dynamodb().and_then(|d| {
                            d.new_image().map(|img| {
                                img.iter()
                                    .map(|(k, v)| (k.clone(), convert_av(v)))
                                    .collect::<HashMap<String, serde_dynamo::AttributeValue>>()
                            })
                        });
                        let old_image = record.dynamodb().and_then(|d| {
                            d.old_image().map(|img| {
                                img.iter()
                                    .map(|(k, v)| (k.clone(), convert_av(v)))
                                    .collect::<HashMap<String, serde_dynamo::AttributeValue>>()
                            })
                        });
                        if let Err(e) = crate::common::stream_handler::handle_stream_record(
                            event_name,
                            new_image.as_ref(),
                            old_image.as_ref(),
                        )
                        .await
                        {
                            tracing::error!(event_name = %event_name, error = %e, "Stream record handler failed");
                        }
                    }
                    if let Some(next) = output.next_shard_iterator() {
                        next_iterators.insert(shard_id.clone(), next.to_string());
                    }
                }
                Err(e) => {
                    tracing::warn!(shard_id = %shard_id, error = %e, "Failed to get records, will refresh shard iterator");
                }
            }
        }

        shard_iterators = next_iterators;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

#[cfg(all(feature = "server", feature = "local-dev"))]
async fn get_stream_arn(dynamodb: &aws_sdk_dynamodb::Client, table_name: &str) -> Option<String> {
    dynamodb
        .describe_table()
        .table_name(table_name)
        .send()
        .await
        .ok()
        .and_then(|output| output.table)
        .and_then(|table| table.latest_stream_arn)
}

/// Convert DynamoDB Streams AttributeValue to serde_dynamo AttributeValue.
///
/// The Streams SDK returns `aws_sdk_dynamodbstreams::types::AttributeValue`
/// while serde_dynamo expects `serde_dynamo::AttributeValue`. They are
/// wire-compatible via JSON, so we serialize + deserialize.
#[cfg(all(feature = "server", feature = "local-dev"))]
fn convert_av(av: &aws_sdk_dynamodbstreams::types::AttributeValue) -> serde_dynamo::AttributeValue {
    // Quick match for common types to avoid JSON round-trip overhead
    match av {
        aws_sdk_dynamodbstreams::types::AttributeValue::S(s) => {
            serde_dynamo::AttributeValue::S(s.clone())
        }
        aws_sdk_dynamodbstreams::types::AttributeValue::N(n) => {
            serde_dynamo::AttributeValue::N(n.clone())
        }
        aws_sdk_dynamodbstreams::types::AttributeValue::Bool(b) => {
            serde_dynamo::AttributeValue::Bool(*b)
        }
        aws_sdk_dynamodbstreams::types::AttributeValue::Null(b) => {
            serde_dynamo::AttributeValue::Null(*b)
        }
        aws_sdk_dynamodbstreams::types::AttributeValue::M(m) => serde_dynamo::AttributeValue::M(
            m.iter().map(|(k, v)| (k.clone(), convert_av(v))).collect(),
        ),
        aws_sdk_dynamodbstreams::types::AttributeValue::L(l) => {
            serde_dynamo::AttributeValue::L(l.iter().map(convert_av).collect())
        }
        aws_sdk_dynamodbstreams::types::AttributeValue::Ss(ss) => {
            serde_dynamo::AttributeValue::Ss(ss.clone())
        }
        aws_sdk_dynamodbstreams::types::AttributeValue::Ns(ns) => {
            serde_dynamo::AttributeValue::Ns(ns.clone())
        }
        _ => serde_dynamo::AttributeValue::Null(true),
    }
}
