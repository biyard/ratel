use crate::*;

#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Default,
    // DynamoEnum,
    // JsonSchema,
    // OperationIo,
    Translate,
    PartialEq,
    Eq,
)]
pub enum SpaceUserRole {
    #[default]
    #[translate(ko = "뷰어")]
    Viewer,
    #[translate(ko = "참가자")]
    Participant,
    #[translate(ko = "참가후보")]
    Candidate,
    #[translate(ko = "관리자")]
    Creator,
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for SpaceUserRole
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        use crate::models::auth::User;
        use crate::models::space::SpaceCommon;

        tracing::debug!("extracting space from request parts");
        if let Some(space_role) = parts.extensions.get::<SpaceUserRole>() {
            return Ok(space_role.clone());
        }

        let _space = if let Some(space) = parts.extensions.get::<SpaceCommon>() {
            space.clone()
        } else {
            SpaceCommon::from_request_parts(parts, state).await?
        };

        let _user = if let Some(user) = parts.extensions.get::<User>() {
            user.clone()
        } else {
            User::from_request_parts(parts, state).await?
        };

        let default_role = SpaceUserRole::Viewer;

        Ok(default_role)
    }
}
