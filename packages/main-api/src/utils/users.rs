use bdk::prelude::by_axum::auth::Authorization;
use bdk::prelude::*;
use dto::*;

pub async fn extract_user_with_allowing_anonymous(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
) -> Result<User> {
    let user = match auth {
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;
            match User::query_builder()
                .principal_equals(principal.clone())
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
            {
                Ok(user) => user,
                Err(_) => {
                    User::get_repository(pool.clone())
                        .insert(
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
        _ => return extract_user(pool, auth).await,
    };

    tracing::debug!("authorized user_id: {:?}", user);

    Ok(user)
}

pub async fn extract_user(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
) -> Result<User> {
    extract_user_with_options(pool, auth, false).await
}

pub async fn extract_user_with_options(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
    with_groups: bool,
) -> Result<User> {
    let user = match auth {
        Some(Authorization::Session(session)) => {
            let mut query = User::query_builder().id_equals(session.user_id);
            if with_groups {
                query = query.groups_builder(Group::query_builder());
            }
            query
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    Error::InvalidUser
                })?
        }
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;
            let mut query = User::query_builder().principal_equals(principal);
            if with_groups {
                query = query.groups_builder(Group::query_builder());
            }
            query
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    Error::InvalidUser
                })?
        }
        Some(Authorization::Bearer { claims }) => {
            let user_id = claims.sub.parse::<i64>().map_err(|e| {
                tracing::error!("failed to parse user id: {:?}", e);
                Error::Unauthorized
            })?;

            let mut query = User::query_builder().id_equals(user_id);
            if with_groups {
                query = query.groups_builder(Group::query_builder());
            }
            query
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    Error::InvalidUser
                })?
        }
        _ => {
            return Err(Error::Unauthorized);
        }
    };

    Ok(user)
}

pub async fn extract_user_id_with_no_error(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
) -> i64 {
    let user_id = match auth {
        Some(Authorization::Session(session)) => session.user_id,

        Some(Authorization::UserSig(sig)) => {
            let principal = match sig.principal() {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("failed to get principal: {:?}", e);
                    return 0;
                }
            };

            let result = User::query_builder()
                .principal_equals(principal)
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await;

            match result {
                Ok(user) => user.id,
                Err(e) => {
                    tracing::error!("failed to get user: {:?}", e);
                    0
                }
            }
        }

        Some(Authorization::Bearer { claims }) => match claims.sub.parse::<i64>() {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("failed to parse user id: {:?}", e);
                0
            }
        },

        _ => 0,
    };

    tracing::debug!("authorized user_id: {:?}", user_id);
    user_id
}

pub async fn extract_user_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
) -> Result<i64> {
    let user_id = match auth {
        Some(Authorization::Session(session)) => session.user_id,
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;
            User::query_builder()
                .principal_equals(principal)
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    Error::InvalidUser
                })?
                .id
        }
        Some(Authorization::Bearer { claims }) => claims.sub.parse::<i64>().map_err(|e| {
            tracing::error!("failed to parse user id: {:?}", e);
            Error::Unauthorized
        })?,
        _ => {
            return Err(Error::Unauthorized);
        }
    };

    tracing::debug!("authorized user_id: {:?}", user_id);

    Ok(user_id)
}

pub async fn extract_user_email(
    pool: &sqlx::Pool<sqlx::Postgres>,
    auth: Option<Authorization>,
) -> Result<String> {
    let email = match auth {
        Some(Authorization::Session(session)) => session.email,
        Some(Authorization::UserSig(sig)) => {
            let principal = sig.principal().map_err(|e| {
                tracing::error!("failed to get principal: {:?}", e);
                Error::Unauthorized
            })?;
            User::query_builder()
                .principal_equals(principal)
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    Error::InvalidUser
                })?
                .email
        }
        Some(Authorization::Bearer { ref claims }) => match claims.custom.get("email") {
            Some(email) => email.clone(),
            None => extract_user(pool, auth).await?.email,
        },
        _ => return Err(Error::Unauthorized),
    };

    Ok(email)
}

pub async fn extract_principal(
    pool: &sqlx::Pool<sqlx::Postgres>,
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
            let user_id = claims.sub.parse::<i64>().map_err(|e| {
                tracing::error!("failed to parse user id: {:?}", e);
                Error::Unauthorized
            })?;
            User::query_builder()
                .id_equals(user_id)
                .query()
                .map(User::from)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("failed to get user: {:?}", e);
                    Error::InvalidUser
                })?
                .principal
        }
        _ => return Err(Error::Unauthorized),
    };

    Ok(principal)
}
