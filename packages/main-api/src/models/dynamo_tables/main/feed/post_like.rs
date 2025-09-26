use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PostLike {
    pub pk: Partition,
    pub sk: EntityType,
}

impl PostLike {
    pub fn new(pk: Partition, User { pk: user_pk, .. }: User) -> Self {
        Self {
            pk,
            sk: EntityType::PostLike(user_pk.to_string()),
        }
    }
}
