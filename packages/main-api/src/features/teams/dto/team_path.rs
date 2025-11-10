use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct TeamPathParam {
    #[schemars(description = "The unique identifier for a team")]
    pub team_pk: Partition,
}

pub type TeamPath = Path<TeamPathParam>;
