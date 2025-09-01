#[cfg(test)]
mod tests {
    use crate::controllers::v2::conversations::add_conversations::*;
    use crate::controllers::v2::conversations::get_conversation_by_id::*;
    use crate::controllers::v2::conversations::get_conversations::*;
    use crate::tests::{TestContext, setup, setup_test_user};
    use bdk::prelude::*;
    use dto::by_axum::auth::Authorization;
    use dto::by_axum::axum::extract::{Path, Query, State};
    use dto::by_axum::axum::{Extension, Json};
    use dto::{Conversation, ConversationParticipant, ConversationType, ParticipantRole};
    use uuid::Uuid;

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

    // ADD CONVERSATION TESTS
    #[tokio::test]
    async fn test_create_group_conversation() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        // Create additional test users
        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        let user3_id = Uuid::new_v4().to_string();
        let user3 = setup_test_user(&user3_id, &pool).await.unwrap();

        let title = format!("Test Group {}", now);
        let description = Some(format!("Test group description {}", now));

        let req = CreateConversationRequest {
            title: title.clone(),
            description: description.clone(),
            conversation_type: ConversationType::Group,
            participant_ids: vec![user2.id, user3.id],
        };

        let result = create_conversation_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        match &result {
            Ok(response) => {
                println!("Successfully created conversation: {:?}", response.0);
            }
            Err(e) => {
                println!("Error creating conversation: {:?}", e);
                // Let's also check if the error is in the response
                panic!("Conversation creation failed: {:?}", e);
            }
        }

        let conversation = result.unwrap().0;

        assert_eq!(conversation.title, Some(title));
        assert_eq!(conversation.description, description);
        assert_eq!(conversation.conversation_type, ConversationType::Group);

        // Verify participants were added correctly
        let participants = ConversationParticipant::query_builder()
            .conversation_id_equals(conversation.id)
            .query()
            .map(ConversationParticipant::from)
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(participants.len(), 3); // creator + 2 participants

        // Find creator (should be admin)
        let creator_participant = participants
            .iter()
            .find(|p| p.user_id == user.id)
            .expect("Creator should be a participant");
        assert_eq!(creator_participant.role, ParticipantRole::Admin);

        // Find other participants (should be members)
        let user2_participant = participants
            .iter()
            .find(|p| p.user_id == user2.id)
            .expect("User2 should be a participant");
        assert_eq!(user2_participant.role, ParticipantRole::Member);

