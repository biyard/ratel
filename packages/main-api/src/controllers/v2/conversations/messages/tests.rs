#[cfg(test)]
mod tests {
    use crate::controllers::v2::conversations::messages::add_messages::{
        self, AddMessageRequest, add_message_handler,
    };
    use crate::controllers::v2::conversations::messages::clear_message::{
        MessagePath, clear_message_handler,
    };
    use crate::controllers::v2::conversations::messages::get_messages::{
        GetMessagesQuery, get_messages_handler,
    };
    use crate::controllers::v2::conversations::messages::poll_messages::{
        PollMessagesQuery, poll_messages_handler,
    };
    use crate::tests::{setup, setup_test_user};
    use bdk::prelude::*;
    use dto::by_axum::auth::Authorization;
    use dto::by_axum::axum::extract::{Path, Query, State};
    use dto::by_axum::axum::{Extension, Json};
    use dto::{
        Conversation, ConversationParticipant, ConversationType, Message, MessageStatus,
        ParticipantRole,
    };
    use tokio::time::{Duration, sleep};

    // Helper function to create a test conversation
    async fn create_test_conversation(
        pool: &sqlx::PgPool,
        creator_id: i64,
        title: String,
        conv_type: ConversationType,
        participant_ids: Vec<i64>,
    ) -> Conversation {
        let mut tx = pool.begin().await.unwrap();

        let conversation_repo = Conversation::get_repository(pool.clone());
        let conversation = conversation_repo
            .insert_with_tx(
                &mut *tx,
                Some(title),
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
        let message_id: i64 = sqlx::query_scalar(
            r#"
            INSERT INTO messages (html_contents, status, sender_id, conversation_id, seq_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 1, EXTRACT(EPOCH FROM NOW())::bigint * 1000, EXTRACT(EPOCH FROM NOW())::bigint * 1000)
            RETURNING id
            "#,
        )
        .bind(content)
        .bind(MessageStatus::Sent as i32)
        .bind(sender_id)
        .bind(conversation_id)
        .fetch_one(pool)
        .await
        .unwrap();

        Message::query_builder()
            .id_equals(message_id)
            .query()
            .map(Message::from)
            .fetch_one(pool)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_add_message_success() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        // Create a test conversation
        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Test Conversation".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        let request = AddMessageRequest {
            html_contents: "<p>Hello, world!</p>".to_string(),
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(add_messages::ConversationPath {
                conversation_id: conversation.id,
            }),
            Json(request),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.message.html_contents, "<p>Hello, world!</p>");
        assert_eq!(response.message.sender_id, ctx.user.id);
        assert_eq!(response.message.conversation_id, conversation.id);
    }

    #[tokio::test]
    async fn test_add_message_unauthorized_user() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        // Create another user with unique ID
        let other_user = setup_test_user(&format!("other-{}", uuid::Uuid::new_v4()), &pool)
            .await
            .unwrap();

