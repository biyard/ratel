use bdk::prelude::*;

/// Represents where a file is displayed/used within a space
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    DynamoEnum,
    JsonSchema,
)]
pub enum FileLocation {
    /// File is shown in the Overview tab
    #[default]
    Overview,
    /// File is shown in the Board/Posts tab  
    Board,
    /// File is shown in the Files tab
    Files,
}

impl FileLocation {
    pub fn all() -> Vec<Self> {
        vec![Self::Overview, Self::Board, Self::Files]
    }
}
