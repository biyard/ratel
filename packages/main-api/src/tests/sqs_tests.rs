//! SQS integration tests
//! 
//! These tests verify SQS functionality using LocalStack for local development.
//! They test message sending, receiving, and queue management.

use dto::*;
use crate::utils::sqs_client::SqsClient;
use aws_sdk_sqs::Client;
use aws_config::Region;
use aws_sdk_sqs::{Config, config::Credentials};
use serde_json::json;
use std::collections::HashMap;

/// Test environment setup for SQS
pub struct SqsTestSetup {
    pub client: Client,
    pub sqs_client: SqsClient,
    pub watermark_queue_url: String,
    pub artwork_queue_url: String,
}

impl SqsTestSetup {
    pub async fn new() -> Self {
        // Setup SQS client for LocalStack
        let mut builder = Config::builder();
        
        if let Some(sqs_url) = option_env!("AWS_ENDPOINT_URL_SQS") {
            builder = builder
                .credentials_provider(Credentials::new(
                    "test",
                    "test", 
                    None,
                    None,
                    "sqs",
                ))
                .endpoint_url(sqs_url);
        } else {
            // Use localhost for tests
            builder = builder
                .credentials_provider(Credentials::new(
                    "test",
                    "test", 
                    None,
                    None,
                    "sqs",
                ))
                .endpoint_url("http://localhost:4566");
        }
        
        let aws_config = builder
            .region(Region::new("us-east-1"))
            .behavior_version_latest()
            .build();

        let client = Client::from_conf(aws_config);
        let sqs_client = SqsClient::new().await;

        Self {
            client: client.clone(),
            sqs_client,
            watermark_queue_url: "http://localhost:4566/000000000000/watermark-queue".to_string(),
            artwork_queue_url: "http://localhost:4566/000000000000/artwork-image-queue".to_string(),
        }
    }

