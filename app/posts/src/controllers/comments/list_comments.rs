use crate::controllers::dto::*;
use crate::models::*;
use crate::*;
use ratel_auth::OptionalUser;

#[get("/api/posts/:post_pk/comments/:comment_sk?bookmark", user: OptionalUser)]
pub async fn list_comments_handler(
    post_pk: FeedPartition,
    comment_sk: String,
    bookmark: Option<String>,
) -> Result<ListItemsResponse<PostCommentResponse>> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let _user: Option<ratel_auth::User> = user.into();
    let post_pk: Partition = post_pk.into();
    let comment_sk: EntityType = comment_sk
        .parse()
        .map_err(|_| Error::BadRequest("Invalid comment_sk".to_string()))?;

    let (comments, next_bookmark) =
        PostComment::list_by_comment(cli, post_pk, comment_sk, bookmark).await?;
    let items = comments.into_iter().map(PostCommentResponse::from).collect();

    Ok(ListItemsResponse {
        items,
        bookmark: next_bookmark,
    })
}
