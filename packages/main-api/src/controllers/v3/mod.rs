use crate::{AppState, Error2, models::user::User, types::*};
use axum::extract::State;
use bdk::prelude::*;

pub mod networks;

pub mod promotions {
    pub mod get_top_promotion;
}
pub mod me {
    pub mod get_info;
    pub mod update_user;

    pub mod list_my_drafts;
    pub mod list_my_posts;
    #[cfg(test)]
    pub mod tests;
}
pub mod users {
    pub mod find_user;

    #[cfg(test)]
    pub mod tests;
}

pub mod assets {
    pub mod complete_multipart_upload;
    pub mod get_put_multi_object_uri;
    pub mod get_put_object_uri;
}

pub mod auth {
    pub mod health;
    pub mod login;
    pub mod logout;
    pub mod signup;

    #[cfg(test)]
    pub mod tests;

    pub mod verification {
        pub mod send_code;
        pub mod verify_code;

        #[cfg(test)]
        pub mod tests;
    }
}

pub mod spaces;

pub mod teams {
    pub mod create_team;
    pub mod delete_team;
    pub mod find_team;
    pub mod get_team;
    pub mod list_members;
    pub mod list_team_posts;
    pub mod update_team;

    pub mod dto;
    #[cfg(test)]
    pub mod tests;

    pub mod groups {
        pub mod add_member;
        pub mod create_group;
        pub mod delete_group;
        pub mod remove_member;
        pub mod update_group;

        #[cfg(test)]
        pub mod tests;
    }
}

pub mod posts;

pub mod memberships;

/// Extract DynamoDB client from AppState
///
/// # Example
/// ```no_run
/// use crate::controllers::v3::extract_state;
/// use axum::extract::State;
///
/// async fn my_handler(state: State<AppState>) {
///     let cli = extract_state(state);
///     // Use cli for DynamoDB operations
/// }
/// ```
pub fn extract_state(State(AppState { dynamo, .. }): State<AppState>) -> aws_sdk_dynamodb::Client {
    dynamo.client
}

/// Verify that the current user is a ServiceAdmin
///
/// # Arguments
/// * `user` - Optional user from authentication middleware
/// * `cli` - DynamoDB client for querying ServiceAdmin records
///
/// # Returns
/// * `Ok(User)` - If user is authenticated and is a ServiceAdmin
/// * `Err(Error2::NoUserFound)` - If no user is authenticated
/// * `Err(Error2::NoPermission)` - If user is not a ServiceAdmin
///
/// # Example
/// ```no_run
/// use crate::controllers::v3::verify_service_admin;
/// use aide::NoApi;
/// use axum::{extract::State, Json};
///
/// async fn admin_only_handler(
///     state: State<AppState>,
///     NoApi(user): NoApi<Option<User>>,
/// ) -> Result<Json<Response>, Error2> {
///     let cli = &state.dynamo.client;
///
///     // Verify user is a ServiceAdmin
///     let admin_user = verify_service_admin(user, cli).await?;
///
///     // Continue with admin operations
///     Ok(Json(response))
/// }
/// ```
pub async fn verify_service_admin(
    user: Option<User>,
    _cli: &aws_sdk_dynamodb::Client,
) -> Result<User, Error2> {
    // Check if user is authenticated
    let user = user.ok_or(Error2::NoUserFound)?;

    if user.user_type == UserType::Admin {
        Ok(user)
    } else {
        Err(Error2::NoPermission)
    }
}
