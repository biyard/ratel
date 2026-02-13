use common::SpaceUserRole;
use dioxus::prelude::*;

#[get("/api/spaces/:space_id/user-role")]
pub async fn get_user_role_in_space(
    space_id: String,
) -> Result<SpaceUserRole, ServerFnError> {
    #[cfg(feature = "server")]
    {
        // TODO: Implement actual role checking with common/session
        
        Ok(SpaceUserRole::Creator)
    }
}

#[get("/api/spaces/:space_id/dashboard-extensions")]
pub async fn fetch_dashboard_extensions(
    space_id: String,
) -> Result<Vec<crate::types::DashboardExtension>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let role = get_user_role_in_space(space_id).await?;

        // TODO: Replace with actual DynamoDB query in common/dynamodb

        let mock_data = match role {
            SpaceUserRole::Creator => include_str!("../assets/mock/creator_extensions.json"),
            SpaceUserRole::Participant => include_str!("../assets/mock/participant_extensions.json"),
            SpaceUserRole::Candidate => include_str!("../assets/mock/candidate_extensions.json"),
            SpaceUserRole::Viewer => include_str!("../assets/mock/viewer_extensions.json"),
        };
        
        serde_json::from_str(mock_data)
            .map_err(|e| ServerFnError::new(format!("Failed to parse mock data: {}", e)))
    }
}
