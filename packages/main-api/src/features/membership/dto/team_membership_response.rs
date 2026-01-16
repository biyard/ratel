use crate::{features::membership::TeamMembership, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
pub struct TeamMembershipResponse {
    pub team_pk: String,
    pub tier: MembershipPartition,

    pub expired_at: i64,

    pub total_credits: i64,
    pub remaining_credits: i64,

    pub next_membership: Option<MembershipPartition>,

    /// Indicates if the current user is the team owner (can modify membership)
    pub is_owner: bool,
}

impl TeamMembershipResponse {
    pub fn from_team_membership(team_membership: TeamMembership, is_owner: bool) -> Self {
        Self {
            team_pk: team_membership.pk.to_string(),
            tier: team_membership.membership_pk,
            expired_at: team_membership.expired_at,
            total_credits: team_membership.total_credits,
            remaining_credits: team_membership.remaining_credits,
            next_membership: team_membership.next_membership,
            is_owner,
        }
    }
}
