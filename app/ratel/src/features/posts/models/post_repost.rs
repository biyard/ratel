use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::features::auth::User;

use super::Post;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct PostRepost {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "POST_PK", name = "find_by_post_pk", index = "gsi2", pk)]
    pub post_pk: Partition,
    pub post_title: String,
    #[serde(alias = "post_html_contents")]
    pub post_body: ContentBody,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub author_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
}

#[cfg(feature = "server")]
impl PostRepost {
    pub fn new(
        pk: Partition,
        Post {
            pk: post_pk,
            title: post_title,
            body: post_body,
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
            post_body,
            author_pk,
            author_display_name,
            author_profile_url,
        }
    }
}
