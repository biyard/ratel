use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_dynamodb::types::AttributeValue;
use bdk::prelude::by_axum::auth::Authorization;
use bdk::prelude::*;
use dto::*;
use serde_dynamo::{from_item, to_item};
use std::collections::HashMap;

use crate::models::dynamo_tables::main::user::{User, UserPrincipal};
use crate::types::{Partition, UserType};

pub async fn extract_user_with_allowing_anonymous_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    auth: Option<Authorization>,
) -> Result<User> {
    let user = match auth {
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;

            match find_user_by_principal_dynamo(dynamo_client, table_name, &principal).await {
                Ok(Some(user)) => user,
                Ok(None) | Err(_) => {
                    let uid = uuid::Uuid::new_v4().to_string();
                    let user = User::new(
                        principal.clone(),
                        principal.clone(),
                        principal.clone(),
                        false,
                        false,
                        UserType::Anonymous,
                        None,
                        principal.clone(),
                        "".to_string(),
                    );

                    let user_principal =
                        UserPrincipal::new(Partition::User(uid.clone()), principal.clone());

                    let user_item = to_item(&user)
                        .map_err(|e| Error::Unknown(format!("Failed to serialize user: {}", e)))?;

                    let principal_item = to_item(&user_principal).map_err(|e| {
                        Error::Unknown(format!("Failed to serialize user principal: {}", e))
                    })?;

                    put_items_transact_dynamo(
                        dynamo_client,
                        table_name,
                        vec![user_item, principal_item],
                    )
                    .await?;
                    user
                }
            }
        }
        _ => return extract_user_dynamo(dynamo_client, table_name, auth).await,
    };

    tracing::debug!("authorized user: {:?}", user.pk);
    Ok(user)
}

pub async fn extract_user_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    auth: Option<Authorization>,
) -> Result<User> {
    let user = match auth {
        Some(Authorization::Session(session)) => {
            let user_id = session.user_id.to_string();
            find_user_by_id_dynamo(dynamo_client, table_name, &user_id)
                .await?
                .ok_or_else(|| {
                    tracing::error!("User not found for session user_id: {}", user_id);
                    Error::InvalidUser
                })?
        }
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;
            find_user_by_principal_dynamo(dynamo_client, table_name, &principal)
                .await?
                .ok_or_else(|| {
                    tracing::error!("User not found for principal: {}", principal);
                    Error::InvalidUser
                })?
        }
        Some(Authorization::Bearer { claims }) => {
            let user_id = claims.sub.parse::<String>().map_err(|e| {
                tracing::error!("failed to parse user id: {:?}", e);
                Error::Unauthorized
            })?;
            find_user_by_id_dynamo(dynamo_client, table_name, &user_id)
                .await?
                .ok_or_else(|| {
                    tracing::error!("User not found for bearer user_id: {}", user_id);
                    Error::InvalidUser
                })?
        }
        _ => return Err(Error::Unauthorized),
    };

    Ok(user)
}

pub async fn extract_user_id_with_no_error_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    auth: Option<Authorization>,
) -> String {
    let user_id = match auth {
        Some(Authorization::Session(session)) => session.user_id.to_string(),
        Some(Authorization::UserSig(sig)) => {
            let principal = match sig.principal() {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("failed to get principal: {:?}", e);
                    return "0".to_string();
                }
            };

            match find_user_by_principal_dynamo(dynamo_client, table_name, &principal).await {
                Ok(Some(user)) => extract_user_id_from_partition(&user.pk),
                _ => {
                    tracing::error!("failed to get user for principal: {}", principal);
                    "0".to_string()
                }
            }
        }
        Some(Authorization::Bearer { claims }) => match claims.sub.parse::<String>() {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("failed to parse user id: {:?}", e);
                "0".to_string()
            }
        },
        _ => "0".to_string(),
    };

    tracing::debug!("authorized user_id: {:?}", user_id);
    user_id
}

pub async fn extract_user_id_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    auth: Option<Authorization>,
) -> Result<String> {
    let user_id = match auth {
        Some(Authorization::Session(session)) => session.user_id.to_string(),
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;
            let user = find_user_by_principal_dynamo(dynamo_client, table_name, &principal)
                .await?
                .ok_or_else(|| {
                    tracing::error!("User not found for principal: {}", principal);
                    Error::InvalidUser
                })?;
            extract_user_id_from_partition(&user.pk)
        }
        Some(Authorization::Bearer { claims }) => claims.sub.parse::<String>().map_err(|e| {
            tracing::error!("failed to parse user id: {:?}", e);
            Error::Unauthorized
        })?,
        _ => return Err(Error::Unauthorized),
    };

    tracing::debug!("authorized user_id: {:?}", user_id);
    Ok(user_id)
}

pub async fn extract_user_email_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    auth: Option<Authorization>,
) -> Result<String> {
    let email = match auth {
        Some(Authorization::Session(session)) => session.email,
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;
            let user = find_user_by_principal_dynamo(dynamo_client, table_name, &principal)
                .await?
                .ok_or_else(|| {
                    tracing::error!("User not found for principal: {}", principal);
                    Error::InvalidUser
                })?;
            user.email
        }
        Some(Authorization::Bearer { ref claims }) => match claims.custom.get("email") {
            Some(email) => email.clone(),
            None => {
                let user = extract_user_dynamo(dynamo_client, table_name, auth).await?;
                user.email
            }
        },
        _ => return Err(Error::Unauthorized),
    };

    Ok(email)
}

