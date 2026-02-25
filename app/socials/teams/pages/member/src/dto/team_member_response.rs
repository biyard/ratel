use crate::dto::MemberGroup;
use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamMemberResponse {
    pub user_id: String,
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
    pub groups: Vec<MemberGroup>,
    pub is_owner: bool,
}
