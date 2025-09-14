use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PartitionKeyId {
    #[default]
    None,
    User(i64),
}

impl PartitionKeyId {
   pub fn new() -> Self {
       Self::None
   }
}
