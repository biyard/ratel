#[cfg(test)]
mod tests {
    use crate::controllers::v2::conversations::messages::add_messages::*;
    use crate::controllers::v2::conversations::messages::clear_message::*;
    use crate::controllers::v2::conversations::messages::get_messages::*;
    use crate::controllers::v2::conversations::messages::poll_messages::*;
    use crate::tests::{TestContext, setup, setup_test_user};
    use bdk::prelude::*;
    use dto::by_axum::auth::Authorization;
    use dto::by_axum::axum::extract::{Query, State};
    use dto::by_axum::axum::{Extension, Json};
    use dto::{
        Conversation, ConversationParticipant, ConversationType, Message, MessageStatus,
        ParticipantRole,
    };
    use tokio::time::Duration;
    use uuid::Uuid;

    // Helper function to create a test conversation with participants
    async fn create_test_conversation_with_participants(
        pool: &sqlx::PgPool,
        creator_id: i64,
        participant_ids: Vec<i64>,
        conv_type: ConversationType,
        title: Option<String>,
    ) -> Conversation {
        let mut tx = pool.begin().await.unwrap();

        let conversation_repo = Conversation::get_repository(pool.clone());
        let conversation = conversation_repo
            .insert_with_tx(
                &mut *tx,
                title,
                Some("Test description".to_string()),
                conv_type,
            )
            .await
            .unwrap()
            .unwrap();

        // Add creator as admin
        let participant_repo = ConversationParticipant::get_repository(pool.clone());
        participant_repo
            .insert_with_tx(
                &mut *tx,
                conversation.id,
                creator_id,
                ParticipantRole::Admin,
            )
            .await
            .unwrap();

        // Add other participants as members
        for participant_id in participant_ids {
            if participant_id != creator_id {
                participant_repo
                    .insert_with_tx(
                        &mut *tx,
                        conversation.id,
                        participant_id,
                        ParticipantRole::Member,
                    )
                    .await
                    .unwrap();
            }
        }

        tx.commit().await.unwrap();
        conversation
    }

