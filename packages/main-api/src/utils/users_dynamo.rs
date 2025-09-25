use crate::Error2 as Error;
use crate::models::dynamo_tables::main::user::user::User;
use crate::models::dynamo_tables::main::user::user_principal::UserPrincipal;
use crate::types::*;
use aws_sdk_dynamodb::types::AttributeValue;
use bdk::prelude::by_axum::auth::Authorization;
use bdk::prelude::*;
use std::collections::HashMap;
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
            get_user_by_pk(ddb, &user_pk)
                .await?
                .ok_or_else(|| {
                    tracing::error!("failed to get user by bearer token");
                    Error::InvalidUser
                })?
                .display_name
        }
        _ => return Err(Error::Unauthorized("Unauthorized access".to_string())),
    };

    Ok(principal)
}

pub async fn get_user_by_pk(ddb: &Arc<aws_sdk_dynamodb::Client>, pk: &str) -> Result<Option<User>> {
    User::get(ddb, pk, Some("USER"))
        .await
        .map_err(|e| Error::Unknown(format!("Failed to get user: {}", e)))
}

// FIXME
async fn get_user_by_principal(
    ddb: &Arc<aws_sdk_dynamodb::Client>,
    principal: &str,
) -> Result<Option<User>> {
    // For now, use a simple approach - query by the principal directly
    // This might need to be updated based on how the UserPrincipal model is structured
    let principal_key = format!("PRINCIPAL#{}", principal);

    // Try to find a UserPrincipal record that matches this principal
    // Since we don't have the exact query method available, let's keep the original logic for now
    let mut expression_values = HashMap::new();
    expression_values.insert(":principal".to_string(), AttributeValue::S(principal_key));

    // This would need to be replaced with the actual UserPrincipal query method
    // For now, let's use a placeholder that will work with the existing DynamoClient interface
    // In a real implementation, we'd use UserPrincipal::query_gsi1() or similar

    // For simplicity, let's just try to get the user directly by principal for now
    // This is a temporary solution until we can properly implement GSI queries with the User model
    User::get(ddb, &format!("USER#{}", principal), Some("USER"))
        .await
        .map_err(|e| Error::Unknown(format!("Failed to get user by principal: {}", e)))
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
    _membership: Membership,
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

    // Create UserPrincipal record
    let user_principal = UserPrincipal::new(user.pk.clone(), principal);

    // Save both records using the models' create methods
    user.create(ddb)
        .await
        .map_err(|e| Error::Unknown(format!("Failed to create user: {}", e)))?;

    user_principal
        .create(ddb)
        .await
        .map_err(|e| Error::Unknown(format!("Failed to create user principal: {}", e)))?;

    Ok(user)
}

fn extract_uuid_from_pk(pk: &str) -> String {
    pk.strip_prefix("USER#")
        .map(|id_str| id_str.to_string())
        .unwrap_or_else(|| "".to_string())
}
