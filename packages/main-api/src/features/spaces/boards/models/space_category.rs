use crate::{
    Error,
    models::{PostCommentLike, team::Team, user::User},
    types::{author::Author, *},
};
use bdk::prelude::*;

#[derive(
    Debug,
    Default,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceCategory {
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
}

impl SpaceCategory {
    pub fn new(space_pk: Partition, name: String) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpaceCategory(name.clone()),
            name,
        }
    }
}