        // Create a conversation that the other user is NOT part of
        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Private Conversation".to_string(),
            ConversationType::Direct,
            vec![ctx.user.id],
        )
        .await;

        let request = AddMessageRequest {
            html_contents: "<p>Unauthorized message</p>".to_string(),
            recipient_id: None,
        };

        // Try to add message as other user who is not a participant
        let other_claims = crate::tests::setup_jwt_token(other_user).0;
        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: other_claims,
            })),
            State(pool.clone()),
            Path(add_messages::ConversationPath {
                conversation_id: conversation.id,
            }),
            Json(request),
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_message_invalid_content() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Test Conversation".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        // Test empty content
        let request = AddMessageRequest {
            html_contents: "".to_string(),
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(add_messages::ConversationPath {
                conversation_id: conversation.id,
            }),
            Json(request),
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_messages_success() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Test Conversation".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        // Create test messages
        create_test_message(&pool, conversation.id, ctx.user.id, "Message 1").await;
        create_test_message(&pool, conversation.id, ctx.user.id, "Message 2").await;
        create_test_message(&pool, conversation.id, ctx.user.id, "Message 3").await;

        let query = GetMessagesQuery { size: 10, page: 1 };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::get_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.items.len(), 3);
        assert_eq!(response.total_count, 3);
    }

    #[tokio::test]
    async fn test_get_messages_pagination() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Test Conversation".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        // Create 5 test messages
        for i in 1..=5 {
            create_test_message(
                &pool,
                conversation.id,
                ctx.user.id,
                &format!("Message {}", i),
            )
            .await;
        }

        // Test first page with size 2
        let query = GetMessagesQuery { size: 2, page: 1 };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::get_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.total_count, 5);
    }

    #[tokio::test]
    async fn test_get_messages_unauthorized() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        // Create another user with unique ID
        let other_user = setup_test_user(&format!("unauthorized-{}", uuid::Uuid::new_v4()), &pool)
            .await
            .unwrap();

        // Create a conversation that the other user is NOT part of
        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Private Conversation".to_string(),
            ConversationType::Direct,
            vec![ctx.user.id],
        )
        .await;

        let query = GetMessagesQuery { size: 10, page: 1 };

        let other_claims = crate::tests::setup_jwt_token(other_user).0;
        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: other_claims,
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::get_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_poll_messages_immediate_return_with_new_messages() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Test Conversation".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        // Create a message first
        let first_message =
            create_test_message(&pool, conversation.id, ctx.user.id, "First message").await;

        // Now create another message
        create_test_message(&pool, conversation.id, ctx.user.id, "Second message").await;

        let query = PollMessagesQuery {
            since_id: Some(first_message.id),
            timeout_seconds: Some(5),
        };

        let result = poll_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::poll_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert!(response.has_new_messages);
        assert_eq!(response.messages.len(), 1);
        assert_eq!(response.messages[0].html_contents, "Second message");
    }

    #[tokio::test]
    async fn test_poll_messages_timeout_no_new_messages() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Test Conversation".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        let message =
            create_test_message(&pool, conversation.id, ctx.user.id, "Existing message").await;

        let query = PollMessagesQuery {
            since_id: Some(message.id),
            timeout_seconds: Some(1), // Short timeout for test
        };

        let start = std::time::Instant::now();
        let result = poll_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::poll_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        let duration = start.elapsed();
        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert!(!response.has_new_messages);
        assert_eq!(response.messages.len(), 0);
        assert!(duration >= Duration::from_secs(1));
    }

    #[tokio::test]
    async fn test_poll_messages_unauthorized() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        // Create another user with unique ID
        let other_user = setup_test_user(&format!("poll-unauth-{}", uuid::Uuid::new_v4()), &pool)
            .await
            .unwrap();

        // Create a conversation that the other user is NOT part of
        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Private Conversation".to_string(),
            ConversationType::Direct,
            vec![ctx.user.id],
        )
        .await;

        let query = PollMessagesQuery {
            since_id: None,
            timeout_seconds: Some(1),
        };

        let other_claims = crate::tests::setup_jwt_token(other_user).0;
        let result = poll_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: other_claims,
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::poll_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_poll_messages_with_new_message_arriving() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Test Conversation".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        let existing_message =
            create_test_message(&pool, conversation.id, ctx.user.id, "Existing message").await;

        let query = PollMessagesQuery {
            since_id: Some(existing_message.id),
            timeout_seconds: Some(5),
        };

        // Clone necessary data for the spawned task
        let poll_pool = pool.clone();
        let poll_claims = ctx.claims.clone();
        let poll_conversation_id = conversation.id;

        // Start polling in background
        let poll_handle = tokio::spawn(async move {
            poll_messages_handler(
                Extension(Some(Authorization::Bearer {
                    claims: poll_claims,
                })),
                State(poll_pool),
                Path(crate::controllers::v2::conversations::messages::poll_messages::ConversationPath { conversation_id: poll_conversation_id }),
                Query(query),
            ).await
        });

        // Wait a bit and then add a new message
        sleep(Duration::from_millis(500)).await;
        create_test_message(
            &pool,
            conversation.id,
            ctx.user.id,
            "New message during poll",
        )
        .await;

        // Wait for poll to complete
        let result = poll_handle.await.unwrap();

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert!(response.has_new_messages);
        assert_eq!(response.messages.len(), 1);
        assert_eq!(
            response.messages[0].html_contents,
            "New message during poll"
        );
    }

    #[tokio::test]
    async fn test_clear_message_success() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Test Conversation".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        let message =
            create_test_message(&pool, conversation.id, ctx.user.id, "Message to clear").await;

        let result = clear_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(MessagePath {
                message_id: message.id,
            }),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert!(response.success);

        // Verify message content is cleared
        let cleared_message = Message::query_builder()
            .id_equals(message.id)
            .query()
            .map(Message::from)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(cleared_message.html_contents, "");
    }

    #[tokio::test]
    async fn test_clear_message_unauthorized_not_sender() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        // Create another user with unique ID
        let other_user = setup_test_user(&format!("clear-unauth-{}", uuid::Uuid::new_v4()), &pool)
            .await
            .unwrap();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Test Conversation".to_string(),
            ConversationType::Group,
            vec![ctx.user.id, other_user.id],
        )
        .await;

        // Create message by first user
        let message =
            create_test_message(&pool, conversation.id, ctx.user.id, "Message by user 1").await;

        // Try to clear message as other user
        let other_claims = crate::tests::setup_jwt_token(other_user).0;
        let result = clear_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: other_claims,
            })),
            State(pool.clone()),
            Path(MessagePath {
                message_id: message.id,
            }),
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_clear_message_not_found() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let result = clear_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(MessagePath { message_id: 99999 }), // Non-existent message ID
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_message_sequence_ordering() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Sequence Test".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        // Add multiple messages
        let messages = vec!["First message", "Second message", "Third message"];

        let mut created_messages = Vec::new();
        for content in messages {
            let request = AddMessageRequest {
                html_contents: content.to_string(),
                recipient_id: None,
            };

            let result = add_message_handler(
                Extension(Some(Authorization::Bearer {
                    claims: ctx.claims.clone(),
                })),
                State(pool.clone()),
                Path(add_messages::ConversationPath {
                    conversation_id: conversation.id,
                }),
                Json(request),
            )
            .await;

            assert!(result.is_ok());
            created_messages.push(result.unwrap().0.message);
        }

        // Verify sequence ordering
        for (i, message) in created_messages.iter().enumerate() {
            assert_eq!(message.seq_id, (i + 1) as i64);
        }

        // Get messages and verify order (should be desc by created_at)
        let query = GetMessagesQuery { size: 10, page: 1 };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::get_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.items.len(), 3);

        // Verify all messages are present (order may vary due to same timestamps)
        let contents: Vec<&str> = response
            .items
            .iter()
            .map(|m| m.html_contents.as_str())
            .collect();
        assert!(contents.contains(&"First message"));
        assert!(contents.contains(&"Second message"));
        assert!(contents.contains(&"Third message"));
    }

    #[tokio::test]
    async fn test_message_size_limits() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Size Test".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        // Test maximum allowed size (10000 characters)
        let max_content = "a".repeat(10000);
        let request = AddMessageRequest {
            html_contents: max_content,
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(add_messages::ConversationPath {
                conversation_id: conversation.id,
            }),
            Json(request),
        )
        .await;

        assert!(result.is_ok());

        // Test over maximum size (10001 characters)
        let over_max_content = "a".repeat(10001);
        let request = AddMessageRequest {
            html_contents: over_max_content,
            recipient_id: None,
        };

        let result = add_message_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(add_messages::ConversationPath {
                conversation_id: conversation.id,
            }),
            Json(request),
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_messages_query_validation() {
        let ctx = setup().await.unwrap();
        let pool = ctx.pool.clone();

        let conversation = create_test_conversation(
            &pool,
            ctx.user.id,
            "Validation Test".to_string(),
            ConversationType::Group,
            vec![ctx.user.id],
        )
        .await;

        // Test size > 100 gets clamped to 100
        let query = GetMessagesQuery { size: 150, page: 1 };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::get_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        assert!(result.is_ok());

        // Test size < 1 gets clamped to 1
        let query = GetMessagesQuery { size: 0, page: 1 };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::get_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        assert!(result.is_ok());

        // Test page < 1 gets clamped to 1
        let query = GetMessagesQuery { size: 10, page: 0 };

        let result = get_messages_handler(
            Extension(Some(Authorization::Bearer {
                claims: ctx.claims.clone(),
            })),
            State(pool.clone()),
            Path(
                crate::controllers::v2::conversations::messages::get_messages::ConversationPath {
                    conversation_id: conversation.id,
                },
            ),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
    }
}
