use crate::*;

#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/comments/{comment_sk}/replies?bookmark", role: SpaceUserRole)]
pub async fn list_replies(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
    bookmark: Option<String>,
) -> Result<Vec<DiscussionCommentResponse>> {
    SpacePost::can_view(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let comment_sk_entity: EntityType = comment_sk.into();

    let opt = SpacePostComment::opt_all()
        .scan_index_forward(false)
        .limit(50);
    let opt = if let Some(b) = bookmark {
        opt.bookmark(b)
    } else {
        opt
    };

    let (replies, _next_bookmark) =
        SpacePostComment::list_by_comment(cli, comment_sk_entity, opt).await?;

    let responses: Vec<DiscussionCommentResponse> = replies.into_iter().map(|r| r.into()).collect();

    Ok(responses)
}
