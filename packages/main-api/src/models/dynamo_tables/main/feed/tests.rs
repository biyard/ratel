use super::*;
use crate::{
    tests::{create_test_user, get_test_aws_config},
    types::*,
    utils::aws::DynamoClient,
};

#[tokio::test]
async fn test_post_creation() {
    let cli = DynamoClient::mock(get_test_aws_config()).client;
    let user = create_test_user(&cli).await;

    let title = uuid::Uuid::new_v4().to_string();
    let html_contents = "<p>This is a test post</p>".to_string();
    let post = Post::new(
        title.clone(),
        html_contents.clone(),
        PostType::Post,
        user.clone(),
    );
    let res = post.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post: {:?}", res);

    let post = Post::get(&cli, &post.pk, Some(EntityType::Post)).await;
    assert!(post.is_ok(), "Failed to find posts: {:?}", post);
    let post = post.unwrap();
    assert!(post.is_some(), "No posts found");
    let post = post.unwrap();
    assert_eq!(post.title, title);
    assert_eq!(post.html_contents, html_contents);
    assert_eq!(post.status, PostStatus::Draft);
    assert_eq!(post.post_type, PostType::Post);
}

#[tokio::test]
async fn test_artwork_post_creation() {
    let cli = DynamoClient::mock(get_test_aws_config()).client;
    let user = create_test_user(&cli).await;

    let title = uuid::Uuid::new_v4().to_string();
    let html_contents = "<p>This is a test artwork post</p>".to_string();
    let post = Post::new(
        title.clone(),
        html_contents.clone(),
        PostType::Artwork,
        user,
    );
    let res = post.create(&cli).await;
    assert!(res.is_ok(), "Failed to create artwork post: {:?}", res);

    let post_artwork_metadata = vec![
        PostArtworkMetadata {
            trait_type: "Background".to_string(),
            value: serde_json::Value::String("#FFFFFF".to_string()),
            display_type: Some("Background".to_string()),
        },
        PostArtworkMetadata {
            trait_type: "Size".to_string(),
            value: serde_json::Value::String("10x10".to_string()),
            display_type: None,
        },
    ];

    let post_artwork = PostArtwork::new(post.pk.clone(), post_artwork_metadata);

    let res = post_artwork.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post artwork: {:?}", res);
    let post_metadata = PostMetadata::query(&cli, post.pk.clone()).await;
    assert!(
        post_metadata.is_ok(),
        "Failed to query post metadata: {:?}",
        res
    );
    let post_metadata = post_metadata.unwrap();

    assert_eq!(
        post_metadata.len(),
        2,
        "Expected 2 post metadata items, got {}",
        post_metadata.len()
    );

    for post in post_metadata.iter() {
        match post {
            PostMetadata::Post(p) => {
                assert_eq!(p.title, title);
                assert_eq!(p.html_contents, html_contents);
                assert_eq!(p.status, PostStatus::Draft);
                assert_eq!(p.post_type, PostType::Artwork);
            }
            PostMetadata::PostArtwork(PostArtwork { metadata, .. }) => {
                assert_eq!(metadata.len(), 2, "Expected 2 artwork metadata items");
            }
            _ => panic!("Expected PostMetadata::Post variant"),
        }
    }
}

