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

    let posts = Post::find_posts(&cli, EntityType::Post, Default::default()).await;
    assert!(posts.is_ok(), "Failed to find posts: {:?}", posts);
    let (posts, bookmark) = posts.unwrap();
    assert!(
        posts.len() >= 1,
        "Expected at least 1 post, got {}",
        posts.len()
    );
    assert!(
        bookmark.is_none(),
        "Expected no bookmark, got {:?}",
        bookmark
    );

    let post = &posts[0];
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

    let post_artwork1 = PostArtworkMetadata::new(
        post.pk.clone(),
        "Background".to_string(),
        "#FFFFFF".to_string(),
        Some("Background".to_string()),
    );

    let post_artwork2 = PostArtworkMetadata::new(
        post.pk.clone(),
        "Size".to_string(),
        "10x10".to_string(),
        None,
    );

    let res = post_artwork1.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post artwork 1: {:?}", res);
    let res = post_artwork2.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post artwork 2: {:?}", res);
    let post_summary = PostSummary::query(&cli, post.pk.clone()).await;
    assert!(
        post_summary.is_ok(),
        "Failed to query post summary: {:?}",
        res
    );
    let post_summary = post_summary.unwrap();

    assert_eq!(
        post_summary.len(),
        3,
        "Expected 3 post summary items, got {}",
        post_summary.len()
    );

    let mut artwork_count = 0;
    for post in post_summary.iter() {
        match post {
            PostSummary::Post(p) => {
                assert_eq!(p.title, title);
                assert_eq!(p.html_contents, html_contents);
                assert_eq!(p.status, PostStatus::Draft);
                assert_eq!(p.post_type, PostType::Artwork);
            }
            PostSummary::PostArtworkMetadata(_) => {
                artwork_count += 1;
            }
            _ => panic!("Expected PostSummary::Post variant"),
        }
    }
    assert_eq!(artwork_count, 2, "Expected 2 artwork metadata items");
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

    let post_artwork1 = PostArtworkMetadata::new(
        post.pk.clone(),
        "Background".to_string(),
        "#FFFFFF".to_string(),
        Some("Background".to_string()),
    );

    let post_artwork2 = PostArtworkMetadata::new(
        post.pk.clone(),
        "Size".to_string(),
        "10x10".to_string(),
        None,
    );

    let res = post_artwork1.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post artwork 1: {:?}", res);
    let res = post_artwork2.create(&cli).await;
    assert!(res.is_ok(), "Failed to create post artwork 2: {:?}", res);

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

    let post = PostSummary::query(&cli, post.pk.clone()).await;
    assert!(post.is_ok(), "Failed to query post summary: {:?}", res);
    let post = post.unwrap();
    assert_eq!(
        post.len(),
        5,
        "Expected 5 post summary items, got {}",
        post.len()
    );

    let post_detail: PostDetailResponse = post.into();
    assert_eq!(post_detail.post.title, title);
    assert_eq!(post_detail.post.html_contents, html_contents);
    assert_eq!(post_detail.post.status, PostStatus::Draft);
    assert_eq!(post_detail.post.post_type, PostType::Artwork);
    assert_eq!(
        post_detail.artwork_metadatas.len(),
        2,
        "Expected 2 artwork items"
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

    let post = PostSummary::query(&cli, reply_post.pk.clone()).await;
    assert!(post.is_ok(), "Failed to query post summary: {:?}", post);
    let post = post.unwrap();
    assert_eq!(
        post.len(),
        2,
        "Expected 2 post summary items, got {}",
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
}
