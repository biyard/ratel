use super::*;
use bdk::prelude::*;

/*
FIXME: Remove PostComment / PostLike from PostMetadata

The reason PostComment and PostLike are included in the metadata is to make it easier to construct PostDetailResponse.

However, since the number of PostComment and PostLike can grow indefinitely,
including them in PostMetadata may cause the size to exceed 1MB, preventing normal creation of a Post.
Therefore, the PKs for PostComment and PostLike should be managed separately from the Post, and queries should also be performed separately.

However, if PostComment and PostLike are not included in PostDetailResponse, an error occurs with untagged serialization.
To prevent this, they are temporarily included in PostMetadata.

*/
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PostMetadata {
    Post(Post),
    PostAuthor(PostAuthor),
    PostComment(PostComment),
    PostArtwork(PostArtwork),
    PostRepost(PostRepost),
    PostLike(PostLike),
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
                _ => { /* Ignore PostLike in this context */ }
            }
        }
        res
    }
}