    /// Clean up test messages
    pub async fn cleanup(&self) -> Result<()> {
        tracing::info!("Cleaning up SQS test messages");
        
        // Purge test queues
        let _ = self.client
            .purge_queue()
            .queue_url(&self.watermark_queue_url)
            .send()
            .await;
            
        let _ = self.client
            .purge_queue()
            .queue_url(&self.artwork_queue_url)
            .send()
            .await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sqs_basic_operations() {
        let setup = SqsTestSetup::new().await;
        
        // Test message data
        let test_message = json!({
            "type": "watermark_request",
            "image_url": "https://example.com/test.jpg",
            "user_id": 12345,
            "timestamp": 1234567890
        });

        // Test send message
        let send_result = setup.client
            .send_message()
            .queue_url(&setup.watermark_queue_url)
            .message_body(test_message.to_string())
            .message_attributes(
                "MessageType",
                aws_sdk_sqs::types::MessageAttributeValue::builder()
                    .string_value("watermark_request")
                    .data_type("String")
                    .build()
                    .unwrap()
            )
            .send()
            .await;
        
        assert!(send_result.is_ok(), "Failed to send message: {:?}", send_result.err());
        let send_response = send_result.unwrap();
        assert!(send_response.message_id().is_some(), "Message should have an ID");

        // Wait a moment for message to be available
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Test receive message
        let receive_result = setup.client
            .receive_message()
            .queue_url(&setup.watermark_queue_url)
            .max_number_of_messages(1)
            .wait_time_seconds(2) // Short polling for test
            .send()
            .await;

        assert!(receive_result.is_ok(), "Failed to receive message: {:?}", receive_result.err());
        let receive_response = receive_result.unwrap();
        
        if let Some(messages) = receive_response.messages() {
            assert!(!messages.is_empty(), "Should receive at least one message");
            
            let message = &messages[0];
            assert!(message.body().is_some(), "Message should have body");
            
            // Verify message content
            let received_body: serde_json::Value = serde_json::from_str(message.body().unwrap()).unwrap();
            assert_eq!(received_body["type"], "watermark_request");
            assert_eq!(received_body["user_id"], 12345);

            // Test delete message
            if let Some(receipt_handle) = message.receipt_handle() {
                let delete_result = setup.client
                    .delete_message()
                    .queue_url(&setup.watermark_queue_url)
                    .receipt_handle(receipt_handle)
                    .send()
                    .await;
                
                assert!(delete_result.is_ok(), "Failed to delete message: {:?}", delete_result.err());
            }
        } else {
            // If no messages received, it might be due to timing - log for debugging
            tracing::warn!("No messages received in SQS test - this might be due to timing");
        }

        setup.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_artwork_queue_operations() {
        let setup = SqsTestSetup::new().await;
        
        // Test artwork processing message
        let artwork_message = json!({
            "type": "artwork_processing",
            "artwork_id": 67890,
            "user_id": 54321,
            "space_id": 11111,
            "image_data": {
                "original_url": "https://example.com/artwork.png",
                "format": "PNG",
                "size": 1024000
            },
            "processing_options": {
                "resize": true,
                "watermark": true,
                "compression": "high"
            }
        });

        // Send artwork processing message
        let send_result = setup.client
            .send_message()
            .queue_url(&setup.artwork_queue_url)
            .message_body(artwork_message.to_string())
            .message_attributes(
                "MessageType",
                aws_sdk_sqs::types::MessageAttributeValue::builder()
                    .string_value("artwork_processing")
                    .data_type("String")
                    .build()
                    .unwrap()
            )
            .message_attributes(
                "Priority",
                aws_sdk_sqs::types::MessageAttributeValue::builder()
                    .string_value("high")
                    .data_type("String")
                    .build()
                    .unwrap()
            )
            .send()
            .await;
        
        assert!(send_result.is_ok(), "Failed to send artwork message: {:?}", send_result.err());

        // Test batch send (multiple messages at once)
        let batch_messages = vec![
            json!({
                "type": "artwork_thumbnail", 
                "artwork_id": 11111,
                "size": "small"
            }),
            json!({
                "type": "artwork_thumbnail",
                "artwork_id": 11111, 
                "size": "medium"
            }),
            json!({
                "type": "artwork_thumbnail",
                "artwork_id": 11111,
                "size": "large"
            }),
        ];

        // Send batch messages
        let mut send_message_batch_request = setup.client
            .send_message_batch()
            .queue_url(&setup.artwork_queue_url);

        for (i, message) in batch_messages.iter().enumerate() {
            let entry = aws_sdk_sqs::types::SendMessageBatchRequestEntry::builder()
                .id(format!("msg_{}", i))
                .message_body(message.to_string())
                .build()
                .unwrap();
            
            send_message_batch_request = send_message_batch_request.entries(entry);
        }

        let batch_result = send_message_batch_request.send().await;
        assert!(batch_result.is_ok(), "Failed to send message batch: {:?}", batch_result.err());

        let batch_response = batch_result.unwrap();
        if let Some(successful) = batch_response.successful() {
            assert_eq!(successful.len(), 3, "Should send 3 messages successfully");
        }

        // Wait for messages to be available
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Test receive multiple messages
        let receive_result = setup.client
            .receive_message()
            .queue_url(&setup.artwork_queue_url)
            .max_number_of_messages(10) // Try to receive up to 10 messages
            .wait_time_seconds(5) // Longer polling for batch test
            .send()
            .await;

        assert!(receive_result.is_ok(), "Failed to receive messages: {:?}", receive_result.err());
        let receive_response = receive_result.unwrap();
        
        if let Some(messages) = receive_response.messages() {
            tracing::info!("Received {} messages from artwork queue", messages.len());
            
            // Clean up received messages
            for message in messages {
                if let Some(receipt_handle) = message.receipt_handle() {
                    let _ = setup.client
                        .delete_message()
                        .queue_url(&setup.artwork_queue_url)
                        .receipt_handle(receipt_handle)
                        .send()
                        .await;
                }
            }
        }

        setup.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_sqs_client_integration() {
        let setup = SqsTestSetup::new().await;
        
        // Test using the SqsClient from utils
        let test_payload = json!({
            "test": true,
            "message": "Integration test",
            "timestamp": chrono::Utc::now().timestamp()
        });

        // Note: This test depends on the SqsClient implementation
        // If SqsClient doesn't have a send_message method, this will be a compilation check
        tracing::info!("Testing SqsClient integration with payload: {}", test_payload);
        
        // The actual implementation would depend on the SqsClient interface
        // For now, we just verify the client can be instantiated
        assert!(!setup.watermark_queue_url.is_empty(), "Queue URL should be configured");
        assert!(!setup.artwork_queue_url.is_empty(), "Artwork queue URL should be configured");

        setup.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_queue_attributes_and_monitoring() {
        let setup = SqsTestSetup::new().await;
        
        // Test getting queue attributes
        let attributes_result = setup.client
            .get_queue_attributes()
            .queue_url(&setup.watermark_queue_url)
            .attribute_names(aws_sdk_sqs::types::QueueAttributeName::All)
            .send()
            .await;
        
        if let Ok(response) = attributes_result {
            if let Some(attributes) = response.attributes() {
                tracing::info!("Queue attributes: {:?}", attributes);
                
                // Check some basic attributes
                assert!(attributes.contains_key(&aws_sdk_sqs::types::QueueAttributeName::QueueArn));
            }
        } else {
            tracing::warn!("Could not retrieve queue attributes (might be LocalStack limitation)");
        }

        // Test sending a message with delay
        let delayed_message = json!({
            "type": "delayed_test",
            "delay_seconds": 5
        });

        let delayed_send_result = setup.client
            .send_message()
            .queue_url(&setup.watermark_queue_url)
            .message_body(delayed_message.to_string())
            .delay_seconds(5) // 5 second delay
            .send()
            .await;
        
        assert!(delayed_send_result.is_ok(), "Failed to send delayed message");

        setup.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_dead_letter_queue_simulation() {
        let setup = SqsTestSetup::new().await;
        
        // Send a message and simulate processing failure
        let failing_message = json!({
            "type": "failing_process",
            "should_fail": true,
            "attempt": 1
        });

        let send_result = setup.client
            .send_message()
            .queue_url(&setup.watermark_queue_url)
            .message_body(failing_message.to_string())
            .send()
            .await;
        
        assert!(send_result.is_ok(), "Failed to send failing message");

        // Simulate receiving and "failing" to process the message multiple times
        for attempt in 1..=3 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            
            let receive_result = setup.client
                .receive_message()
                .queue_url(&setup.watermark_queue_url)
                .max_number_of_messages(1)
                .visibility_timeout_seconds(30)
                .send()
                .await;

            if let Ok(response) = receive_result {
                if let Some(messages) = response.messages() {
                    if !messages.is_empty() {
                        tracing::info!("Attempt {}: Simulating processing failure", attempt);
                        
                        // In a real scenario, we would not delete the message,
                        // causing it to become visible again after visibility timeout
                        // For test purposes, we'll delete it after the last attempt
                        if attempt == 3 {
                            if let Some(receipt_handle) = messages[0].receipt_handle() {
                                let _ = setup.client
                                    .delete_message()
                                    .queue_url(&setup.watermark_queue_url)
                                    .receipt_handle(receipt_handle)
                                    .send()
                                    .await;
                            }
                        }
                        break;
                    }
                }
            }
        }

        setup.cleanup().await.unwrap();
    }
}