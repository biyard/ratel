use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum)]
pub enum FileLinkTarget {
    #[default]
    Files,
    Overview,
    Board(String),
}
