use aws_sdk_dynamodb::types::AttributeValue;
use dto::{Error, Result};
use std::collections::HashMap;
use tracing::{error, info, warn};

use crate::utils::aws::DynamoClient;

pub struct BatchProcessor {
    client: DynamoClient,
    table_name: String,
    batch_size: usize,
    max_retries: usize,
}

impl BatchProcessor {
    pub fn new(table_name: &str) -> Self {
        Self {
            client: DynamoClient::new(None),
            table_name: table_name.to_string(),
            batch_size: 25, // DynamoDB batch write limit
            max_retries: 3,
        }
    }

    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        // Ensure batch size doesn't exceed DynamoDB limit
        self.batch_size = batch_size.min(25);
        self
    }

    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub async fn process_batch(&self, items: Vec<HashMap<String, AttributeValue>>) -> Result<()> {
        if items.is_empty() {
            return Ok(());
        }

        info!("Processing batch of {} items", items.len());

        // Split into smaller batches if needed
        let chunks: Vec<_> = items.chunks(self.batch_size).collect();

        for (i, chunk) in chunks.iter().enumerate() {
            info!(
                "Processing chunk {} of {} ({} items)",
                i + 1,
                chunks.len(),
                chunk.len()
            );

            let mut retry_count = 0;
            let mut current_chunk = chunk.to_vec();

            while retry_count <= self.max_retries {
                match self.write_batch_with_retry(&current_chunk).await {
                    Ok(unprocessed_items) => {
                        if unprocessed_items.is_empty() {
                            info!("Chunk {} completed successfully", i + 1);
                            break;
                        } else {
                            warn!(
                                "Chunk {} had {} unprocessed items, retrying...",
                                i + 1,
                                unprocessed_items.len()
                            );
                            current_chunk = unprocessed_items;
                            retry_count += 1;

                            // Exponential backoff
                            let delay = std::time::Duration::from_millis(
                                1000 * (2_u64.pow(retry_count as u32)),
                            );
                            tokio::time::sleep(delay).await;
                        }
                    }
                    Err(e) => {
                        error!("Error processing chunk {}: {:?}", i + 1, e);
                        retry_count += 1;

                        if retry_count > self.max_retries {
                            return Err(e);
                        }

                        // Exponential backoff
                        let delay = std::time::Duration::from_millis(
                            1000 * (2_u64.pow(retry_count as u32)),
                        );
                        tokio::time::sleep(delay).await;
                    }
                }
            }

            if retry_count > self.max_retries {
                return Err(Error::DynamoDbError(format!(
                    "Failed to process chunk {} after {} retries",
                    i + 1,
                    self.max_retries
                )));
            }
        }

        info!("Batch processing completed successfully");
        Ok(())
    }

    async fn write_batch_with_retry(
        &self,
        items: &[HashMap<String, AttributeValue>],
    ) -> Result<Vec<HashMap<String, AttributeValue>>> {
        use aws_sdk_dynamodb::types::{PutRequest, WriteRequest};

        let mut write_requests = Vec::new();
        for item in items {
            let put_request = PutRequest::builder()
                .set_item(Some(item.clone()))
                .build()
                .map_err(|e| {
                    Error::DynamoDbError(format!("Failed to build put request: {:?}", e))
                })?;

            let write_request = WriteRequest::builder().put_request(put_request).build();
            write_requests.push(write_request);
        }

        let table_name = self.table_name.clone();
        let mut request_items = HashMap::new();
        request_items.insert(table_name.clone(), write_requests);

        match self
            .client
            .client
            .batch_write_item()
            .set_request_items(Some(request_items))
            .send()
            .await
        {
            Ok(output) => {
                let unprocessed_items = output
                    .unprocessed_items()
                    .and_then(|items| items.get(&table_name))
                    .map(|requests| {
                        requests
                            .iter()
                            .filter_map(|req| req.put_request.as_ref().map(|put| put.item.clone()))
                            .collect()
                    })
                    .unwrap_or_default();

                Ok(unprocessed_items)
            }
            Err(e) => Err(Error::DynamoDbError(format!("Batch write failed: {:?}", e))),
        }
    }

    pub async fn process_items_in_batches<T, F>(&self, items: Vec<T>, converter: F) -> Result<()>
    where
        F: Fn(&T) -> Result<HashMap<String, AttributeValue>>,
    {
        let mut batch = Vec::new();
        let mut processed = 0;

        for item in items.iter() {
            match converter(item) {
                Ok(dynamo_item) => {
                    batch.push(dynamo_item);

                    if batch.len() >= self.batch_size {
                        let batch_len = batch.len();
                        self.process_batch(batch.clone()).await?;
                        processed += batch_len;
                        batch.clear();

                        info!("Processed {} items so far", processed);
                    }
                }
                Err(e) => {
                    error!("Failed to convert item: {:?}", e);
                    return Err(e);
                }
            }
        }

        // Process remaining items
        if !batch.is_empty() {
            let batch_len = batch.len();
            self.process_batch(batch).await?;
            processed += batch_len;
        }

        info!("Total items processed: {}", processed);
        Ok(())
    }

    // Helper method for processing with progress callback
    pub async fn process_with_progress<T, F, P>(
        &self,
        items: Vec<T>,
        converter: F,
        mut progress_callback: P,
    ) -> Result<()>
    where
        F: Fn(&T) -> Result<HashMap<String, AttributeValue>>,
        P: FnMut(usize, usize),
    {
        let total_items = items.len();
        let mut batch = Vec::new();
        let mut processed = 0;

        for (index, item) in items.iter().enumerate() {
            match converter(item) {
                Ok(dynamo_item) => {
                    batch.push(dynamo_item);

                    if batch.len() >= self.batch_size || index == total_items - 1 {
                        self.process_batch(batch.clone()).await?;
                        processed += batch.len();
                        batch.clear();

                        progress_callback(processed, total_items);
                    }
                }
                Err(e) => {
                    error!("Failed to convert item at index {}: {:?}", index, e);
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
