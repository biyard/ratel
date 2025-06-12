use crate::by_axum::axum::extract::Path;
use crate::by_axum::axum::routing::post;
use crate::utils::middlewares::authorization_middleware;
use bdk::prelude::*;
use by_axum::auth::Authorization;
use by_axum::axum::{
    Extension, Json,
    extract::{Query, State},
    middleware,
    routing::get,
};
use by_types::QueryResponse;
use dto::*;
use rest_api::Signature;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgRow;
use tracing::instrument;
use validator::Validate;

use crate::utils::users::extract_user_id;

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    users: UserRepository,
    pool: Pool<Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct UserByIdPath {
    pub id: i64,
}

impl UserControllerV1 {
    pub fn route(pool: Pool<Postgres>) -> Result<by_axum::axum::Router> {
        let users = User::get_repository(pool.clone());

        let ctrl = UserControllerV1 { users, pool };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::read_user).post(Self::act_user))
            .route("/:id", post(Self::act_user_by_id))
            .route("/:id/followings", get(Self::get_followings))
            .route("/:id/followers", get(Self::get_followers))
            .with_state(ctrl.clone())
            .layer(middleware::from_fn(authorization_middleware)))
    }

    pub async fn act_user_by_id(
        State(ctrl): State<UserControllerV1>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(UserByIdPath { id }): Path<UserByIdPath>,
        Json(body): Json<UserByIdAction>,
    ) -> Result<Json<User>> {
        let user_id = extract_user_id(&ctrl.pool, auth).await?;

        let team = match TeamMember::query_builder()
            .team_id_equals(id)
            .user_id_equals(user_id)
            .query()
            .map(TeamMember::from)
            .fetch_one(&ctrl.pool)
            .await
        {
            Ok(v) => Some(v),
            Err(_) => None,
        };

        if user_id != id && team.is_none() {
            return Err(Error::Unauthorized);
        }

        match body {
            UserByIdAction::EditProfile(req) => ctrl.edit_profile(id, req).await,
        }
    }

    #[instrument]
    pub async fn act_user(
        State(ctrl): State<UserControllerV1>,
        Extension(sig): Extension<Option<Signature>>,
        Json(body): Json<UserAction>,
    ) -> Result<Json<User>> {
        tracing::debug!("act_user: sig={:?} {:?}", sig, body);
        let sig = sig.ok_or(Error::Unauthorized)?;
        body.validate()?;

        match body {
            UserAction::Signup(req) => ctrl.signup(req, sig).await,
            UserAction::UpdateEvmAddress(req) => ctrl.update_evm_address(req, sig).await,
        }
    }

    #[instrument]
    pub async fn read_user(
        State(ctrl): State<UserControllerV1>,
        Extension(sig): Extension<Option<Signature>>,

        Query(mut req): Query<UserReadAction>,
    ) -> Result<Json<User>> {
        tracing::debug!("read_user: sig={:?}", sig);
        let principal = sig.ok_or(Error::Unauthorized)?.principal().map_err(|s| {
            tracing::error!("failed to get principal: {:?}", s);
            Error::Unknown(s.to_string())
        })?;
        req.validate()?;

        match req.action {
            Some(UserReadActionType::FindByEmail) => ctrl.find_by_email(req).await,
            Some(UserReadActionType::CheckEmail) => ctrl.check_email(req).await,
            Some(UserReadActionType::UserInfo) => {
                req.principal = Some(principal);
                ctrl.user_info(req).await
            }
            Some(UserReadActionType::Login) => {
                req.principal = Some(principal);
                ctrl.login(req).await
            }
            None | Some(UserReadActionType::ByPrincipal) => Err(Error::BadRequest)?,
        }
    }
}

