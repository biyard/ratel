use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct GetMeResponse {
    pub user: Option<User>,
}

#[get("/api/auth/me", user: OptionalUser)]
pub async fn get_me_handler() -> Result<GetMeResponse> {
    let user: Option<User> = user.into();
    Ok(GetMeResponse { user })
}
