use super::user::User;
use crate::types::UserType;
use crate::*;

#[cfg(feature = "server")]
use tower_sessions::Session;

pub struct AdminUser(pub User);

impl std::ops::Deref for AdminUser {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
    Session: FromRequestParts<S, Rejection: std::fmt::Debug>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let user = User::from_request_parts(parts, state)
            .await
            .map_err(|_| Error::NoSessionFound)?;

        if user.user_type != UserType::Admin {
            return Err(Error::NoPermission);
        }

        Ok(AdminUser(user))
    }
}
