use super::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PostSummary {
    Post(Post),
    // PostAuthor(PostAuthor),
    PostComment(PostComment),
    PostArtwork(PostArtwork),
    PostRepost(PostRepost),
}
#[derive(Default, Debug, Clone, serde::Serialize)]
pub struct PostDetailResponse {
    #[serde(flatten)]
    pub post: Post,
    // #[serde(flatten)]
    // pub author: PostAuthor,
    pub comments: Vec<PostComment>,
    pub artwork_metadata: Vec<PostArtworkMetadata>,
    pub repost: Option<PostRepost>,
    pub is_liked: bool, // Should be set externally
}

impl From<Vec<PostSummary>> for PostDetailResponse {
    fn from(items: Vec<PostSummary>) -> Self {
        let mut res = Self::default();
        for item in items {
            match item {
                PostSummary::Post(post) => res.post = post,
                // PostSummary::PostAuthor(author) => res.author = author,
                PostSummary::PostComment(comment) => res.comments.push(comment),
                PostSummary::PostArtwork(artwork) => res.artwork_metadata = artwork.metadata,
                PostSummary::PostRepost(repost) => res.repost = Some(repost),
            }
        }
        res
    }
}