impl UserControllerV1 {
    pub async fn edit_profile(&self, id: i64, req: UserEditProfileRequest) -> Result<Json<User>> {
        let user = self
            .users
            .update(
                id,
                UserRepositoryUpdateRequest {
                    nickname: Some(req.nickname),
                    profile_url: Some(req.profile_url),
                    html_contents: Some(req.html_contents),
                    ..Default::default()
                },
            )
            .await?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn login(&self, req: UserReadAction) -> Result<Json<User>> {
        let user = self.users.find_one(&req).await?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn update_evm_address(
        &self,
        req: UserUpdateEvmAddressRequest,
        sig: Signature,
    ) -> Result<Json<User>> {
        let principal = sig.principal().map_err(|s| {
            tracing::error!("failed to get principal: {:?}", s);
            Error::Unauthorized
        })?;
        let user = User::query_builder()
            .principal_equals(principal.clone())
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await?;

        let user = self
            .users
            .update(
                user.id,
                UserRepositoryUpdateRequest::new().with_evm_address(req.evm_address),
            )
            .await?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn signup(&self, req: UserSignupRequest, sig: Signature) -> Result<Json<User>> {
        let principal = sig.principal().map_err(|s| {
            tracing::error!("failed to get principal: {:?}", s);
            Error::Unauthorized
        })?;

        if req.term_agreed == false {
            return Err(Error::BadRequest);
        }

        if let Ok(user) = User::query_builder()
            .principal_equals(principal.clone())
            .user_type_equals(UserType::Anonymous)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
        {
            let user = self
                .users
                .update(
                    user.id,
                    UserRepositoryUpdateRequest::new()
                        .with_email(req.email)
                        .with_nickname(req.nickname)
                        .with_profile_url(req.profile_url)
                        .with_term_agreed(req.term_agreed)
                        .with_informed_agreed(req.informed_agreed)
                        .with_username(req.username)
                        .with_evm_address(req.evm_address)
                        .with_user_type(UserType::Individual),
                )
                .await?;

            return Ok(Json(user));
        }

        let user = self
            .users
            .insert(
                req.nickname,
                principal,
                req.email,
                req.profile_url,
                req.term_agreed,
                req.informed_agreed,
                UserType::Individual,
                None,
                req.username,
                "".to_string(),
                req.evm_address,
            )
            .await?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn check_email(
        &self,
        UserReadAction { email, .. }: UserReadAction,
    ) -> Result<Json<User>> {
        let user = User::query_builder()
            .email_equals(email.ok_or(Error::InvalidEmail)?)
            .user_type_equals(UserType::Individual)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound)?;

        Ok(Json(user))
    }

    #[instrument]
    async fn find_by_email(
        &self,
        UserReadAction { email, .. }: UserReadAction,
    ) -> Result<Json<User>> {
        let user = User::query_builder()
            .email_equals(email.ok_or(Error::InvalidEmail)?)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound)?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn user_info(
        &self,
        UserReadAction { principal, .. }: UserReadAction,
    ) -> Result<Json<User>> {
        let user = User::query_builder()
            .principal_equals(principal.ok_or(Error::InvalidUser)?)
            .groups_builder(Group::query_builder())
            .user_type_equals(UserType::Individual)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound)?;

        Ok(Json(user))
    }

    #[instrument]
    pub async fn get_followings(
        State(ctrl): State<UserControllerV1>,
        Extension(_): Extension<Option<Authorization>>,
        Path(UserByIdPath { id }): Path<UserByIdPath>,

        Query(param): Query<UserQuery>,
    ) -> Result<Json<QueryResponse<User>>> {
        // Get paginated list of users that the given user is following
        let following_ids: Vec<i64> = Mynetwork::query_builder()
            .follower_id_equals(id)
            .limit(param.size())
            .page(param.page())
            .order_by_created_at_desc()
            .query()
            .map(|row: PgRow| {
                let network: Mynetwork = row.into();

                network.following_id
            })
            .fetch_all(&ctrl.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get following relationships: {:?}", e);

                Error::DatabaseException(e.to_string())
            })?;

        let total_count = following_ids.len() as i64;
        // Get the actual user details for the following IDs
        let users: Vec<User> = if following_ids.is_empty() {
            vec![]
        } else {
            // Create OR conditions for multiple IDs using BitOr operator

            let mut combined_query = None;


            for following_id in following_ids {
                let single_query = User::query_builder().id_equals(following_id);

                match combined_query {
                    None => combined_query = Some(single_query),

                    Some(existing_query) => combined_query = Some(existing_query | single_query),
                }
            }

            if let Some(query) = combined_query {
                query
                    .order_by_created_at_desc()
                    .query()
                    .map(User::from)
                    .fetch_all(&ctrl.pool)
                    .await
                    .map_err(|e| {
                        tracing::error!("failed to get users: {:?}", e);

                        Error::DatabaseException(e.to_string())
                    })?
            } else {
                vec![]
            }
        };

        Ok(Json(QueryResponse {
            items: users,
            total_count,
        }))
    }


    #[instrument]
    pub async fn get_followers(
        State(ctrl): State<UserControllerV1>,

        Extension(_): Extension<Option<Authorization>>,

        Path(UserByIdPath { id }): Path<UserByIdPath>,

        Query(param): Query<UserQuery>,
    ) -> Result<Json<QueryResponse<User>>> {
        // Get paginated list of users that are following the given user
        let follower_ids: Vec<i64> = Mynetwork::query_builder()
        .following_id_equals(id)
            .limit(param.size())
            .page(param.page())
            .order_by_created_at_desc()
            .query()
            .map(|row: PgRow| {                
                let network: Mynetwork = row.into();
                
                network.follower_id
            })
            .fetch_all(&ctrl.pool)
            .await
            .map_err(|e| {
                tracing::error!("failed to get follower relationships: {:?}", e);

                Error::DatabaseException(e.to_string())
            })?;
            
            
        let total_count = follower_ids.len() as i64;
            // Get the actual user details for the follower IDs
        let users: Vec<User> = if follower_ids.is_empty() {
            vec![]
        } else {
            // Create OR conditions for multiple IDs using BitOr operator

            let mut combined_query = None;

            for follower_id in follower_ids {
                let single_query = User::query_builder().id_equals(follower_id);

                match combined_query {
                    None => combined_query = Some(single_query),

                    Some(existing_query) => combined_query = Some(existing_query | single_query),
                }
            }

            if let Some(query) = combined_query {
                query
                    .order_by_created_at_desc()
                    .query()
                    .map(User::from)
                    .fetch_all(&ctrl.pool)
                    .await
                    .map_err(|e| {
                        tracing::error!("failed to get users: {:?}", e);

                        Error::DatabaseException(e.to_string())
                    })?
            } else {
                vec![]
            }
        };

        Ok(Json(QueryResponse {
            items: users,

            total_count,
        }))
    }
}