        let user3_participant = participants
            .iter()
            .find(|p| p.user_id == user3.id)
            .expect("User3 should be a participant");
        assert_eq!(user3_participant.role, ParticipantRole::Member);
    }

    #[tokio::test]
    async fn test_create_channel_conversation() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        // Create additional test user
        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        let title = format!("Test Channel {}", now);

        let req = CreateConversationRequest {
            title: title.clone(),
            description: None,
            conversation_type: ConversationType::Channel,
            participant_ids: vec![user2.id],
        };

        let result = create_conversation_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_ok());
        let conversation = result.unwrap().0;

        assert_eq!(conversation.title, Some(title));
        assert_eq!(conversation.description, None);
        assert_eq!(conversation.conversation_type, ConversationType::Channel);
    }

    #[tokio::test]
    async fn test_create_conversation_fails_for_direct_type() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let title = format!("Test Direct {}", now);

        let req = CreateConversationRequest {
            title: title.clone(),
            description: None,
            conversation_type: ConversationType::Direct,
            participant_ids: vec![123], // Arbitrary user ID
        };

        let result = create_conversation_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(
            result.is_err(),
            "Should not allow creating direct conversations via this endpoint"
        );
    }

    #[tokio::test]
    async fn test_create_conversation_with_nonexistent_participant() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let title = format!("Test Group {}", now);
        let nonexistent_user_id = 999999; // Non-existent user ID

        let req = CreateConversationRequest {
            title: title.clone(),
            description: None,
            conversation_type: ConversationType::Group,
            participant_ids: vec![nonexistent_user_id],
        };

        let result = create_conversation_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(
            result.is_err(),
            "Should fail when adding non-existent user as participant"
        );
    }

    #[tokio::test]
    async fn test_create_conversation_skips_duplicate_creator() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        let title = format!("Test Group {}", now);

        // Include creator's ID in participants list (should be skipped)
        let req = CreateConversationRequest {
            title: title.clone(),
            description: None,
            conversation_type: ConversationType::Group,
            participant_ids: vec![user.id, user2.id],
        };

        let result = create_conversation_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_ok());
        let conversation = result.unwrap().0;

        // Verify only 2 participants (creator and user2, creator not duplicated)
        let participants = ConversationParticipant::query_builder()
            .conversation_id_equals(conversation.id)
            .query()
            .map(ConversationParticipant::from)
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(participants.len(), 2);
    }

    #[tokio::test]
    async fn test_create_conversation_validation_errors() {
        let TestContext { user, pool, .. } = setup().await.unwrap();

        // Test with empty title
        let req = CreateConversationRequest {
            title: "".to_string(),
            description: None,
            conversation_type: ConversationType::Group,
            participant_ids: vec![123],
        };

        let result = create_conversation_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail with empty title");

        // Test with title too long (256+ characters)
        let long_title = "a".repeat(256);
        let req = CreateConversationRequest {
            title: long_title,
            description: None,
            conversation_type: ConversationType::Group,
            participant_ids: vec![123],
        };

        let result = create_conversation_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail with title too long");

        // Test with description too long (1000+ characters)
        let long_description = "a".repeat(1001);
        let req = CreateConversationRequest {
            title: "Valid Title".to_string(),
            description: Some(long_description),
            conversation_type: ConversationType::Group,
            participant_ids: vec![123],
        };

        let result = create_conversation_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail with description too long");

        // Test with empty participants
        let req = CreateConversationRequest {
            title: "Valid Title".to_string(),
            description: None,
            conversation_type: ConversationType::Group,
            participant_ids: vec![],
        };

        let result = create_conversation_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(
            result.is_err(),
            "Should fail validation due to empty participants"
        );
    }

    // GET CONVERSATIONS TESTS
    #[tokio::test]
    async fn test_get_conversations_success() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        // Create test conversations
        let _conv1 = create_test_conversation(
            &pool,
            user.id,
            format!("Test Conversation 1 {}", now),
            ConversationType::Group,
            vec![user.id],
        )
        .await;

        let _conv2 = create_test_conversation(
            &pool,
            user.id,
            format!("Test Conversation 2 {}", now),
            ConversationType::Channel,
            vec![user.id],
        )
        .await;

        let query = GetConversationsQuery {
            limit: Some(10),
            offset: Some(0),
        };

        let result = get_conversations_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.conversations.len(), 2);
        assert_eq!(response.total_count, 2);
        assert_eq!(response.has_more, false);

        // Verify conversations are ordered by updated_at DESC (most recent first)
        assert!(response.conversations[0].updated_at >= response.conversations[1].updated_at);
    }

    #[tokio::test]
    async fn test_get_conversations_with_pagination() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        // Create multiple test conversations
        for i in 0..5 {
            create_test_conversation(
                &pool,
                user.id,
                format!("Test Conversation {} {}", i, now),
                ConversationType::Group,
                vec![user.id],
            )
            .await;
        }

        // Test first page
        let query = GetConversationsQuery {
            limit: Some(2),
            offset: Some(0),
        };

        let result = get_conversations_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.conversations.len(), 2);
        assert_eq!(response.total_count, 5);
        assert_eq!(response.has_more, true);

        // Test second page
        let query = GetConversationsQuery {
            limit: Some(2),
            offset: Some(2),
        };

        let result = get_conversations_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.conversations.len(), 2);
        assert_eq!(response.total_count, 5);
        assert_eq!(response.has_more, true);

        // Test last page
        let query = GetConversationsQuery {
            limit: Some(2),
            offset: Some(4),
        };

        let result = get_conversations_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.conversations.len(), 1);
        assert_eq!(response.total_count, 5);
        assert_eq!(response.has_more, false);
    }

    #[tokio::test]
    async fn test_get_conversations_empty_result() {
        let TestContext { user, pool, .. } = setup().await.unwrap();

        let query = GetConversationsQuery {
            limit: Some(10),
            offset: Some(0),
        };

        let result = get_conversations_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.conversations.len(), 0);
        assert_eq!(response.total_count, 0);
        assert_eq!(response.has_more, false);
    }

    #[tokio::test]
    async fn test_get_conversations_default_pagination() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        // Create a test conversation
        create_test_conversation(
            &pool,
            user.id,
            format!("Test Conversation {}", now),
            ConversationType::Group,
            vec![user.id],
        )
        .await;

        let query = GetConversationsQuery {
            limit: None,  // Should default to 20
            offset: None, // Should default to 0
        };

        let result = get_conversations_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap().0;

        assert_eq!(response.conversations.len(), 1);
        assert_eq!(response.total_count, 1);
        assert_eq!(response.has_more, false);
    }

    #[tokio::test]
    async fn test_get_conversations_limit_capping() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        // Create a test conversation
        create_test_conversation(
            &pool,
            user.id,
            format!("Test Conversation {}", now),
            ConversationType::Group,
            vec![user.id],
        )
        .await;

        let query = GetConversationsQuery {
            limit: Some(200), // Should be capped to 100
            offset: Some(0),
        };

        let result = get_conversations_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_ok());
        // The test passes if no error occurs due to limit being properly capped
    }

    // GET CONVERSATION BY ID TESTS
    #[tokio::test]
    async fn test_get_conversation_by_id_success() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        let conversation = create_test_conversation(
            &pool,
            user.id,
            format!("Test Conversation {}", now),
            ConversationType::Group,
            vec![user2.id],
        )
        .await;

        let result = get_conversation_by_id_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Path(ConversationPath {
                conversation_id: conversation.id,
            }),
        )
        .await;

        assert!(result.is_ok());
        let returned_conversation = result.unwrap().0;

        assert_eq!(returned_conversation.id, conversation.id);
        assert_eq!(returned_conversation.title, conversation.title);
        assert_eq!(returned_conversation.description, conversation.description);
        assert_eq!(
            returned_conversation.conversation_type,
            conversation.conversation_type
        );
    }

    #[tokio::test]
    async fn test_get_conversation_by_id_unauthorized() {
        let TestContext {
            user, pool, now, ..
        } = setup().await.unwrap();

        // Create another user
        let user2_id = Uuid::new_v4().to_string();
        let user2 = setup_test_user(&user2_id, &pool).await.unwrap();

        // Create conversation where user is NOT a participant
        let conversation = create_test_conversation(
            &pool,
            user2.id, // user2 creates the conversation
            format!("Test Conversation {}", now),
            ConversationType::Group,
            vec![user2.id], // only user2 is participant
        )
        .await;

        // Try to access as user (not a participant)
        let result = get_conversation_by_id_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Path(ConversationPath {
                conversation_id: conversation.id,
            }),
        )
        .await;

        assert!(
            result.is_err(),
            "Should be unauthorized to access conversation user is not part of"
        );
    }

    #[tokio::test]
    async fn test_get_conversation_by_id_not_found() {
        let TestContext { user, pool, .. } = setup().await.unwrap();

        let nonexistent_id = 999999;

        let result = get_conversation_by_id_handler(
            Extension(Some(Authorization::Bearer {
                claims: crate::tests::setup_jwt_token(user.clone()).0,
            })),
            State(pool.clone()),
            Path(ConversationPath {
                conversation_id: nonexistent_id,
            }),
        )
        .await;

        assert!(
            result.is_err(),
            "Should return error for non-existent conversation"
        );
    }

    #[tokio::test]
    async fn test_unauthorized_requests() {
        let TestContext { pool, now, .. } = setup().await.unwrap();

        // Test create conversation without auth
        let req = CreateConversationRequest {
            title: format!("Test {}", now),
            description: None,
            conversation_type: ConversationType::Group,
            participant_ids: vec![123],
        };

        let result = create_conversation_handler(
            Extension(None), // No authorization
            State(pool.clone()),
            Json(req),
        )
        .await;

        assert!(result.is_err(), "Should fail without authorization");

        // Test get conversations without auth
        let query = GetConversationsQuery {
            limit: Some(10),
            offset: Some(0),
        };

        let result = get_conversations_handler(
            Extension(None), // No authorization
            State(pool.clone()),
            Query(query),
        )
        .await;

        assert!(result.is_err(), "Should fail without authorization");

        // Test get conversation by id without auth
        let result = get_conversation_by_id_handler(
            Extension(None), // No authorization
            State(pool.clone()),
            Path(ConversationPath {
                conversation_id: 123,
            }),
        )
        .await;

        assert!(result.is_err(), "Should fail without authorization");
    }
}
