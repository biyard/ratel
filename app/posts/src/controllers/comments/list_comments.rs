use crate::controllers::dto::*;
use crate::models::*;
use crate::*;
use ratel_auth::OptionalUser;

#[get("/api/posts/:post_pk/comments/:comment_sk", user: OptionalUser)]
pub async fn list_comments_handler(
    post_pk: FeedPartition,
    comment_sk: EntityType,
) -> Result<ListItemsResponse<PostComment>> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let _user: Option<ratel_auth::User> = user.into();
    let post_pk: Partition = post_pk.into();

    let comments = PostComment::list_by_comment(cli, post_pk, comment_sk).await?;

    Ok(comments.into())
}
