use crate::Error2 as Error;
use crate::models::dynamo_tables::main::user::user::User;
use crate::models::dynamo_tables::main::user::user_membership::UserMembership;
use crate::models::dynamo_tables::main::user::user_principal::{
    UserPrincipal, UserPrincipalQueryOption,
};
use crate::types::*;
use bdk::prelude::by_axum::auth::Authorization;
use bdk::prelude::*;
use std::sync::Arc;

type Result<T> = std::result::Result<T, Error>;

pub async fn extract_user_with_allowing_anonymous(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
) -> Result<User> {
    let user = match auth {
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized("Failed to get principal from signature".to_string())
            })?;
            match get_user_by_principal(ddb, &principal).await {
                Ok(Some(user)) => user,
                Ok(None) | Err(_) => {
                    create_user(
                        ddb,
                        principal.clone(),
                        principal.clone(),
                        principal.clone(),
                        "".to_string(),
                        false,
                        false,
                        UserType::Anonymous,
                        None,
                        principal.clone(),
                        "".to_string(),
                        Membership::Free,
                        None,
                        "".to_string(),
                        None,
                        None,
                    )
                    .await?
                }
            }
        }
        _ => return extract_user(ddb, auth).await,
    };

    tracing::debug!("authorized user_id: {:?}", user);

    Ok(user)
}

pub async fn extract_user(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
) -> Result<User> {
    extract_user_with_options(ddb, auth, false).await
}

pub async fn extract_user_with_options(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
    with_groups: bool,
) -> Result<User> {
    let user = match auth {
        Some(Authorization::Session(_session)) => {
            // For session-based auth, we need to query by a different method
            // since session.user_id is i64 but DynamoDB User uses UUID
            // This would need additional mapping logic
            return Err(Error::Unknown(
                "Session-based auth not yet implemented for DynamoDB".to_string(),
            ));
        }
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized("Unauthorized access".to_string())
            })?;
            let user = get_user_by_principal(ddb, &principal)
                .await?
                .ok_or_else(|| {
                    tracing::error!("failed to get user by principal");
                    Error::InvalidUser
                })?;

            if with_groups {
                // TODO: Load groups when needed
            }
            user
        }
        Some(Authorization::Bearer { claims }) => {
            let user_id = claims.sub.clone();
            let user_pk = if user_id.starts_with("USER#") {
                user_id
            } else {
                format!("USER#{}", user_id)
            };
            let user = get_user_by_pk(ddb, &user_pk).await?.ok_or_else(|| {
                tracing::error!("failed to get user by bearer token");
                Error::InvalidUser
            })?;

            if with_groups {
                // TODO: Load groups when needed
            }
            user
        }
        _ => {
            return Err(Error::Unauthorized("Unauthorized access".to_string()));
        }
    };

    tracing::debug!("authorized user_id: {:?}", user);

    Ok(user)
}

pub async fn extract_user_id_with_no_error(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
) -> String {
    let user_id = match auth {
        Some(Authorization::Session(_session)) => {
            // Session-based auth not implemented for DynamoDB yet
            return "".to_string();
        }

        Some(Authorization::UserSig(sig)) => {
            let principal = match sig.principal() {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("failed to get principal: {:?}", e);
                    return "".to_string();
                }
            };

            match get_user_by_principal(ddb, &principal).await {
                Ok(Some(user)) => extract_uuid_from_pk(&user.pk.to_string()),
                Ok(None) => {
                    tracing::error!("user not found for principal");
                    "".to_string()
                }
                Err(e) => {
                    tracing::error!("failed to get user: {:?}", e);
                    "".to_string()
                }
            }
        }

        Some(Authorization::Bearer { claims }) => claims.sub.clone(),

        _ => "".to_string(),
    };

    tracing::debug!("authorized user_id: {:?}", user_id);
    user_id
}

pub async fn extract_user_id(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
) -> Result<String> {
    let user_id = match auth {
        Some(Authorization::Session(_session)) => {
            // Session-based auth not implemented for DynamoDB yet
            return Err(Error::Unknown(
                "Session-based auth not yet implemented for DynamoDB".to_string(),
            ));
        }
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized("Unauthorized access".to_string())
            })?;
            let user = get_user_by_principal(ddb, &principal)
                .await?
                .ok_or_else(|| {
                    tracing::error!("failed to get user by principal");
                    Error::InvalidUser
                })?;
            extract_uuid_from_pk(&user.pk.to_string())
        }
        Some(Authorization::Bearer { claims }) => claims.sub.clone(),
        _ => {
            return Err(Error::Unauthorized("Unauthorized access".to_string()));
        }
    };

    tracing::debug!("authorized user_id: {:?}", user_id);

    Ok(user_id)
}

pub async fn extract_user_email(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
) -> Result<String> {
    let email = match auth {
        Some(Authorization::Session(_session)) => {
            // Session-based auth not implemented for DynamoDB yet
            return Err(Error::Unknown(
                "Session-based auth not yet implemented for DynamoDB".to_string(),
            ));
        }
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized("Unauthorized access".to_string())
            })?;
            get_user_by_principal(ddb, &principal)
                .await?
                .ok_or_else(|| {
                    tracing::error!("failed to get user by principal");
                    Error::InvalidUser
                })?
                .email
        }
        Some(Authorization::Bearer { ref claims }) => match claims.custom.get("email") {
            Some(email) => email.clone(),
            None => extract_user(ddb, auth).await?.email,
        },
        _ => return Err(Error::Unauthorized("Unauthorized access".to_string())),
    };

    Ok(email)
}

