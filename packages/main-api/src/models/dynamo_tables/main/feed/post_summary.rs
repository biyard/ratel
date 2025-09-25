use super::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PostMetadata {
    Post(Post),
    PostAuthor(PostAuthor),
    PostComment(PostComment),
    PostArtwork(PostArtwork),
    PostRepost(PostRepost),
}

#[derive(Default, Debug, Clone, serde::Serialize, JsonSchema)]
pub struct PostDetailResponse {
    #[serde(flatten)]
    pub post: Post,
    pub author: PostAuthor,
    pub comments: Vec<PostComment>,
    pub artwork_metadata: Vec<PostArtworkMetadata>,
    pub repost: Option<PostRepost>,
    pub is_liked: bool, // Should be set externally
}

impl From<Vec<PostMetadata>> for PostDetailResponse {
    fn from(items: Vec<PostMetadata>) -> Self {
        let mut res = Self::default();
        for item in items {
            match item {
                PostMetadata::Post(post) => res.post = post,
                PostMetadata::PostAuthor(author) => res.author = author,
                PostMetadata::PostComment(comment) => res.comments.push(comment),
                PostMetadata::PostArtwork(artwork) => res.artwork_metadata = artwork.metadata,
                PostMetadata::PostRepost(repost) => res.repost = Some(repost),
            }
        }
        res
    }
}