#[tokio::test]
async fn test_post_detail_response() {
    let cli = DynamoClient::mock(get_test_aws_config()).client;
    let user = create_test_user(&cli).await;

    let title = uuid::Uuid::new_v4().to_string();
    let html_contents = "<p>This is a test artwork post</p>".to_string();
    let post = Post::new(
        title.clone(),
        html_contents.clone(),
        PostType::Artwork,
        user.clone(),
    );
    let res = post.create(&cli).await;
    assert!(res.is_ok(), "Failed to create artwork post: {:?}", res);

    let post_artwork = PostArtwork::new(
        post.pk.clone(),
        vec![PostArtworkMetadata {
            trait_type: "Background".to_string(),
            value: serde_json::Value::String("#FFFFFF".to_string()),
            display_type: Some("Background".to_string()),
        }],
    );

    let res = post_artwork.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post artwork: {:?}", res);

    let another_user = create_test_user(&cli).await;

    let post_comment_by_user = PostComment::new(
        post.pk.clone(),
        "This is a test comment".to_string(),
        user.clone(),
    );
    let res = post_comment_by_user.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post comment: {:?}", res);

    let post_comment_another_user = PostComment::new(
        post.pk.clone(),
        "This is another test comment".to_string(),
        another_user,
    );
    let res = post_comment_another_user.create(&cli).await;
    assert!(
        res.is_ok(),
        "Failed to create another post comment: {:?}",
        res
    );

    let post = PostMetadata::query(&cli, post.pk.clone()).await;
    assert!(post.is_ok(), "Failed to query post metadata: {:?}", res);
    let post = post.unwrap();
    assert_eq!(
        post.len(),
        4,
        "Expected 4 post metadata items, got {}",
        post.len()
    );

    let post_detail: PostDetailResponse = post.into();
    assert_eq!(post_detail.post.title, title);
    assert_eq!(post_detail.post.html_contents, html_contents);
    assert_eq!(post_detail.post.status, PostStatus::Draft);
    assert_eq!(post_detail.post.post_type, PostType::Artwork);
    assert_eq!(
        post_detail.artwork_metadata.len(),
        1,
        "Expected 1 artwork item"
    );
    assert_eq!(post_detail.comments.len(), 2, "Expected 2 comment items");
}

#[tokio::test]
async fn test_post_reply() {
    let cli = DynamoClient::mock(get_test_aws_config()).client;
    let user = create_test_user(&cli).await;

    let title = uuid::Uuid::new_v4().to_string();
    let html_contents = "<p>This is a Original post</p>".to_string();
    let original_post = Post::new(
        title.clone(),
        html_contents.clone(),
        PostType::Post,
        user.clone(),
    );
    let res = original_post.create(&cli).await;
    assert!(res.is_ok(), "Failed to create original post: {:?}", res);

    let reply_title = uuid::Uuid::new_v4().to_string();
    let reply_html_contents = "<p>This is a Reply post</p>".to_string();
    let reply_post = Post::new(
        reply_title.clone(),
        reply_html_contents.clone(),
        PostType::Repost,
        user.clone(),
    );
    reply_post
        .create(&cli)
        .await
        .expect("Failed to create reply post");

    let res = PostRepost::new(reply_post.pk.clone(), original_post, user)
        .create(&cli)
        .await;
    assert!(res.is_ok(), "Failed to create reply post: {:?}", res);

    let post = PostMetadata::query(&cli, reply_post.pk.clone()).await;
    assert!(post.is_ok(), "Failed to query post metadata: {:?}", post);
    let post = post.unwrap();
    assert_eq!(
        post.len(),
        2,
        "Expected 2 post metadata items, got {}",
        post.len()
    );
}

#[tokio::test]
async fn test_post_like() {
    let cli = DynamoClient::mock(get_test_aws_config()).client;
    let user = create_test_user(&cli).await;

    let title = uuid::Uuid::new_v4().to_string();
    let html_contents = "<p>This is a test post</p>".to_string();
    let post = Post::new(
        title.clone(),
        html_contents.clone(),
        PostType::Post,
        user.clone(),
    );
    let res = post.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post: {:?}", res);

    let user2 = create_test_user(&cli).await;

    let post_like = PostLike::new(post.pk.clone(), user2.clone());
    let res = post_like.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post like: {:?}", res);

    let is_liked = PostLike::get(
        &cli,
        post.pk.clone(),
        Some(EntityType::PostLike(user2.pk.to_string())),
    )
    .await
    .expect("Failed to get post like")
    .is_some();

    assert!(is_liked, "Expected post to be liked");

    let is_liked = PostLike::get(
        &cli,
        post.pk.clone(),
        Some(EntityType::PostLike(user.pk.to_string())),
    )
    .await
    .expect("Failed to get post like")
    .is_some();

    assert!(!is_liked, "Expected post to not be liked");

    let meta = PostMetadata::query(&cli, post.pk.clone())
        .await
        .expect("Failed to query post metadata");
    let res = PostDetailResponse::from(meta);

    println!("PostDetailResponse: {:?}", res);
}