pub async fn extract_principal(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    auth: Option<Authorization>,
) -> Result<String> {
    tracing::debug!("auth: {:?}", auth);
    let principal = match auth {
        Some(Authorization::Session(_session)) => {
            // Session-based auth not implemented for DynamoDB yet
            return Err(Error::Unknown(
                "Session-based auth not yet implemented for DynamoDB".to_string(),
            ));
        }
        Some(Authorization::UserSig(sig)) => sig.principal().map_err(|e| {
            tracing::error!("failed to get principal: {:?}", e);
            Error::Unauthorized("Unauthorized access".to_string())
        })?,
        Some(Authorization::Bearer { claims }) => {
            let user_id = claims.sub.clone();
            let user_pk = if user_id.starts_with("USER#") {
                user_id
            } else {
                format!("USER#{}", user_id)
            };

            // Verify user exists first
            let _user = get_user_by_pk(ddb, &user_pk).await?.ok_or_else(|| {
                tracing::error!("failed to get user by bearer token");
                Error::InvalidUser
            })?;


            //FIXME: if needed
            claims.custom.get("principal").cloned().ok_or_else(|| {
                tracing::error!("Principal not found in Bearer token claims");
                Error::Unauthorized("Principal not available in token".to_string())
            })?
        }
        _ => return Err(Error::Unauthorized("Unauthorized access".to_string())),
    };

    Ok(principal)
}

pub async fn get_user_by_pk(ddb: &Arc<aws_sdk_dynamodb::Client>, pk: &str) -> Result<Option<User>> {
    User::get(ddb, pk, Some(&EntityType::User.to_string()))
        .await
        .map_err(|e| Error::Unknown(format!("Failed to get user: {}", e)))
}

async fn get_user_by_principal(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    principal: &str,
) -> Result<Option<User>> {
    // Use the UserPrincipal::find_by_principal GSI query to find the user
    let (user_principals, _) =
        UserPrincipal::find_by_principal(ddb, principal, UserPrincipalQueryOption::builder())
            .await
            .map_err(|e| Error::Unknown(format!("Failed to query user principal: {}", e)))?;

    if let Some(user_principal) = user_principals.first() {
        // Get the user using the pk from the UserPrincipal record
        get_user_by_pk(ddb, &user_principal.pk.to_string()).await
    } else {
        Ok(None)
    }
}

async fn create_user(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    display_name: String,
    email: String,
    profile_url: String,
    _html_contents: String,
    term_agreed: bool,
    informed_agreed: bool,
    user_type: UserType,
    _parent_id: Option<String>,
    username: String,
    password: String,
    membership: Membership,
    _theme: Option<Theme>,
    principal: String,
    _referral_code: Option<String>,
    _referred_by: Option<String>,
) -> Result<User> {
    let user = User::new(
        display_name,
        email,
        profile_url,
        term_agreed,
        informed_agreed,
        user_type,
        username,
        Some(password),
    );

    let user_id = extract_uuid_from_pk(&user.pk.to_string());

    // Create UserPrincipal record
    let user_principal = UserPrincipal::new(user.pk.clone(), principal);

    // Create UserMembership record
    let user_membership = UserMembership::from_membership(user_id, membership);

    // Save all records using the models' create methods
    user.create(ddb)
        .await
        .map_err(|e| Error::Unknown(format!("Failed to create user: {}", e)))?;

    user_principal
        .create(ddb)
        .await
        .map_err(|e| Error::Unknown(format!("Failed to create user principal: {}", e)))?;

    user_membership
        .create(ddb)
        .await
        .map_err(|e| Error::Unknown(format!("Failed to create user membership: {}", e)))?;

    Ok(user)
}

pub fn extract_uuid_from_pk(pk: &str) -> String {
    pk.strip_prefix("USER#")
        .map(|id_str| id_str.to_string())
        .unwrap_or_else(|| "".to_string())
}

/// Get user membership by user pk
pub async fn get_user_membership_by_pk(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    user_pk: &str,
) -> Result<Option<UserMembership>> {
    UserMembership::get(ddb, user_pk, Some(&EntityType::UserMembership.to_string()))
        .await
        .map_err(|e| Error::Unknown(format!("Failed to get user membership: {}", e)))
}

/// Get user membership by user id (strips USER# prefix if present)
pub async fn get_user_membership_by_user_id(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    user_id: &str,
) -> Result<Option<UserMembership>> {
    let user_pk = if user_id.starts_with("USER#") {
        user_id.to_string()
    } else {
        format!("USER#{}", user_id)
    };
    get_user_membership_by_pk(ddb, &user_pk).await
}

/// Get or create user membership (creates Free tier by default)
pub async fn get_or_create_user_membership(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    user_id: &str,
) -> Result<UserMembership> {
    let user_pk = if user_id.starts_with("USER#") {
        user_id.to_string()
    } else {
        format!("USER#{}", user_id)
    };

    if let Some(membership) = get_user_membership_by_pk(ddb, &user_pk).await? {
        return Ok(membership);
    }

    // Create new Free membership using builder pattern
    let uuid = extract_uuid_from_pk(&user_pk);
    let membership = UserMembership::builder(uuid).with_free().build();

    membership
        .create(ddb)
        .await
        .map_err(|e| Error::Unknown(format!("Failed to create user membership: {}", e)))?;

    Ok(membership)
}

/// Update user membership
pub async fn update_user_membership(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    membership: &UserMembership,
) -> Result<()> {
    UserMembership::updater(membership.pk.clone(), membership.sk.clone())
        .with_membership_type(membership.membership_type)
        .with_subscription_start(membership.subscription_start)
        .with_subscription_end(membership.subscription_end)
        .with_space_capabilities(membership.space_capabilities.clone())
        .with_updated_at(crate::utils::time::get_now_timestamp_millis())
        .execute(ddb)
        .await
        .map_err(|e| Error::Unknown(format!("Failed to update user membership: {}", e)))
}
