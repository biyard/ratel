use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionAssemblyMemberRequest {
    /// Fetches assembly members by Assembly Open API.
    /// And update the information of the assembly members.
    FetchMembers,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionAssemblyMemberByIdRequest {
    /// Manually, update crypto stance.
    /// It will be utilized to update crypto stance by contact.
    UpdateCryptoStance(CryptoStance),
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CryptoStance {
    Supportive,
    Neutral,
    Against,
    #[default]
    NoStance,
}
