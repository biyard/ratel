use crate::{
    controllers::v3::spaces::create_space::CreateSpaceResponse,
    models::feed::Post,
    post, send,
    tests::v3_setup::{TestContextV3, setup_v3},
    types::PostType,
};

#[tokio::test]
async fn test_create_space() {
    let TestContextV3 {
        app,
        test_user: (user, headers),
        ddb,
        ..
    } = setup_v3().await;
    //FIXME: After Post using session, create a post first
    // let (status, _, res) = post! {
    //     app: app,
    //     path: "/v3/posts",
    //     headers: headers.clone(),
    //     body: {
    //     }
    // };
    // tracing::debug!("Create post response: {:?}", res);
    // assert_eq!(status, 200);
    // let post_pk = res.post_pk;

    let post = Post::new("AA", "BB", PostType::Post, user);
    post.create(&ddb).await.expect("Failed to create post");
    let post_pk = post.pk;

    let (status, _, res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: headers.clone(),
        body: {
            "space_type": 1,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };
    tracing::debug!("Create space response: {:?}", res);
    assert_eq!(status, 200);

    let space_pk = res.space_pk;
    let encoded_pk = percent_encoding::percent_encode(
        space_pk.to_string().as_bytes(),
        percent_encoding::NON_ALPHANUMERIC,
    )
    .to_string();
    let path = format!("/v3/spaces/{}", encoded_pk);

    let (status, _, res) = send! {
        app: app,
        method: "DELETE",
        path: path,
        headers: headers.clone(),
    };
    tracing::debug!("Delete space response: {:?}", res);
    assert_eq!(status, 200);
}
