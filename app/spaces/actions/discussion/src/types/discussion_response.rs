use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct DiscussionResponse {
    pub pk: Partition,

    pub created_at: i64,
    pub updated_at: i64,
    pub started_at: i64,
    pub ended_at: i64,
    pub title: String,
    pub html_contents: String,
    pub category_name: String,
    pub number_of_comments: i64,

    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub status: DiscussionStatus,
}

#[cfg(feature = "server")]
impl From<SpacePost> for DiscussionResponse {
    fn from(post: SpacePost) -> Self {
        let status = post.status();
        Self {
            pk: match post.sk {
                EntityType::SpacePost(v) => Partition::SpacePost(v),
                _ => Partition::SpacePost("".to_string()),
            },
            created_at: post.created_at,
            updated_at: post.updated_at,
            started_at: post.started_at,
            ended_at: post.ended_at,
            title: post.title,
            html_contents: post.html_contents,
            category_name: post.category_name,
            number_of_comments: post.comments,
            user_pk: post.user_pk,
            author_display_name: post.author_display_name,
            author_profile_url: post.author_profile_url,
            author_username: post.author_username,
            status,
        }
    }
}
