use crate::common::*;

/// Polymorphic recipient of the space creator 10% fee — either a single
/// user or an entire team.
///
/// The recipient is frozen at space creation time; reassigning space
/// ownership is out of V1 scope. For `User` recipients, the creator share
/// also flows into their `UserGlobalXp` aggregate so individual creators
/// level up from their own spaces. For `Team` recipients, earnings accrue
/// on the team entity; team-internal distribution is out of V1 scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(tag = "type", content = "id")]
pub enum CreatorRecipient {
    User(UserPartition),
    Team(TeamPartition),
}

impl Default for CreatorRecipient {
    fn default() -> Self {
        Self::User(UserPartition::default())
    }
}
