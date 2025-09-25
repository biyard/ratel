use crate::Error2 as Error;
use crate::models::dynamo_tables::main::user::user::User;
use crate::models::dynamo_tables::main::user::user_principal::UserPrincipal;
use crate::types::*;
use crate::utils::aws::dynamo::DynamoClient;
use aws_sdk_dynamodb::types::AttributeValue;
use bdk::prelude::by_axum::auth::Authorization;
use bdk::prelude::*;
use serde_dynamo::{from_item, to_item};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Error>;

pub async fn extract_user_with_allowing_anonymous(
    dynamo_client: &DynamoClient,
    auth: Option<Authorization>,
) -> Result<User> {
    let user = match auth {
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;
            match get_user_by_principal(dynamo_client, &principal).await {
                Ok(Some(user)) => user,
                Ok(None) | Err(_) => {
                    create_user(
                        dynamo_client,
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
        _ => return extract_user(dynamo_client, auth).await,
    };

    tracing::debug!("authorized user_id: {:?}", user);

    Ok(user)
}

pub async fn extract_user(
    dynamo_client: &DynamoClient,
    auth: Option<Authorization>,
) -> Result<User> {
    extract_user_with_options(dynamo_client, auth, false).await
}

pub async fn extract_user_with_options(
    dynamo_client: &DynamoClient,
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
                Error::Unauthorized
            })?;
            let user = get_user_by_principal(dynamo_client, &principal)
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
            let user = get_user_by_pk(dynamo_client, &user_pk)
                .await?
                .ok_or_else(|| {
                    tracing::error!("failed to get user by bearer token");
                    Error::InvalidUser
                })?;

            if with_groups {
                // TODO: Load groups when needed
            }
            user
        }
        _ => {
            return Err(Error::Unauthorized);
        }
    };

    tracing::debug!("authorized user_id: {:?}", user);

    Ok(user)
}

pub async fn extract_user_id_with_no_error(
    dynamo_client: &DynamoClient,
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

            match get_user_by_principal(dynamo_client, &principal).await {
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
    dynamo_client: &DynamoClient,
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
                Error::Unauthorized
            })?;
            let user = get_user_by_principal(dynamo_client, &principal)
                .await?
                .ok_or_else(|| {
                    tracing::error!("failed to get user by principal");
                    Error::InvalidUser
                })?;
            extract_uuid_from_pk(&user.pk.to_string())
        }
        Some(Authorization::Bearer { claims }) => claims.sub.clone(),
        _ => {
            return Err(Error::Unauthorized);
        }
    };

    tracing::debug!("authorized user_id: {:?}", user_id);

    Ok(user_id)
}

pub async fn extract_user_email(
    dynamo_client: &DynamoClient,
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
                Error::Unauthorized
            })?;
            get_user_by_principal(dynamo_client, &principal)
                .await?
                .ok_or_else(|| {
                    tracing::error!("failed to get user by principal");
                    Error::InvalidUser
                })?
                .email
        }
        Some(Authorization::Bearer { ref claims }) => match claims.custom.get("email") {
            Some(email) => email.clone(),
            None => extract_user(dynamo_client, auth).await?.email,
        },
        _ => return Err(Error::Unauthorized),
    };

    Ok(email)
}

pub async fn extract_principal(
    dynamo_client: &DynamoClient,
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
            Error::Unauthorized
        })?,
        Some(Authorization::Bearer { claims }) => {
            let user_id = claims.sub.clone();
            let user_pk = if user_id.starts_with("USER#") {
                user_id
            } else {
                format!("USER#{}", user_id)
            };
            get_user_by_pk(dynamo_client, &user_pk)
                .await?
                .ok_or_else(|| {
                    tracing::error!("failed to get user by bearer token");
                    Error::InvalidUser
                })?
                .display_name
        }
        _ => return Err(Error::Unauthorized),
    };

    Ok(principal)
}

