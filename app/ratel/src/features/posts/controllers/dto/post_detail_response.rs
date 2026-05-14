use std::collections::HashSet;

use super::*;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::features::social::pages::member::dto::TeamRole;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct PostDetailResponse {
    pub post: Option<Post>,
    pub comments: Vec<PostCommentResponse>,
    pub artwork_metadata: Vec<PostArtworkMetadata>,
    pub repost: Option<PostRepost>,
    pub is_liked: bool,
    pub is_report: bool,
    /// Legacy bitmask. Kept for backward compatibility — new client code
    /// should consume `viewer_role` (Spaces-style direct role check).
    pub permissions: i64,
    /// Viewer's role with respect to this post's author. `None` when the
    /// viewer is not the author and not a member of the author's team
    /// (i.e. an outsider). Mirrors the Spaces pattern of shipping the
    /// resolved role to the client instead of a permission bitmask.
    #[serde(default)]
    pub viewer_role: Option<TeamRole>,
    /// True when the logged-in viewer is the post's individual author.
    /// Independent of `viewer_role` (which describes team membership);
    /// individual posts only ever set this and leave `viewer_role = None`.
    #[serde(default)]
    pub is_post_owner: bool,
}

impl From<Vec<PostMetadata>> for PostDetailResponse {
    fn from(items: Vec<PostMetadata>) -> Self {
        let mut res = Self::default();

        for item in items {
            match item {
                PostMetadata::Post(post) => res.post = Some(post),
                PostMetadata::PostComment(comment) => res.comments.push(comment.into()),
                PostMetadata::PostArtwork(artwork) => res.artwork_metadata = artwork.metadata,
                PostMetadata::PostRepost(repost) => res.repost = Some(repost),
            }
        }
        res
    }
}

impl
    From<(
        Vec<PostMetadata>,
        i64,
        bool,
        bool,
        Vec<PostCommentLike>,
        HashSet<String>,
    )> for PostDetailResponse
{
    fn from(
        (items, perms, is_liked, is_report, post_comment_likes, reported_comment_ids): (
            Vec<PostMetadata>,
            i64,
            bool,
            bool,
            Vec<PostCommentLike>,
            HashSet<String>,
        ),
    ) -> Self {
        let mut res = Self::default();
        res.permissions = perms;
        res.is_liked = is_liked;
        res.is_report = is_report;

        for item in items {
            match item {
                PostMetadata::Post(post) => res.post = Some(post),
                PostMetadata::PostComment(comment) => {
                    let liked = post_comment_likes.iter().any(|like| like == &comment);

                    let sk_str = comment.sk.to_string();
                    let is_comment_reported = reported_comment_ids.contains(&sk_str);

                    res.comments
                        .push((comment, liked, is_comment_reported).into());
                }
                PostMetadata::PostArtwork(artwork) => {
                    res.artwork_metadata = artwork.metadata;
                }
                PostMetadata::PostRepost(repost) => res.repost = Some(repost),
            }
        }

        res
    }
}
