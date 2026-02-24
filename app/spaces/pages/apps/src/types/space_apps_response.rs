use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct InstalledAppResponse {
    pub name: SpaceAppName,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct GetSpaceAppsResponse {
    pub apps: Vec<InstalledAppResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SpaceAppMutationResponse {
    pub name: SpaceAppName,

    pub installed: bool,
}
