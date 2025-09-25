use super::Post;
use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PostRepost {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "USER_PK", name = "find_by_post_pk", index = "gsi2", pk)]
    pub post_pk: Partition,
    pub post_title: String,
    pub post_html_contents: String,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub author_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
}

impl PostRepost {
    pub fn new(
        pk: Partition,
        Post {
            pk: post_pk,
            title: post_title,
            html_contents: post_html_contents,
            ..
        }: Post,
        User {
            pk: author_pk,
            display_name: author_display_name,
            profile_url: author_profile_url,
            ..
        }: User,
    ) -> Self {
        Self {
            pk,
            sk: EntityType::PostRepost,

            post_pk,
            post_title,
            post_html_contents,
            author_pk,
            author_display_name,
            author_profile_url,
        }
    }
}
