use super::super::*;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "lowercase")]
pub enum TeamRole {
    #[translate(en = "Admin", ko = "관리자")]
    Admin,
    #[default]
    #[translate(en = "Member", ko = "멤버")]
    Member,
}

impl std::fmt::Display for TeamRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamRole::Admin => write!(f, "Admin"),
            TeamRole::Member => write!(f, "Member"),
        }
    }
}
