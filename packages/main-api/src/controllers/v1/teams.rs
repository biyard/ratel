use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

use crate::utils::users::extract_user_id;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct TeamPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct TeamController {
    user: UserRepository,
    repo: TeamRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl TeamController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: TeamQuery,
    ) -> Result<QueryResponse<Team>> {
        let mut total_count = 0;
        let items: Vec<Team> = Team::query_builder()
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn has_permission(
        &self,
        auth: Option<Authorization>,
        team_id: i64,
    ) -> Result<(i64, Team)> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        let team = Team::query_builder()
            .id_equals(team_id)
            .query()
            .map(Team::from)
            .fetch_one(&self.pool)
            .await?;

        // FIXME: check if the user is a member of the team.
        if team.parent_id != user_id {
            return Err(ServiceError::Unauthorized);
        }

        Ok((user_id, team))
    }

    async fn create(
        &self,
        auth: Option<Authorization>,
        TeamCreateRequest {
            profile_url,
            username,
        }: TeamCreateRequest,
    ) -> Result<Team> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        let user = self
            .user
            .insert(
                "".to_string(),
                username.clone(),
                username.clone(),
                profile_url,
                false,
                false,
                UserType::Team,
                Some(user_id),
                username,
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to create team: {:?}", e);
                ServiceError::DuplicatedTeamName
            })?;

        Ok(user.into())
    }

    async fn update_profile_image(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: TeamUpdateProfileImageRequest,
    ) -> Result<Team> {
        let (_user_id, _team) = self.has_permission(auth, id).await?;
        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn update_team_name(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: TeamUpdateTeamNameRequest,
    ) -> Result<Team> {
        let (_user_id, _team) = self.has_permission(auth, id).await?;
        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn invite_member(
        &self,
        id: i64,
        auth: Option<Authorization>,
        TeamInviteMemberRequest { email }: TeamInviteMemberRequest,
    ) -> Result<Team> {
        let (_user_id, _team) = self.has_permission(auth, id).await?;

        let user = match User::query_builder()
            .email_equals(email)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
        {
            Ok(user) => user,
            Err(_) => {
                // FIXME: allow pending user
                return Err(ServiceError::InvalidUser);
            }
        };

        TeamMember::get_repository(self.pool.clone())
            .insert(id, user.id)
            .await?;

        let team = Team::query_builder()
            .id_equals(id)
            .query()
            .map(Team::from)
            .fetch_one(&self.pool)
            .await?;

        Ok(team)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Team> {
        if auth.is_none() {
            return Err(ServiceError::Unauthorized);
        }

        let res = self.repo.delete(id).await?;

        Ok(res)
    }
}

impl TeamController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = User::get_repository(pool.clone());
        let team = Team::get_repository(pool.clone());

        Self {
            user: repo,
            pool,
            repo: team,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_team_by_id).post(Self::act_team_by_id))
            .with_state(self.clone())
            .route("/", post(Self::act_team).get(Self::get_team))
            .with_state(self.clone()))
    }

    pub async fn act_team(
        State(ctrl): State<TeamController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<TeamAction>,
    ) -> Result<Json<Team>> {
        tracing::debug!("act_team {:?}", body);
        match body {
            TeamAction::Create(param) => {
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_team_by_id(
        State(ctrl): State<TeamController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(TeamPath { id }): Path<TeamPath>,
        Json(body): Json<TeamByIdAction>,
    ) -> Result<Json<Team>> {
        tracing::debug!("act_team_by_id {:?} {:?}", id, body);
        match body {
            TeamByIdAction::UpdateProfileImage(param) => {
                let res = ctrl.update_profile_image(id, auth, param).await?;
                Ok(Json(res))
            }
            TeamByIdAction::UpdateTeamName(param) => {
                let res = ctrl.update_team_name(id, auth, param).await?;
                Ok(Json(res))
            }
            TeamByIdAction::InviteMember(param) => {
                let res = ctrl.invite_member(id, auth, param).await?;
                Ok(Json(res))
            }
            TeamByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_team_by_id(
        State(ctrl): State<TeamController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(TeamPath { id }): Path<TeamPath>,
    ) -> Result<Json<Team>> {
        tracing::debug!("get_team {:?}", id);

        Ok(Json(
            Team::query_builder()
                .id_equals(id)
                .query()
                .map(Team::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_team(
        State(ctrl): State<TeamController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<TeamParam>,
    ) -> Result<Json<TeamGetResponse>> {
        tracing::debug!("list_team {:?}", q);

        match q {
            TeamParam::Query(param) => {
                Ok(Json(TeamGetResponse::Query(ctrl.query(auth, param).await?)))
            } // TeamParam::Read(param)
              //     if param.action == Some(TeamReadActionType::ActionType) =>
              // {
              //     let res = ctrl.run_read_action(auth, param).await?;
              //     Ok(Json(TeamGetResponse::Read(res)))
              // }
        }
    }
}