pub async fn extract_principal_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    auth: Option<Authorization>,
) -> Result<String> {
    tracing::debug!("auth: {:?}", auth);
    let principal = match auth {
        Some(Authorization::Session(session)) => session.principal,
        Some(Authorization::UserSig(sig)) => sig.principal().map_err(|e| {
            tracing::error!("failed to get principal: {:?}", e);
            Error::Unauthorized
        })?,
        Some(Authorization::Bearer { claims }) => {
            let user_id = claims.sub.parse::<String>().map_err(|e| {
                tracing::error!("failed to parse user id: {:?}", e);
                Error::Unauthorized
            })?;
            let user = find_user_by_id_dynamo(dynamo_client, table_name, &user_id)
                .await?
                .ok_or_else(|| {
                    tracing::error!("User not found for user_id: {}", user_id);
                    Error::InvalidUser
                })?;
            find_principal_by_user_dynamo(dynamo_client, table_name, &user.pk)
                .await?
                .ok_or_else(|| {
                    tracing::error!("Principal not found for user: {:?}", user.pk);
                    Error::InvalidUser
                })?
        }
        _ => return Err(Error::Unauthorized),
    };

    Ok(principal)
}

async fn find_user_by_id_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    user_id: &str,
) -> Result<Option<User>> {
    let pk_value = format!("USER#{}", user_id);
    let sk_value = "USER".to_string();

    let mut key = HashMap::new();
    key.insert("pk".to_string(), AttributeValue::S(pk_value));
    key.insert("sk".to_string(), AttributeValue::S(sk_value));

    let resp = dynamo_client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await
        .map_err(|e| Error::Unknown(format!("DynamoDB get_item failed: {}", e)))?;

    match resp.item {
        Some(item) => {
            let user: User = from_item(item)
                .map_err(|e| Error::Unknown(format!("Failed to deserialize user: {}", e)))?;
            Ok(Some(user))
        }
        None => Ok(None),
    }
}

async fn find_user_by_principal_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    principal: &str,
) -> Result<Option<User>> {
    let pk_value = format!("PRINCIPAL#{}", principal);

    let mut expression_values = HashMap::new();
    expression_values.insert(":pk".to_string(), AttributeValue::S(pk_value));

    let resp = dynamo_client
        .query()
        .table_name(table_name)
        .index_name("gsi1")
        .key_condition_expression("gsi1pk = :pk")
        .set_expression_attribute_values(Some(expression_values))
        .send()
        .await
        .map_err(|e| Error::Unknown(format!("DynamoDB query failed: {}", e)))?;

    if let Some(items) = resp.items {
        if let Some(item) = items.first() {
            let user_principal: UserPrincipal = from_item(item.clone()).map_err(|e| {
                Error::Unknown(format!("Failed to deserialize user principal: {}", e))
            })?;

            return find_user_by_partition_dynamo(dynamo_client, table_name, &user_principal.pk)
                .await;
        }
    }

    Ok(None)
}

async fn find_user_by_partition_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    partition: &Partition,
) -> Result<Option<User>> {
    let pk_value = partition.to_string();
    let sk_value = "USER".to_string();

    let mut key = HashMap::new();
    key.insert("pk".to_string(), AttributeValue::S(pk_value));
    key.insert("sk".to_string(), AttributeValue::S(sk_value));

    let resp = dynamo_client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await
        .map_err(|e| Error::Unknown(format!("DynamoDB get_item failed: {}", e)))?;

    match resp.item {
        Some(item) => {
            let user: User = from_item(item)
                .map_err(|e| Error::Unknown(format!("Failed to deserialize user: {}", e)))?;
            Ok(Some(user))
        }
        None => Ok(None),
    }
}

async fn find_principal_by_user_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    user_partition: &Partition,
) -> Result<Option<String>> {
    let pk_value = user_partition.to_string();
    let sk_value = "USER_PRINCIPAL".to_string();

    let mut key = HashMap::new();
    key.insert("pk".to_string(), AttributeValue::S(pk_value));
    key.insert("sk".to_string(), AttributeValue::S(sk_value));

    let resp = dynamo_client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await
        .map_err(|e| Error::Unknown(format!("DynamoDB get_item failed: {}", e)))?;

    match resp.item {
        Some(item) => {
            let user_principal: UserPrincipal = from_item(item).map_err(|e| {
                Error::Unknown(format!("Failed to deserialize user principal: {}", e))
            })?;
            Ok(Some(user_principal.principal))
        }
        None => Ok(None),
    }
}

async fn put_items_transact_dynamo(
    dynamo_client: &DynamoClient,
    table_name: &str,
    items: Vec<HashMap<String, AttributeValue>>,
) -> Result<()> {
    use aws_sdk_dynamodb::types::{Put, TransactWriteItem};

    let mut transact_items = Vec::new();
    for item in items {
        let put = Put::builder()
            .table_name(table_name)
            .set_item(Some(item))
            .build()
            .map_err(|e| Error::Unknown(format!("Failed to build put item: {}", e)))?;

        transact_items.push(TransactWriteItem::builder().put(put).build());
    }

    dynamo_client
        .transact_write_items()
        .set_transact_items(Some(transact_items))
        .send()
        .await
        .map_err(|e| Error::Unknown(format!("DynamoDB transact_write failed: {}", e)))?;

    Ok(())
}

fn extract_user_id_from_partition(partition: &Partition) -> String {
    match partition {
        Partition::User(id) => id.clone(),
        _ => "0".to_string(),
    }
}