async fn get_user_by_pk(dynamo_client: &DynamoClient, pk: &str) -> Result<Option<User>> {
    let item = dynamo_client
        .get_item("pk", pk, Some(("sk", "USER")))
        .await
        .map_err(|e| Error::Unknown(format!("Failed to get user: {}", e)))?;

    match item {
        Some(attrs) => {
            let user = deserialize_user_from_dynamo(attrs)?;
            Ok(Some(user))
        }
        None => Ok(None),
    }
}

async fn get_user_by_principal(
    dynamo_client: &DynamoClient,
    principal: &str,
) -> Result<Option<User>> {
    // First, find the user PK from UserPrincipal table
    let mut expression_values = HashMap::new();
    expression_values.insert(
        ":principal".to_string(),
        AttributeValue::S(format!("PRINCIPAL#{}", principal)),
    );

    let principal_items = dynamo_client
        .query_gsi("gsi1", "gsi1pk = :principal", expression_values)
        .await
        .map_err(|e| Error::Unknown(format!("Failed to query user principal: {}", e)))?;

    if let Some(principal_item) = principal_items.into_iter().next() {
        // Extract the user PK from the principal item
        let user_pk = principal_item
            .get("pk")
            .and_then(|v| v.as_s().ok())
            .ok_or_else(|| Error::Unknown("Missing pk in UserPrincipal".to_string()))?;

        // Now get the user with this PK
        get_user_by_pk(dynamo_client, user_pk).await
    } else {
        Ok(None)
    }
}

async fn create_user(
    dynamo_client: &DynamoClient,
    display_name: String,
    email: String,
    profile_url: String,
    _html_contents: String,
    term_agreed: bool,
    informed_agreed: bool,
    user_type: UserType,
    parent_id: Option<String>,
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
        parent_id,
        username,
        password,
    );

    // Create UserPrincipal record
    let user_principal = UserPrincipal::new(user.pk.clone(), principal);

    let user_item = serialize_user_to_dynamo(&user)?;
    let principal_item = serialize_user_principal_to_dynamo(&user_principal)?;

    // Use transaction to create both records atomically
    dynamo_client
        .transact_write(vec![user_item, principal_item])
        .await
        .map_err(|e| Error::Unknown(format!("Failed to create user: {}", e)))?;

    Ok(user)
}

fn serialize_user_to_dynamo(user: &User) -> Result<HashMap<String, AttributeValue>> {
    let mut item: HashMap<String, AttributeValue> =
        to_item(user).map_err(|e| Error::SerdeDynamo(e))?;

    // Add GSI fields that aren't part of the struct
    item.insert(
        "gsi1pk".to_string(),
        AttributeValue::S(format!("EMAIL#{}", user.email)),
    );
    item.insert(
        "gsi1sk".to_string(),
        AttributeValue::S(format!("TS#{}", user.created_at)),
    );

    item.insert(
        "gsi2pk".to_string(),
        AttributeValue::S(format!("USERNAME#{}", user.username)),
    );
    item.insert(
        "gsi2sk".to_string(),
        AttributeValue::S(format!("TS#{}", user.created_at)),
    );

    Ok(item)
}

fn deserialize_user_from_dynamo(item: HashMap<String, AttributeValue>) -> Result<User> {
    from_item(item).map_err(|e| Error::SerdeDynamo(e))
}

fn extract_uuid_from_pk(pk: &str) -> String {
    pk.strip_prefix("USER#")
        .map(|id_str| id_str.to_string())
        .unwrap_or_else(|| "".to_string())
}

fn serialize_user_principal_to_dynamo(
    user_principal: &UserPrincipal,
) -> Result<HashMap<String, AttributeValue>> {
    let mut item: HashMap<String, AttributeValue> =
        to_item(user_principal).map_err(|e| Error::SerdeDynamo(e))?;

    // Add GSI1 for principal lookup
    item.insert(
        "gsi1pk".to_string(),
        AttributeValue::S(format!("PRINCIPAL#{}", user_principal.principal)),
    );
    item.insert(
        "gsi1sk".to_string(),
        AttributeValue::S(user_principal.sk.to_string()),
    );

    Ok(item)
}
