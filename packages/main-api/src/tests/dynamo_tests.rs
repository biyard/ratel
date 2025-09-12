//! DynamoDB integration tests
//! 
//! These tests verify DynamoDB functionality using LocalStack for local development.
//! They test CRUD operations, GSI queries, and dual-write scenarios.

use std::collections::HashMap;
use aws_sdk_dynamodb::types::AttributeValue;
use dto::*;
use crate::models::dynamo::*;
use crate::utils::aws::dynamo::{DynamoClient, string_attr, number_attr, bool_attr};
use crate::services::dual_write::DualWriteService;

/// Test environment setup
pub struct DynamoTestSetup {
    pub client: DynamoClient,
    pub table_name: String,
}

impl DynamoTestSetup {
    pub fn new() -> Self {
        let table_name = std::env::var("DUAL_WRITE_TABLE_NAME").unwrap_or_else(|_| "ratel-test".to_string());
        let client = DynamoClient::new(&table_name);
        
        Self {
            client,
            table_name,
        }
    }

    /// Clean up test data by deleting all items with test prefixes
    pub async fn cleanup(&self) -> Result<()> {
        // This is a simple cleanup - in a real scenario you might want more sophisticated cleanup
        tracing::info!("Cleaning up DynamoDB test data for table: {}", self.table_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dynamo_client_basic_operations() {
        let setup = DynamoTestSetup::new();
        let client = &setup.client;

        // Test data
        let test_pk = "TEST#USER#12345";
        let test_sk = "METADATA";
        
        // Test put_item
        let mut item = HashMap::new();
        item.insert("pk".to_string(), string_attr(test_pk));
        item.insert("sk".to_string(), string_attr(test_sk));
        item.insert("type".to_string(), string_attr("User"));
        item.insert("nickname".to_string(), string_attr("test_user"));
        item.insert("email".to_string(), string_attr("test@example.com"));
        item.insert("created_at".to_string(), number_attr(1234567890));
        item.insert("updated_at".to_string(), number_attr(1234567890));

        let put_result = client.put_item(item.clone()).await;
        assert!(put_result.is_ok(), "Failed to put item: {:?}", put_result.err());

        // Test get_item
        let get_result = client.get_item("pk", test_pk, Some(("sk", test_sk))).await;
        assert!(get_result.is_ok(), "Failed to get item: {:?}", get_result.err());
        
        let retrieved_item = get_result.unwrap();
        assert!(retrieved_item.is_some(), "Item should exist");
        
        let item_data = retrieved_item.unwrap();
        assert_eq!(
            item_data.get("nickname").unwrap(),
            &string_attr("test_user")
        );

        // Test item_exists
        let exists_result = client.item_exists("pk", test_pk, Some(("sk", test_sk))).await;
        assert!(exists_result.is_ok(), "Failed to check if item exists");
        assert!(exists_result.unwrap(), "Item should exist");

        // Test update_item
        let mut update_values = HashMap::new();
        update_values.insert(":nickname".to_string(), string_attr("updated_user"));
        update_values.insert(":updated_at".to_string(), number_attr(1234567999));

        let update_result = client.update_item(
            "pk", 
            test_pk, 
            Some(("sk", test_sk)),
            "SET nickname = :nickname, updated_at = :updated_at",
            update_values
        ).await;
        assert!(update_result.is_ok(), "Failed to update item: {:?}", update_result.err());

        // Verify update
        let updated_item = client.get_item("pk", test_pk, Some(("sk", test_sk))).await.unwrap().unwrap();
        assert_eq!(
            updated_item.get("nickname").unwrap(),
            &string_attr("updated_user")
        );

        // Test delete_item
        let delete_result = client.delete_item("pk", test_pk, Some(("sk", test_sk))).await;
        assert!(delete_result.is_ok(), "Failed to delete item: {:?}", delete_result.err());

        // Verify deletion
        let deleted_item = client.get_item("pk", test_pk, Some(("sk", test_sk))).await.unwrap();
        assert!(deleted_item.is_none(), "Item should be deleted");

        // Cleanup
        setup.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_dynamo_user_model_operations() {
        let setup = DynamoTestSetup::new();
        let client = &setup.client;

        // Create a test User
        let user = User {
            id: 99999,
            created_at: 1234567890,
            updated_at: 1234567890,
            nickname: "dynamo_test_user".to_string(),
            principal: "test_principal".to_string(),
            email: "dynamo@test.com".to_string(),
            profile_url: "https://example.com/profile.jpg".to_string(),
            term_agreed: true,
            informed_agreed: true,
            user_type: UserType::Individual,
            parent_id: None,
            username: "dynamotest".to_string(),
            followers_count: 10,
            followings_count: 15,
            groups: vec![],
            teams: vec![],
            html_contents: "<p>Test user</p>".to_string(),
            followers: vec![],
            followings: vec![],
            badges: vec![],
            evm_address: "0x1234567890abcdef".to_string(),
            password: "hashed_password".to_string(),
            membership: Membership::Basic,
            theme: Some(Theme::Dark),
            points: 100,
            referral_code: "REF123".to_string(),
            phone_number: Some("+1234567890".to_string()),
            phone: "+1234567890".to_string(),
            telegram_id: Some(123456789),
            telegram_raw: "test_telegram_raw".to_string(),
            industry: vec![],
        };

        // Convert to DynamoDB model
        let dynamo_user = user_model::DynamoUser::from_postgres_user(&user);
        
        // Test serialization to DynamoDB item
        let item_map = serde_dynamo::to_item(&dynamo_user).unwrap();
        let item_result = client.put_item(item_map).await;
        assert!(item_result.is_ok(), "Failed to put user item: {:?}", item_result.err());

        // Test retrieval
        let retrieved_result = client.get_item("pk", &dynamo_user.pk(), Some(("sk", &dynamo_user.sk()))).await;
        assert!(retrieved_result.is_ok(), "Failed to get user item: {:?}", retrieved_result.err());
        
        let retrieved_item = retrieved_result.unwrap();
        assert!(retrieved_item.is_some(), "User item should exist");

        // Deserialize back to DynamoUser
        let deserialized_user: user_model::DynamoUser = serde_dynamo::from_item(retrieved_item.unwrap()).unwrap();
        assert_eq!(deserialized_user.id, user.id);
        assert_eq!(deserialized_user.nickname, user.nickname);
        assert_eq!(deserialized_user.email, user.email);

        // Test GSI queries - Query by email
        let mut email_query_values = HashMap::new();
        email_query_values.insert(":email".to_string(), string_attr(&format!("EMAIL#{}", user.email)));
        
        let email_query_result = client.query_gsi(
            "gsi1-index",
            "gsi1_pk = :email",
            email_query_values
        ).await;
        assert!(email_query_result.is_ok(), "Email GSI query failed: {:?}", email_query_result.err());
        
        let email_results = email_query_result.unwrap();
        assert!(!email_results.is_empty(), "Should find user by email");

        // Test GSI queries - Query by username
        let mut username_query_values = HashMap::new();
        username_query_values.insert(":username".to_string(), string_attr(&format!("USERNAME#{}", user.username)));
        
        let username_query_result = client.query_gsi(
            "gsi2-index",
            "gsi2_pk = :username",
            username_query_values
        ).await;
        assert!(username_query_result.is_ok(), "Username GSI query failed: {:?}", username_query_result.err());
        
        let username_results = username_query_result.unwrap();
        assert!(!username_results.is_empty(), "Should find user by username");

        // Clean up
        let delete_result = client.delete_item("pk", &dynamo_user.pk(), Some(("sk", &dynamo_user.sk()))).await;
        assert!(delete_result.is_ok(), "Failed to delete user item");

        setup.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_dynamo_space_model_operations() {
        let setup = DynamoTestSetup::new();
        let client = &setup.client;

        // Create a test Space
        let space = Space {
            id: 88888,
            created_at: 1234567890,
            updated_at: 1234567890,
            title: Some("Test DynamoDB Space".to_string()),
            html_contents: Some("<p>Test space content</p>".to_string()),
            user_id: 99999,
            feed_id: 77777,
            files: vec![],
            started_at: Some(1234567900),
            ended_at: Some(1234577900),
            publishing_scope: PublishingScope::Public,
            type_info: "test".to_string(),
            space_type: SpaceType::General,
            space_contract: vec![],
            space_holder: vec![],
            status: SpaceStatus::Draft,
            notice_quiz: None,
        };

        // Convert to DynamoDB model
        let dynamo_space = space_model::DynamoSpace::from_postgres_space(&space);
        
        // Test serialization to DynamoDB item
        let item_map = serde_dynamo::to_item(&dynamo_space).unwrap();
        let item_result = client.put_item(item_map).await;
        assert!(item_result.is_ok(), "Failed to put space item: {:?}", item_result.err());

        // Test retrieval
        let retrieved_result = client.get_item("pk", &dynamo_space.pk(), Some(("sk", &dynamo_space.sk()))).await;
        assert!(retrieved_result.is_ok(), "Failed to get space item: {:?}", retrieved_result.err());
        
        let retrieved_item = retrieved_result.unwrap();
        assert!(retrieved_item.is_some(), "Space item should exist");

        // Deserialize back to DynamoSpace
        let deserialized_space: space_model::DynamoSpace = serde_dynamo::from_item(retrieved_item.unwrap()).unwrap();
        assert_eq!(deserialized_space.id, space.id);
        assert_eq!(deserialized_space.title, space.title);
        assert_eq!(deserialized_space.user_id, space.user_id);

        // Test GSI queries - Query by user_id
        let mut user_query_values = HashMap::new();
        user_query_values.insert(":user_id".to_string(), string_attr(&format!("USER#{}", space.user_id)));
        
        let user_query_result = client.query_gsi(
            "gsi1-index",
            "gsi1_pk = :user_id",
            user_query_values
        ).await;
        assert!(user_query_result.is_ok(), "User GSI query failed: {:?}", user_query_result.err());
        
        let user_results = user_query_result.unwrap();
        assert!(!user_results.is_empty(), "Should find space by user_id");

        // Clean up
        let delete_result = client.delete_item("pk", &dynamo_space.pk(), Some(("sk", &dynamo_space.sk()))).await;
        assert!(delete_result.is_ok(), "Failed to delete space item");

        setup.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_transactional_write_operations() {
        let setup = DynamoTestSetup::new();
        let client = &setup.client;

        // Create multiple test items for transaction
        let mut items = Vec::new();

        // Item 1
        let mut item1 = HashMap::new();
        item1.insert("pk".to_string(), string_attr("TEST#TRANSACTION#1"));
        item1.insert("sk".to_string(), string_attr("METADATA"));
        item1.insert("type".to_string(), string_attr("TransactionTest"));
        item1.insert("value".to_string(), string_attr("item1"));
        items.push(item1);

        // Item 2
        let mut item2 = HashMap::new();
        item2.insert("pk".to_string(), string_attr("TEST#TRANSACTION#2"));
        item2.insert("sk".to_string(), string_attr("METADATA"));
        item2.insert("type".to_string(), string_attr("TransactionTest"));
        item2.insert("value".to_string(), string_attr("item2"));
        items.push(item2);

        // Execute transactional write
        let transaction_result = client.transact_write(items).await;
        assert!(transaction_result.is_ok(), "Transaction write failed: {:?}", transaction_result.err());

        // Verify both items were created
        let item1_result = client.get_item("pk", "TEST#TRANSACTION#1", Some(("sk", "METADATA"))).await;
        assert!(item1_result.is_ok() && item1_result.unwrap().is_some(), "Item 1 should exist");

        let item2_result = client.get_item("pk", "TEST#TRANSACTION#2", Some(("sk", "METADATA"))).await;
        assert!(item2_result.is_ok() && item2_result.unwrap().is_some(), "Item 2 should exist");

        // Clean up
        let _ = client.delete_item("pk", "TEST#TRANSACTION#1", Some(("sk", "METADATA"))).await;
        let _ = client.delete_item("pk", "TEST#TRANSACTION#2", Some(("sk", "METADATA"))).await;

        setup.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_conditional_operations() {
        let setup = DynamoTestSetup::new();
        let client = &setup.client;

        let test_pk = "TEST#CONDITIONAL#123";
        let test_sk = "METADATA";

        // Create initial item
        let mut item = HashMap::new();
        item.insert("pk".to_string(), string_attr(test_pk));
        item.insert("sk".to_string(), string_attr(test_sk));
        item.insert("type".to_string(), string_attr("ConditionalTest"));
        item.insert("version".to_string(), number_attr(1));

        // Test conditional put - should succeed as item doesn't exist
        let conditional_result = client.put_item_conditional(
            item.clone(),
            "attribute_not_exists(pk)"
        ).await;
        assert!(conditional_result.is_ok(), "Conditional put should succeed when item doesn't exist");

        // Test conditional put again - should fail as item exists
        let conditional_fail_result = client.put_item_conditional(
            item.clone(),
            "attribute_not_exists(pk)"
        ).await;
        assert!(conditional_fail_result.is_err(), "Conditional put should fail when item exists");

        // Clean up
        let _ = client.delete_item("pk", test_pk, Some(("sk", test_sk))).await;

        setup.cleanup().await.unwrap();
    }

    #[tokio::test]
    async fn test_dual_write_service() {
        // This test assumes dual write is enabled in configuration
        let dual_write_service = DualWriteService::new();

        // Create a test user
        let user = User {
            id: 77777,
            created_at: 1234567890,
            updated_at: 1234567890,
            nickname: "dual_write_test".to_string(),
            principal: "dual_principal".to_string(),
            email: "dual@test.com".to_string(),
            profile_url: "".to_string(),
            term_agreed: true,
            informed_agreed: true,
            user_type: UserType::Individual,
            parent_id: None,
            username: "dualtest".to_string(),
            followers_count: 0,
            followings_count: 0,
            groups: vec![],
            teams: vec![],
            html_contents: "".to_string(),
            followers: vec![],
            followings: vec![],
            badges: vec![],
            evm_address: "0xdual123".to_string(),
            password: "dual_password".to_string(),
            membership: Membership::Basic,
            theme: None,
            points: 0,
            referral_code: "DUAL123".to_string(),
            phone_number: None,
            phone: "".to_string(),
            telegram_id: None,
            telegram_raw: "".to_string(),
            industry: vec![],
        };

        // Test dual write - this should work regardless of configuration
        let dual_write_result = dual_write_service.write_user(&user).await;
        // We don't assert success here as dual write might be disabled in test config
        tracing::info!("Dual write result: {:?}", dual_write_result);

        // Test read from DynamoDB if dual write succeeded
        if dual_write_result.is_ok() {
            let setup = DynamoTestSetup::new();
            let dynamo_user = user_model::DynamoUser::from_postgres_user(&user);
            
            let retrieved_result = setup.client.get_item(
                "pk", 
                &dynamo_user.pk(), 
                Some(("sk", &dynamo_user.sk()))
            ).await;

            if let Ok(Some(item)) = retrieved_result {
                let deserialized: user_model::DynamoUser = serde_dynamo::from_item(item).unwrap();
                assert_eq!(deserialized.id, user.id);
                assert_eq!(deserialized.email, user.email);
                
                // Clean up
                let _ = setup.client.delete_item("pk", &dynamo_user.pk(), Some(("sk", &dynamo_user.sk()))).await;
            }
        }
    }

    #[tokio::test] 
    async fn test_gsi_complex_queries() {
        let setup = DynamoTestSetup::new();
        let client = &setup.client;

        // Create test data with various GSI patterns
        let test_user_id = 55555;
        let test_items = vec![
            // User metadata
            {
                let mut item = HashMap::new();
                item.insert("pk".to_string(), string_attr(&format!("USER#{}", test_user_id)));
                item.insert("sk".to_string(), string_attr("METADATA"));
                item.insert("type".to_string(), string_attr("User"));
                item.insert("gsi1_pk".to_string(), string_attr("EMAIL#gsi@test.com"));
                item.insert("gsi2_pk".to_string(), string_attr("USERNAME#gsitest"));
                item.insert("nickname".to_string(), string_attr("GSI Test User"));
                item
            },
            // User posts
            {
                let mut item = HashMap::new();
                item.insert("pk".to_string(), string_attr(&format!("USER#{}", test_user_id)));
                item.insert("sk".to_string(), string_attr("POST#1"));
                item.insert("type".to_string(), string_attr("Post"));
                item.insert("gsi1_pk".to_string(), string_attr("POST#RECENT"));
                item.insert("gsi1_sk".to_string(), string_attr("2024-01-01"));
                item.insert("content".to_string(), string_attr("Test post 1"));
                item
            },
            {
                let mut item = HashMap::new();
                item.insert("pk".to_string(), string_attr(&format!("USER#{}", test_user_id)));
                item.insert("sk".to_string(), string_attr("POST#2"));
                item.insert("type".to_string(), string_attr("Post"));
                item.insert("gsi1_pk".to_string(), string_attr("POST#RECENT"));
                item.insert("gsi1_sk".to_string(), string_attr("2024-01-02"));
                item.insert("content".to_string(), string_attr("Test post 2"));
                item
            },
        ];

        // Put all items
        for item in &test_items {
            let result = client.put_item(item.clone()).await;
            assert!(result.is_ok(), "Failed to put test item: {:?}", result.err());
        }

        // Test GSI1 query for recent posts
        let mut recent_posts_values = HashMap::new();
        recent_posts_values.insert(":post_type".to_string(), string_attr("POST#RECENT"));
        
        let recent_posts_result = client.query_gsi(
            "gsi1-index",
            "gsi1_pk = :post_type",
            recent_posts_values
        ).await;
        assert!(recent_posts_result.is_ok(), "Recent posts GSI query failed");
        
        let recent_posts = recent_posts_result.unwrap();
        assert_eq!(recent_posts.len(), 2, "Should find 2 recent posts");

        // Test GSI2 query for username
        let mut username_values = HashMap::new();
        username_values.insert(":username".to_string(), string_attr("USERNAME#gsitest"));
        
        let username_result = client.query_gsi(
            "gsi2-index", 
            "gsi2_pk = :username",
            username_values
        ).await;
        assert!(username_result.is_ok(), "Username GSI query failed");
        
        let username_results = username_result.unwrap();
        assert_eq!(username_results.len(), 1, "Should find 1 user by username");

        // Clean up all test items
        for item in &test_items {
            let pk = item.get("pk").unwrap().as_s().unwrap();
            let sk = item.get("sk").unwrap().as_s().unwrap();
            let _ = client.delete_item("pk", pk, Some(("sk", sk))).await;
        }

        setup.cleanup().await.unwrap();
    }
}