    // Helper function to create a test message
    async fn create_test_message(
        pool: &sqlx::PgPool,
        conversation_id: i64,
        sender_id: i64,
        content: &str,
    ) -> Message {
        let mut tx = pool.begin().await.unwrap();

        // First, lock the conversation to prevent race conditions on seq_id
        sqlx::query("SELECT id FROM conversations WHERE id = $1 FOR UPDATE")
            .bind(conversation_id)
            .execute(&mut *tx)
            .await
            .unwrap();

        // Now get the next seq_id safely
        let next_seq_id: i64 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(seq_id), 0) + 1 FROM messages WHERE conversation_id = $1",
        )
        .bind(conversation_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

        let message_id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO messages (html_content, status, sender_id, conversation_id, seq_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, EXTRACT(EPOCH FROM NOW())::bigint * 1000, EXTRACT(EPOCH FROM NOW())::bigint * 1000)
            RETURNING id
            "#,
        )
        .bind(content)
        .bind(MessageStatus::Sent as i32)
        .bind(sender_id)
        .bind(conversation_id)
        .bind(next_seq_id)
        .bind(content)
        .bind(MessageStatus::Sent as i32)
        .bind(sender_id)
        .bind(conversation_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();

        let message = Message::query_builder()
            .id_equals(message_id)
            .query()
            .map(Message::from)
            .fetch_one(&mut *tx)
            .await
            .unwrap();

        tx.commit().await.unwrap();
        message
    }

    // ADD MESSAGE TESTS
    #[tokio::test]
    async fn test_add_message_to_existing_group_conversation() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create a group conversation
        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user2.id],
            ConversationType::Group,
            Some(format!("Test Group {}", now)),
        )
        .await;

        let html_content = format!("Test message content {}", now);

        let req = AddMessageRequest {
            html_content: html_content.clone(),
            conversation_id: Some(conversation.id),
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.message.html_content, html_content);
        assert_eq!(response.message.sender_id, user.id);
        assert_eq!(response.message.conversation_id, conversation.id);
        assert_eq!(response.message.status, MessageStatus::Sent);
        assert!(response.message.seq_id > 0);
    }

    #[tokio::test]
    async fn test_add_message_to_existing_channel_conversation() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create a channel conversation
        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user2.id],
            ConversationType::Channel,
            Some(format!("Test Channel {}", now)),
        )
        .await;

        let html_content = format!("Test channel message {}", now);

        let req = AddMessageRequest {
            html_content: html_content.clone(),
            conversation_id: Some(conversation.id),
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.message.html_content, html_content);
        assert_eq!(response.message.sender_id, user.id);
        assert_eq!(response.message.conversation_id, conversation.id);
    }

    #[tokio::test]
    async fn test_add_message_create_new_direct_conversation() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        let html_content = format!("Test direct message {}", now);

        let req = AddMessageRequest {
            html_content: html_content.clone(),
            conversation_id: None,
            recipient_id: Some(user2.id),
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        // Debug the actual error
        if let Err(ref e) = result {
            eprintln!("Test failed with error: {:?}", e);
            panic!("Expected success but got error: {:?}", e);
        }

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.message.html_content, html_content);
        assert_eq!(response.message.sender_id, user.id);
        assert_eq!(response.message.status, MessageStatus::Sent);

        // Verify direct conversation was created
        let conversation = Conversation::query_builder()
            .id_equals(response.message.conversation_id)
            .query()
            .map(Conversation::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(conversation.conversation_type, ConversationType::Direct);
        assert_eq!(conversation.title, None);

        // Verify both users are participants
        let participants = ConversationParticipant::query_builder()
            .conversation_id_equals(conversation.id)
            .query()
            .map(ConversationParticipant::from)
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(participants.len(), 2);
        let participant_ids: Vec<i64> = participants.iter().map(|p| p.user_id).collect();
        assert!(participant_ids.contains(&user.id));
        assert!(participant_ids.contains(&user2.id));
    }

    #[tokio::test]
    async fn test_add_message_use_existing_direct_conversation() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create existing direct conversation
        let existing_conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user2.id],
            ConversationType::Direct,
            None,
        )
        .await;

        let html_content = format!("Test direct message in existing conversation {}", now);

        let req = AddMessageRequest {
            html_content: html_content.clone(),
            conversation_id: None,
            recipient_id: Some(user2.id),
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        // Should use existing conversation, not create a new one
        assert_eq!(response.message.conversation_id, existing_conversation.id);
        assert_eq!(response.message.html_content, html_content);
        assert_eq!(response.message.sender_id, user.id);
    }

    #[tokio::test]
    async fn test_add_message_unauthorized_to_conversation() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        let user3_id = Uuid::new_v4().to_string();
        let user3 = setup_test_user(&user3_id, &pool).await.unwrap();

        // Create conversation where user is NOT a participant
        let conversation = create_test_conversation_with_participants(
            &pool,
            user2.id,
            vec![user3.id], // Only user2 and user3 are participants
            ConversationType::Group,
            Some(format!("Private Group {}", now)),
        )
        .await;

        let req = AddMessageRequest {
            html_content: "Unauthorized message".to_string(),
            conversation_id: Some(conversation.id),
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(
            result.is_err(),
            "Should be unauthorized to send message to conversation user is not part of"
        );
    }

    #[tokio::test]
    async fn test_add_message_to_nonexistent_conversation() {
        let TestContext { user, pool, .. } = setup().await.unwrap();

        let req = AddMessageRequest {
            html_content: "Test message".to_string(),
            conversation_id: Some(999999), // Non-existent conversation
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail for non-existent conversation");
    }

    #[tokio::test]
    async fn test_add_message_to_nonexistent_recipient() {
        let TestContext { user, pool, .. } = setup().await.unwrap();

        let req = AddMessageRequest {
            html_content: "Test message".to_string(),
            conversation_id: None,
            recipient_id: Some(999999), // Non-existent user
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail for non-existent recipient");
    }

    #[tokio::test]
    async fn test_add_message_to_self() {
        let TestContext { user, pool, .. } = setup().await.unwrap();

        let req = AddMessageRequest {
            html_content: "Message to self".to_string(),
            conversation_id: None,
            recipient_id: Some(user.id), // Same as sender
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should not allow sending message to self");
    }

    #[tokio::test]
    async fn test_add_message_validation_errors() {
        let TestContext { user, pool, .. } = setup().await.unwrap();

        // Test empty content
        let req = AddMessageRequest {
            html_content: "".to_string(),
            conversation_id: Some(1),
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail with empty content");

        // Test content too long (10000+ characters)
        let long_content = "a".repeat(10001);
        let req = AddMessageRequest {
            html_content: long_content,
            conversation_id: Some(1),
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail with content too long");

        // Test missing both conversation_id and recipient_id
        let req = AddMessageRequest {
            html_content: "Valid content".to_string(),
            conversation_id: None,
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(
            result.is_err(),
            "Should fail when both conversation_id and recipient_id are missing"
        );
    }

    #[tokio::test]
    async fn test_add_message_sequential_ids() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create a conversation
        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user2.id],
            ConversationType::Group,
            Some(format!("Test Sequential {}", now)),
        )
        .await;

        // Add first message
        let req1 = AddMessageRequest {
            html_content: "First message".to_string(),
            conversation_id: Some(conversation.id),
            recipient_id: None,
        };

        let result1 = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req1),
        )
        .await;

        assert!(result1.is_ok());
        let message1 = result1.unwrap().0.message;

        // Add second message
        let req2 = AddMessageRequest {
            html_content: "Second message".to_string(),
            conversation_id: Some(conversation.id),
            recipient_id: None,
        };

        let result2 = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user2.clone()).0,
            })),
            State(pool.clone()),
            Json(req2),
        )
        .await;

        assert!(result2.is_ok());
        let message2 = result2.unwrap().0.message;

        // Verify sequential seq_ids
        assert!(message2.seq_id > message1.seq_id);
        assert_eq!(message2.seq_id, message1.seq_id + 1);
    }

    // GET MESSAGES TESTS
    #[tokio::test]
    async fn test_get_messages_success() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create conversation and messages
        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user2.id],
            ConversationType::Group,
            Some(format!("Test Messages {}", now)),
        )
        .await;

        // Create test messages
        let _message1 = create_test_message(&pool, conversation.id, user.id, "Message 1").await;
        let _message2 = create_test_message(&pool, conversation.id, user2.id, "Message 2").await;

        let query = GetMessagesQuery {
            conversation_id: conversation.id,
            size: 50,
            page: 1,
        };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.items.len(), 2);
        assert_eq!(response.total_count, 2);

        // Messages should be ordered by created_at DESC (newest first)
        assert!(response.items[0].created_at >= response.items[1].created_at);
    }

    #[tokio::test]
    async fn test_get_messages_with_pagination() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create conversation
        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user2.id],
            ConversationType::Group,
            Some(format!("Test Pagination {}", now)),
        )
        .await;

        // Create multiple messages
        for i in 0..5 {
            create_test_message(&pool, conversation.id, user.id, &format!("Message {}", i)).await;
            tokio::time::sleep(Duration::from_millis(10)).await; // Ensure different timestamps
        }

        // Test first page
        let query = GetMessagesQuery {
            conversation_id: conversation.id,
            size: 2,
            page: 1,
        };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.items.len(), 2);
        assert_eq!(response.total_count, 5);

        // Test second page
        let query = GetMessagesQuery {
            conversation_id: conversation.id,
            size: 2,
            page: 2,
        };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.items.len(), 2);
        assert_eq!(response.total_count, 5);

        // Test last page
        let query = GetMessagesQuery {
            conversation_id: conversation.id,
            size: 2,
            page: 3,
        };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.total_count, 5);
    }

    #[tokio::test]
    async fn test_get_messages_unauthorized() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        let user3_id = Uuid::new_v4().to_string();
        let user3 = setup_test_user(&user3_id, &pool).await.unwrap();

        // Create conversation where user is NOT a participant
        let conversation = create_test_conversation_with_participants(
            &pool,
            user2.id,
            vec![user3.id], // Only user2 and user3
            ConversationType::Group,
            Some(format!("Private Messages {}", now)),
        )
        .await;

        let query = GetMessagesQuery {
            conversation_id: conversation.id,
            size: 50,
            page: 1,
        };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(
            result.is_err(),
            "Should be unauthorized to get messages from conversation user is not part of"
        );
    }

    #[tokio::test]
    async fn test_get_messages_invalid_conversation_id() {
        let TestContext { user, pool, .. } = setup().await.unwrap();

        // Test with conversation_id = 0
        let query = GetMessagesQuery {
            conversation_id: 0,
            size: 50,
            page: 1,
        };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_err(), "Should fail with invalid conversation_id");
    }

    #[tokio::test]
    async fn test_get_messages_size_and_page_validation() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user.id],
            ConversationType::Group,
            Some(format!("Test Validation {}", now)),
        )
        .await;

        // Test size too large (should be capped to 100)
        let query = GetMessagesQuery {
            conversation_id: conversation.id,
            size: 200,
            page: 1,
        };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok(), "Should handle large size by capping it");

        // Test negative size (should be set to minimum 1)
        let query = GetMessagesQuery {
            conversation_id: conversation.id,
            size: -5,
            page: 1,
        };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(
            result.is_ok(),
            "Should handle negative size by setting to minimum"
        );

        // Test page less than 1 (should be set to minimum 1)
        let query = GetMessagesQuery {
            conversation_id: conversation.id,
            size: 10,
            page: 0,
        };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(
            result.is_ok(),
            "Should handle page less than 1 by setting to minimum"
        );
    }

    // POLL MESSAGES TESTS
    #[tokio::test]
    async fn test_poll_messages_with_new_messages() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create conversation
        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user2.id],
            ConversationType::Group,
            Some(format!("Test Poll {}", now)),
        )
        .await;

        // Create an existing message
        let existing_message =
            create_test_message(&pool, conversation.id, user.id, "Existing message").await;

        // Start polling from after the existing message
        let since_timestamp = existing_message.created_at;

        let query = PollMessagesQuery {
            conversation_id: conversation.id,
            since: Some(since_timestamp),
            timeout_seconds: Some(3), // Longer timeout to allow polling to catch the new message
        };

        // Create new message in a separate task to simulate real-time message
        let pool_clone = pool.clone();
        let conversation_id = conversation.id;
        let user2_id_for_task = user2.id;
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await; // Wait longer to ensure polling starts
            create_test_message(
                &pool_clone,
                conversation_id,
                user2_id_for_task,
                "New message",
            )
            .await;
        });

        let result = poll_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.has_new_messages, true);
        assert_eq!(response.messages.len(), 1);
        assert_eq!(response.messages[0].html_content, "New message");
        assert!(response.messages[0].created_at > since_timestamp);
    }

    #[tokio::test]
    async fn test_poll_messages_timeout() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create conversation
        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user2.id],
            ConversationType::Group,
            Some(format!("Test Poll Timeout {}", now)),
        )
        .await;

        let query = PollMessagesQuery {
            conversation_id: conversation.id,
            since: Some(chrono::Utc::now().timestamp_millis()),
            timeout_seconds: Some(1), // Short timeout
        };

        let start = std::time::Instant::now();
        let result = poll_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        let elapsed = start.elapsed();

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.has_new_messages, false);
        assert_eq!(response.messages.len(), 0);
        assert!(elapsed >= Duration::from_secs(1)); // Should have waited for timeout
    }

    #[tokio::test]
    async fn test_poll_messages_unauthorized() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        let user3_id = Uuid::new_v4().to_string();
        let user3 = setup_test_user(&user3_id, &pool).await.unwrap();

        // Create conversation where user is NOT a participant
        let conversation = create_test_conversation_with_participants(
            &pool,
            user2.id,
            vec![user3.id],
            ConversationType::Group,
            Some(format!("Private Poll {}", now)),
        )
        .await;

        let query = PollMessagesQuery {
            conversation_id: conversation.id,
            since: Some(0),
            timeout_seconds: Some(1),
        };

        let result = poll_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(
            result.is_err(),
            "Should be unauthorized to poll messages from conversation user is not part of"
        );
    }

    #[tokio::test]
    async fn test_poll_messages_default_values() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user.id],
            ConversationType::Group,
            Some(format!("Test Poll Defaults {}", now)),
        )
        .await;

        // Create a message to be returned immediately
        create_test_message(&pool, conversation.id, user.id, "Immediate message").await;

        let query = PollMessagesQuery {
            conversation_id: conversation.id,
            since: None,           // Should default to 0
            timeout_seconds: None, // Should default to 30
        };

        let result = poll_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        // Should return immediately since there are messages after timestamp 0
        assert_eq!(response.has_new_messages, true);
        assert_eq!(response.messages.len(), 1);
    }

    // CLEAR MESSAGE TESTS
    #[tokio::test]
    async fn test_clear_message_success() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create conversation and message
        let conversation = create_test_conversation_with_participants(
            &pool,
            user.id,
            vec![user2.id],
            ConversationType::Group,
            Some(format!("Test Clear {}", now)),
        )
        .await;

        let message =
            create_test_message(&pool, conversation.id, user.id, "Message to clear").await;

        let req = ClearMessageRequest {
            message_id: message.id,
        };

        let result = clear_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_ok());

        // Verify message content was cleared
        let cleared_message = Message::query_builder()
            .id_equals(message.id)
            .query()
            .map(Message::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(cleared_message.html_content, "");
        assert_eq!(cleared_message.seq_id, message.seq_id); // seq_id should remain
        assert_eq!(cleared_message.conversation_id, message.conversation_id);
    }

    #[tokio::test]
    async fn test_clear_message_unauthorized() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        let user3_id = Uuid::new_v4().to_string();
        let user3 = setup_test_user(&user3_id, &pool).await.unwrap();

        // Create conversation and message by user2
        let conversation = create_test_conversation_with_participants(
            &pool,
            user2.id,
            vec![user3.id],
            ConversationType::Group,
            Some(format!("Test Clear Unauthorized {}", now)),
        )
        .await;

        let message =
            create_test_message(&pool, conversation.id, user2.id, "Message by user2").await;

        // Try to clear as user (not sender, not participant)
        let req = ClearMessageRequest {
            message_id: message.id,
        };

        let result = clear_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(
            result.is_err(),
            "Should be unauthorized to clear message not sent by user"
        );
    }

    #[tokio::test]
    async fn test_clear_nonexistent_message() {
        let TestContext { user, pool, .. } = setup().await.unwrap();

        let req = ClearMessageRequest {
            message_id: 999999, // Non-existent message
        };

        let result = clear_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail for non-existent message");
    }

    #[tokio::test]
    async fn test_unauthorized_message_requests() {
        let TestContext { pool, now, .. } = setup().await.unwrap();

        // Test add message without auth
        let req = AddMessageRequest {
            html_content: format!("Test {}", now),
            conversation_id: Some(1),
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(None), // No authorization
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail without authorization");

        // Test get messages without auth
        let query = GetMessagesQuery {
            conversation_id: 1,
            size: 50,
            page: 1,
        };

        let result = get_messages_handler(
            Extension(None), // No authorization
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_err(), "Should fail without authorization");

        // Test poll messages without auth
        let query = PollMessagesQuery {
            conversation_id: 1,
            since: Some(0),
            timeout_seconds: Some(1),
        };

        let result = poll_messages_handler(
            Extension(None), // No authorization
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_err(), "Should fail without authorization");

        // Test clear message without auth
        let req = ClearMessageRequest { message_id: 1 };

        let result = clear_message_handler(
            Extension(None), // No authorization
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail without authorization");
    }
}
