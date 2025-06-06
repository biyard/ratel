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
            return Err(Error::Unauthorized);
        }

        Ok((user_id, team))
    }

    async fn create(
        &self,
        auth: Option<Authorization>,
        TeamCreateRequest {
            profile_url,
            username,
            nickname,
            html_contents,
        }: TeamCreateRequest,
    ) -> Result<Team> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        let mut tx = self.pool.begin().await?;

        let team = self
            .user
            .insert_with_tx(
                &mut *tx,
                nickname,
                username.clone(),
                username.clone(),
                profile_url,
                false,
                false,
                UserType::Team,
                Some(user_id),
                username,
                html_contents,
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to create team: {:?}", e);
                Error::DuplicatedTeamName
            })?
            .ok_or(Error::DuplicatedTeamName)?;

        TeamMember::get_repository(self.pool.clone())
            .insert_with_tx(&mut *tx, team.id, user_id)
            .await?;

        tx.commit().await?;

        Ok(team.into())
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
                return Err(Error::InvalidUser);
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
            return Err(Error::Unauthorized);
        }

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    async fn get_team_by_username(
        &self,
        _auth: Option<Authorization>,
        TeamReadAction { username, .. }: TeamReadAction,
    ) -> Result<Team> {
        Team::query_builder()
            .username_equals(username.ok_or(Error::InvalidTeamname)?)
            .query()
            .map(Team::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| Error::NotFound)
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
            }
            TeamParam::Read(param) if param.action == Some(TeamReadActionType::GetByUsername) => {
                let res = ctrl.get_team_by_username(auth, param).await?;
                Ok(Json(TeamGetResponse::Read(res)))
            }
            _ => Err(Error::BadRequest),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup, setup_test_user};

    #[tokio::test]
    async fn test_create_team() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();

        let cli = Team::get_client(&endpoint);
        let profile_url = format!("https://test.com/team-{}", now);
        let username = format!("create-team-{}", now);

        let res = cli
            .create(
                username.clone(),
                profile_url.clone(),
                username.clone(),
                "".to_string(),
            )
            .await
            .expect("failed to create team");

        assert_eq!(res.profile_url, profile_url);
        assert_eq!(res.username, username);
        assert_eq!(res.parent_id, user.id);
    }

    #[tokio::test]
    async fn test_invite_team_member() {
        let TestContext {
            user,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();

        let cli = Team::get_client(&endpoint);
        let profile_url = format!("https://test.com/team-{}", now);
        let username = format!("invite-team-{}", now);

        let res = cli
            .create(
                username.clone(),
                profile_url.clone(),
                username.clone(),
                "".to_string(),
            )
            .await
            .expect("failed to create team");

        let id = uuid::Uuid::new_v4().to_string();
        let new_user = setup_test_user(&id, &pool).await.unwrap();
        tracing::debug!("new_user: {:?}", new_user);

        let res = cli
            .invite_member(res.id, new_user.email.clone())
            .await
            .expect("failed to invite member");

        assert_eq!(res.profile_url, profile_url);
        assert_eq!(res.username, username);
        assert_eq!(res.parent_id, user.id);
        assert_eq!(res.members.len(), 2);
        assert_eq!(res.members[0].id, user.id);
        assert_eq!(res.members[0].email, user.email);
        assert_eq!(res.members[1].id, new_user.id);
        assert_eq!(res.members[1].email, new_user.email);

        let id = uuid::Uuid::new_v4().to_string();
        let new_user2 = setup_test_user(&id, &pool).await.unwrap();
        tracing::debug!("new_user: {:?}", new_user2);

        let res = cli
            .invite_member(res.id, new_user2.email.clone())
            .await
            .expect("failed to invite member");

        assert_eq!(res.profile_url, profile_url);
        assert_eq!(res.username, username);
        assert_eq!(res.parent_id, user.id);
        assert_eq!(res.members.len(), 3);
        assert_eq!(res.members[0].id, user.id);
        assert_eq!(res.members[0].email, user.email);
        assert_eq!(res.members[1].id, new_user.id);
        assert_eq!(res.members[1].email, new_user.email);
        assert_eq!(res.members[2].id, new_user2.id);
        assert_eq!(res.members[2].email, new_user2.email);
    }

    #[tokio::test]
    async fn test_update_team_name() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();

        let cli = Team::get_client(&endpoint);
        let profile_url = format!("https://test.com/team-{}", now);
        let username = format!("update-team-{}", now);

        let res = cli
            .create(
                username.clone(),
                profile_url.clone(),
                username.clone(),
                "".to_string(),
            )
            .await
            .expect("failed to create team");

        let new_username = format!("update-team-name-{}", now);
        let res = cli
            .update_team_name(res.id, new_username.clone())
            .await
            .expect("failed to update team name");

        assert_eq!(res.profile_url, profile_url);
        assert_eq!(res.username, new_username);
        assert_eq!(res.parent_id, user.id);
    }

    #[tokio::test]
    async fn test_update_team_profile_image() {
        let TestContext {
            user,
            now,
            endpoint,
            ..
        } = setup().await.unwrap();

        let cli = Team::get_client(&endpoint);
        let profile_url = format!("https://test.com/team-{}", now);
        let username = format!("update-team-{}", now);

        let res = cli
            .create(
                username.clone(),
                profile_url.clone(),
                username.clone(),
                "".to_string(),
            )
            .await
            .expect("failed to create team");

        let new_profile_url = format!("https://test.com/team-profile-{}", now);
        let res = cli
            .update_profile_image(res.id, new_profile_url.clone())
            .await
            .expect("failed to update team profile image");

        assert_eq!(res.profile_url, new_profile_url);
        assert_eq!(res.username, username);
        assert_eq!(res.parent_id, user.id);
    }
}
